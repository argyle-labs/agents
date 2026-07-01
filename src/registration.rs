//! The `domain = "agents"` AgentProvider backend.
//!
//! orca's plugin-loader reads [`backends_json`] at load time, registers a
//! `domain = "agents"` backend, and thereafter drives it by calling
//! `"{invoke_prefix}.{op}"` across the FFI boundary for each composition op
//! (`agents` / `hooks` / `skills` / `commands` / `prompt_fragments`).
//! [`backend_dispatch`] answers those calls with the embedded roster; the
//! toolkit's hybrid `invoke` routes everything else to the `agent.` tool surface.

use plugin_toolkit::serde_json;

/// The op-call prefix the loader drives this backend through. Must be unique
/// against the `agent.` tool surface — the `__backend` infix guarantees no
/// collision with a real tool verb.
const BACKEND_PREFIX: &str = "agent.__backend";

/// Backend descriptor(s) this plugin advertises. One `domain = "agents"`
/// backend whose `invoke_prefix` routes composition ops back to
/// [`backend_dispatch`]. `..Default::default()` keeps the literal forward
/// compatible with new `BackendDef` axes.
pub fn backends_json() -> String {
    let def = plugin_toolkit::abi::BackendDef {
        domain: "agents".to_string(),
        name: "orca-embedded-roster".to_string(),
        invoke_prefix: BACKEND_PREFIX.to_string(),
        ..Default::default()
    };
    serde_json::to_string(&[def]).unwrap_or_else(|_| "[]".to_string())
}

/// Handle the loader's `agent.__backend.<op>` composition calls. Returns
/// `Some(Ok(json_array))` for a known op, `Some(Err(..))` for an unknown op
/// under our prefix, and `None` for anything else (so the toolkit falls through
/// to the `agent.` tool surface). A JSON encode failure degrades to an empty
/// array rather than taking down `orca install`.
pub fn backend_dispatch(name: &str, _args: &str) -> Option<Result<String, String>> {
    let op = name.strip_prefix(BACKEND_PREFIX)?.strip_prefix('.')?;
    let json = match op {
        "agents" => encode(crate::embedded::base_roster_agents()),
        "commands" => encode(crate::commands::base_roster_commands()),
        // No hooks/skills/prompt-fragments in the base roster yet.
        "hooks" | "skills" | "prompt_fragments" => "[]".to_string(),
        other => return Some(Err(format!("unknown agents backend op: {other}"))),
    };
    Some(Ok(json))
}

fn encode<T: plugin_toolkit::serde::Serialize>(items: Vec<T>) -> String {
    serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
}
