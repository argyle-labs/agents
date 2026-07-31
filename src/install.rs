//! `orca agents install` — materialize this plugin's roster to `~/.claude/`.
//!
//! The agents repo OWNS agent installation. orca core no longer writes any
//! agent / slash-command artifacts (that responsibility was removed from
//! `orca system install` when it moved here). This tool is the **sole** path —
//! explicit, never automatic — that writes the embedded roster to the operator's
//! Claude Code config dir so its native Agent picker discovers wolf/otter/… .
//!
//! Agents land in `<dir>/agents/<name>.md`, slash-commands in
//! `<dir>/commands/<name>.md`. Overwrite policy is unconditional: re-run any
//! time to refresh. Operators who want to edit a prompt should fork it to a new
//! name (e.g. `wolf-custom.md`), which this never touches.

use plugin_toolkit::prelude::*;
use std::path::PathBuf;

use crate::commands::base_roster_commands;
use crate::embedded::base_roster_agents;

/// Default install root — the operator's Claude Code config dir (`$HOME/.claude`).
fn default_claude_dir() -> String {
    std::env::var("HOME")
        .map(|h| format!("{h}/.claude"))
        .unwrap_or_else(|_| ".claude".to_string())
}

#[orca_struct(args)]
pub struct InstallArgs {
    /// Claude config dir to materialize into (default `$HOME/.claude`). Agents
    /// are written under `<dir>/agents/`, slash-commands under `<dir>/commands/`.
    #[arg(long)]
    #[serde(default)]
    pub dir: Option<String>,
}

#[orca_struct]
pub struct InstallResult {
    /// Absolute dir the roster was materialized under.
    pub dir: String,
    /// Number of agent `.md` files written.
    pub agents: usize,
    /// Number of slash-command `.md` files written.
    pub commands: usize,
    pub notes: Vec<String>,
}

/// Materialize the embedded roster (agents + slash-commands) to the Claude
/// config dir. This is the one and only agent-install entrypoint — nothing
/// materializes agents automatically.
#[orca_tool(
    domain = "agents",
    verb = "install",
    data_mutation = true,
    role = "admin"
)]
pub async fn agents_install(args: InstallArgs, _ctx: &ToolCtx) -> Result<InstallResult> {
    let root = PathBuf::from(args.dir.unwrap_or_else(default_claude_dir));
    let agents_dir = root.join("agents");
    let commands_dir = root.join("commands");
    std::fs::create_dir_all(&agents_dir)
        .map_err(|e| anyhow!("create {}: {e}", agents_dir.display()))?;
    std::fs::create_dir_all(&commands_dir)
        .map_err(|e| anyhow!("create {}: {e}", commands_dir.display()))?;

    let mut agents = 0usize;
    for def in base_roster_agents() {
        let path = agents_dir.join(format!("{}.md", def.name));
        std::fs::write(&path, &def.body).map_err(|e| anyhow!("write {}: {e}", path.display()))?;
        agents += 1;
    }

    let mut commands = 0usize;
    for def in base_roster_commands() {
        let path = commands_dir.join(format!("{}.md", def.name));
        std::fs::write(&path, &def.body).map_err(|e| anyhow!("write {}: {e}", path.display()))?;
        commands += 1;
    }

    Ok(InstallResult {
        dir: root.display().to_string(),
        agents,
        commands,
        notes: vec![format!(
            "materialized {agents} agents + {commands} slash-commands to {}",
            root.display()
        )],
    })
}
