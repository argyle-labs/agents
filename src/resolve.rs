//! Agent prompt resolution for the `agent.get` tool.
//!
//! Search order:
//!   1. `$ORCA_AGENTS_DIR` (a filesystem override dir, for dev hot-reload)
//!   2. Embedded baseline (compiled into the cdylib)
//!
//! Profile-aware resolution (per-user override dirs) is an orca-core concern
//! resolved through the composition registry, not the plugin: the plugin ships
//! the baseline roster and an optional env override, nothing more.

use std::path::PathBuf;

/// Load an agent prompt by name, honoring the `$ORCA_AGENTS_DIR` override then
/// the embedded baseline. Frontmatter is stripped from the returned body.
pub fn load_agent_prompt(name: &str) -> Option<String> {
    let override_dir = std::env::var_os("ORCA_AGENTS_DIR").map(PathBuf::from);
    let dirs: Vec<&std::path::Path> = override_dir.iter().map(|p| p.as_path()).collect();
    crate::embedded::load_agent_prompt_from_dirs(name, &dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_when_no_override() {
        let first = crate::embedded::list_embedded_agents()
            .into_iter()
            .next()
            .unwrap()
            .0;
        assert!(load_agent_prompt(&first).is_some());
    }

    #[test]
    fn unknown_agent_returns_none() {
        assert!(load_agent_prompt("zzz_nonexistent_agent_xyz").is_none());
    }
}
