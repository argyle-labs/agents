//! Embedded agent roster.
//!
//! Agent definitions are `.md` files with YAML frontmatter under `src/agents/`,
//! embedded at compile time by `build.rs`. The `agent.*` tool surface reads them
//! (filesystem-override first via `ORCA_AGENTS_DIR`, then embedded); the
//! `domain = "agents"` backend ([`base_roster_agents`]) hands the full roster to
//! orca's composition registry.

// Generated at build time by build.rs — embeds agent .md files into the cdylib.
include!(concat!(env!("OUT_DIR"), "/embedded_agents.rs"));

use crate::def::AgentDef;
use std::path::Path;

/// The compiled-in base roster as `AgentDef`s (frontmatter intact in `body`,
/// written verbatim to `~/.claude/agents/<name>.md` by `orca install`). This is
/// the payload the `domain = "agents"` backend returns for the `agents` op.
pub fn base_roster_agents() -> Vec<AgentDef> {
    embedded_agent_names()
        .iter()
        .filter_map(|name| {
            let raw = embedded_agent(name)?;
            Some(AgentDef {
                name: name.to_string(),
                body: raw.to_string(),
                origin: "embedded".to_string(),
            })
        })
        .collect()
}

/// All embedded agents with their name and description (parsed from frontmatter).
pub fn list_embedded_agents() -> Vec<(String, String)> {
    embedded_agent_names()
        .iter()
        .filter_map(|name| {
            let raw = embedded_agent(name)?;
            let desc = frontmatter_field_from_str(raw, "description").unwrap_or_default();
            Some((name.to_string(), desc))
        })
        .collect()
}

/// Load an agent prompt searching filesystem override dirs in priority order,
/// falling back to the embedded agent. The first directory that contains a
/// readable `<name>.md` wins.
pub fn load_agent_prompt_from_dirs(name: &str, dirs: &[&Path]) -> Option<String> {
    for dir in dirs {
        let path = dir.join(format!("{name}.md"));
        if path.exists()
            && let Ok(raw) = std::fs::read_to_string(&path)
        {
            return Some(strip_frontmatter(&raw));
        }
    }
    embedded_agent(name).map(strip_frontmatter)
}

fn frontmatter_field_from_str(content: &str, field: &str) -> Option<String> {
    let prefix = format!("{field}:");
    content
        .lines()
        .find_map(|l| l.strip_prefix(&prefix).map(|v| v.trim().to_string()))
}

fn strip_frontmatter(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if lines.first().map(|l| l.trim()) == Some("---")
        && let Some(end) = lines[1..].iter().position(|l| l.trim() == "---")
    {
        return lines[end + 2..].join("\n").trim().to_string();
    }
    content.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn list_embedded_agents_is_non_empty() {
        assert!(
            !list_embedded_agents().is_empty(),
            "at least one agent must be embedded at build time"
        );
    }

    #[test]
    fn every_embedded_agent_has_name_and_description() {
        for (name, desc) in list_embedded_agents() {
            assert!(!name.is_empty(), "agent name must not be empty");
            assert!(!desc.is_empty(), "agent '{name}' has empty description");
        }
    }

    #[test]
    fn base_roster_surfaces_every_embedded_agent() {
        let roster = base_roster_agents();
        assert_eq!(roster.len(), embedded_agent_names().len());
        assert!(roster.iter().all(|a| a.origin == "embedded"));
        assert!(roster.iter().all(|a| !a.body.is_empty()));
    }

    #[test]
    fn descriptions_have_no_frontmatter_delimiter() {
        for (name, desc) in list_embedded_agents() {
            assert!(
                !desc.starts_with("---"),
                "agent {name} description still has frontmatter"
            );
        }
    }

    #[test]
    fn load_prompt_prefers_filesystem_over_embedded() {
        let dir = std::env::temp_dir().join(format!("orca_agent_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let content = "---\ndescription: override\n---\nOverride prompt from filesystem!";
        std::fs::write(dir.join("orca.md"), content).unwrap();
        let prompt = load_agent_prompt_from_dirs("orca", &[dir.as_path()]).unwrap();
        assert_eq!(prompt, "Override prompt from filesystem!");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_prompt_falls_back_to_embedded() {
        let first = list_embedded_agents().into_iter().next().unwrap().0;
        let missing = PathBuf::from("/tmp/__orca_no_such_dir__");
        assert!(load_agent_prompt_from_dirs(&first, &[missing.as_path()]).is_some());
    }

    #[test]
    fn strip_frontmatter_removes_yaml_block() {
        let raw = "---\nname: test\ndescription: stuff\n---\nBody content here.";
        assert_eq!(strip_frontmatter(raw), "Body content here.");
    }

    #[test]
    fn strip_frontmatter_passthrough_when_absent() {
        let raw = "Just a plain prompt with no frontmatter.";
        assert_eq!(strip_frontmatter(raw), raw);
    }
}
