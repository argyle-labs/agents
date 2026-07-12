//! Minimal wire mirrors of core's roster def types.
//!
//! The `agents.register` capability carries each roster kind as a JSON-array
//! string; orca core parses those arrays back into its own
//! `agents::registry::{AgentDef, CommandDef}` (see the `agents.register` host
//! handler). Those core types live in the daemon-only `agents` crate, which a
//! thin subprocess plugin does not — and must not — link. So this crate defines
//! its own serialize-only mirrors with the identical field shape (`name`,
//! `body`, `origin`); the JSON is byte-for-byte what core expects.

use plugin_toolkit::serde::Serialize;

/// A subagent contribution — mirrors core's `agents::registry::AgentDef`.
/// `body` is the full agent `.md` (frontmatter + prompt), written verbatim to
/// `~/.claude/agents/<name>.md` by `orca install`.
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "plugin_toolkit::serde")]
pub struct AgentDef {
    /// kebab-case agent name — the file stem and picker id.
    pub name: String,
    /// Full markdown: frontmatter + prompt body, ready to write verbatim.
    pub body: String,
    /// Contributing provider name — for precedence reporting.
    pub origin: String,
}

/// A slash-command contribution — mirrors core's `agents::registry::CommandDef`.
/// Materialized to `~/.claude/commands/<name>.md`.
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "plugin_toolkit::serde")]
pub struct CommandDef {
    /// Command name (invoked as `/<name>`).
    pub name: String,
    /// Full markdown body (frontmatter + prompt), written verbatim.
    pub body: String,
    pub origin: String,
}
