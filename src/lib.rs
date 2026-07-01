//! The orca agents plugin.
//!
//! A **hybrid** tool plugin (see [`abi_export`]):
//!
//! - **Tool surface** — `agent.list` / `agent.get`, the `#[orca_tool]`s in
//!   [`agents`]. (The tool domain is `agent`, singular; the crate/plugin is
//!   `agents`.)
//! - **AgentProvider backend** — a `domain = "agents"` backend ([`registration`])
//!   that feeds orca's composition registry (`contract::agents`) with the
//!   embedded base roster ([`embedded`]) and slash-commands ([`commands`]). This
//!   is how `orca install` and the internal chat roster obtain agents; orca core
//!   carries no embedded fallback.
//!
//! The `.md` prompts under `src/agents/`, `src/commands/`, and `src/templates/`
//! are embedded at build time by `build.rs`.

pub mod abi_export;
pub mod agents;
pub mod commands;
pub mod embedded;
pub mod registration;
pub mod resolve;
