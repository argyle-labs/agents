//! Push the embedded base roster into orca core's `agents` domain.
//!
//! This repo is the canonical base-agent-roster registrant: orca core carries
//! **no** embedded roster (removed in orca #61), so the wolf/otter/… agents and
//! the slash-commands live here and are pushed into core over the
//! `agents.register` capability via the [`plugin_toolkit::agents::register`]
//! seam (landed in orca #58).
//!
//! ## How the push is triggered
//!
//! `agents.register` is a capability: it round-trips over the plugin socket, so
//! its sink is only live **while the plugin is servicing an orca `Invoke`**
//! (see `plugin_toolkit::serve` — the cap sink is installed per-invoke). A
//! subprocess plugin therefore cannot push at process start; it pushes the
//! first time orca drives it.
//!
//! To give orca that trigger, the plugin advertises a `domain = "agents"`
//! backend ([`backends_json`]). orca's plugin-loader registers the backend at
//! load and drives it by calling `"{invoke_prefix}.{op}"` across the socket.
//! The first such call lands in [`backend_dispatch`] with the capability sink
//! live, so we build the full [`AgentRegistration`] and call
//! [`plugin_toolkit::agents::register`] once — handing core the whole roster
//! (agents + commands; no hooks/skills/prompt-fragments in the base roster yet)
//! as a `StaticProvider`. Subsequent calls are no-ops.

use std::sync::atomic::{AtomicBool, Ordering};

use plugin_toolkit::abi::AgentRegistration;
use plugin_toolkit::serde_json;

/// Provider name the roster registers under in core's `agents` domain.
/// Re-registering the same name replaces the provider in place.
const PROVIDER_NAME: &str = "argyle-labs/agents";

/// The op-call prefix orca's loader drives this backend through. The `__backend`
/// infix guarantees no collision with any real tool verb.
const BACKEND_PREFIX: &str = "agents.__backend";

/// Ensures the roster is pushed to core exactly once, however many composition
/// ops the loader drives the backend through in a single session.
static REGISTERED: AtomicBool = AtomicBool::new(false);

/// The `domain = "agents"` backend descriptor the plugin advertises in its
/// `Hello`. orca's loader maps `domain = "agents"` to its agents registration
/// arm and drives `"{invoke_prefix}.{op}"` calls into [`backend_dispatch`];
/// that first call is our trigger to push the roster. `..Default::default()`
/// keeps the literal forward-compatible with new `BackendDef` axes.
pub fn backends_json() -> String {
    let def = plugin_toolkit::abi::BackendDef {
        domain: "agents".to_string(),
        name: PROVIDER_NAME.to_string(),
        invoke_prefix: BACKEND_PREFIX.to_string(),
        ..Default::default()
    };
    serde_json::to_string(&[def]).unwrap_or_else(|_| "[]".to_string())
}

/// Handle the loader's `agents.__backend.<op>` calls. On the first call (cap
/// sink live) push the whole roster into core via
/// [`plugin_toolkit::agents::register`]; thereafter the roster is already a
/// registered `StaticProvider`, so we return an empty array (core reads the
/// composition from the pushed provider, not from these per-op replies).
///
/// Returns `Some(Ok(_))` for a call under our prefix, `Some(Err(_))` if the
/// push fails, and `None` for anything else (so the toolkit's hybrid `invoke`
/// falls through to tool dispatch).
pub fn backend_dispatch(
    name: &str,
    _args: serde_json::Value,
) -> Option<Result<serde_json::Value, serde_json::Value>> {
    let _op = name.strip_prefix(BACKEND_PREFIX)?.strip_prefix('.')?;
    if let Err(e) = ensure_registered() {
        return Some(Err(serde_json::Value::String(e)));
    }
    Some(Ok(serde_json::json!([])))
}

/// Push the embedded roster into core exactly once. Idempotent: after the first
/// successful push `REGISTERED` is set and further calls short-circuit.
fn ensure_registered() -> Result<(), String> {
    if REGISTERED.load(Ordering::Acquire) {
        return Ok(());
    }
    let reg = build_registration().map_err(|e| format!("build agents registration: {e}"))?;
    plugin_toolkit::agents::register(reg)
        .map_err(|e| format!("agents.register capability failed: {e}"))?;
    REGISTERED.store(true, Ordering::Release);
    Ok(())
}

/// Serialize the embedded roster into an [`AgentRegistration`]: each field is a
/// JSON-array string of the matching roster def (see [`crate::def`]), which core
/// parses back into its own `AgentDef` / `CommandDef`. The base roster
/// contributes agents + slash-commands; hooks, skills, and prompt fragments are
/// empty (`[]`) for now.
fn build_registration() -> serde_json::Result<AgentRegistration> {
    Ok(AgentRegistration {
        name: PROVIDER_NAME.to_string(),
        agents_json: serde_json::to_string(&crate::embedded::base_roster_agents())?,
        hooks_json: "[]".to_string(),
        skills_json: "[]".to_string(),
        commands_json: serde_json::to_string(&crate::commands::base_roster_commands())?,
        prompt_fragments_json: "[]".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backends_json_declares_the_agents_domain() {
        let json = backends_json();
        assert!(
            json.contains("\"agents\""),
            "backend must declare domain agents"
        );
        assert!(
            json.contains(PROVIDER_NAME),
            "backend must carry the provider name"
        );
        assert!(
            json.contains(BACKEND_PREFIX),
            "backend must carry the invoke prefix"
        );
    }

    #[test]
    fn registration_serializes_every_embedded_agent_and_command() {
        let reg = build_registration().expect("build registration");
        assert_eq!(reg.name, PROVIDER_NAME);

        let agents: serde_json::Value = serde_json::from_str(&reg.agents_json).unwrap();
        assert_eq!(
            agents.as_array().unwrap().len(),
            crate::embedded::base_roster_agents().len(),
            "every embedded agent must be in agents_json"
        );

        let commands: serde_json::Value = serde_json::from_str(&reg.commands_json).unwrap();
        assert_eq!(
            commands.as_array().unwrap().len(),
            crate::commands::base_roster_commands().len(),
            "every embedded command must be in commands_json"
        );

        // Roster carries no hooks/skills/prompt-fragments yet.
        for empty in [
            &reg.hooks_json,
            &reg.skills_json,
            &reg.prompt_fragments_json,
        ] {
            let v: serde_json::Value = serde_json::from_str(empty).unwrap();
            assert!(v.as_array().unwrap().is_empty());
        }
    }

    #[test]
    fn backend_dispatch_ignores_foreign_prefixes() {
        assert!(backend_dispatch("something.else", serde_json::json!({})).is_none());
    }
}
