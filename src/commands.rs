//! Embedded slash-command roster. Sister to [`crate::embedded`] (agents) — the
//! `.md` files under `src/commands/` are embedded at build time so the
//! `domain = "agents"` backend can hand them to `orca install`, which
//! materializes `~/.claude/commands/<name>.md`.

use plugin_toolkit::contract::agents::CommandDef;

include!(concat!(env!("OUT_DIR"), "/embedded_commands.rs"));

/// The compiled-in slash commands as `CommandDef`s (frontmatter intact in
/// `body`, written verbatim). This is the payload the `domain = "agents"`
/// backend returns for the `commands` op.
pub fn base_roster_commands() -> Vec<CommandDef> {
    embedded_command_names()
        .iter()
        .filter_map(|name| {
            let body = embedded_command(name)?;
            Some(CommandDef {
                name: name.to_string(),
                body: body.to_string(),
                origin: "embedded".to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_roster_commands_are_well_formed() {
        for cmd in base_roster_commands() {
            assert!(!cmd.name.is_empty(), "command name must not be empty");
            assert!(!cmd.name.contains(' '), "whitespace in command: {}", cmd.name);
            assert_eq!(cmd.origin, "embedded");
        }
    }

    #[test]
    fn base_roster_commands_match_embedded_names() {
        assert_eq!(base_roster_commands().len(), embedded_command_names().len());
    }
}
