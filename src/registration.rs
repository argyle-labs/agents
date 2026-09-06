//! The embedded base agent roster this repo contributes to orca core.
//!
//! orca core carries **no** embedded roster (removed in orca #61), so the
//! wolf/otter/… agents and the slash-commands live here and are pushed into
//! core's `agents` domain over the `agents.register` capability (landed in orca
//! #58). The push mechanics — advertise a `domain = "agents"` trigger backend
//! and register once the first time orca drives it, while the capability sink is
//! live — are now the `Plugin` builder's `.agents()` facet, which ANY plugin can
//! use to contribute its own agents. This module just builds the baseline
//! [`AgentRegistration`]; `main.rs` hands it to `.agents(...)`.

use plugin_toolkit::abi::AgentRegistration;
use plugin_toolkit::serde_json;

/// Provider name the roster registers under in core's `agents` domain.
/// Re-registering the same name replaces the provider in place.
pub const PROVIDER_NAME: &str = "argyle-labs/agents";

/// Serialize the embedded roster into an [`AgentRegistration`]: each field is a
/// JSON-array string of the matching roster def (see [`crate::def`]), which core
/// parses back into its own `AgentDef` / `CommandDef`. The base roster
/// contributes agents + slash-commands; hooks, skills, and prompt fragments are
/// empty (`[]`) for now.
pub fn registration() -> serde_json::Result<AgentRegistration> {
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
    fn registration_serializes_every_embedded_agent_and_command() {
        let reg = registration().expect("build registration");
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
}
