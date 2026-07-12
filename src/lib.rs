//! The canonical base-agent-roster registrant for orca.
//!
//! orca core carries **no** embedded agent roster (removed in orca #61). This
//! crate owns the roster content — the wolf/otter/… agent prompts and the
//! slash-commands — and registers it into core's `agents` domain via the
//! `plugin_toolkit::agents` seam (the `agents.register` capability, landed in
//! orca #58). Without this plugin loaded, orca has no agents to route to or
//! materialize.
//!
//! It ships as a **dynamic subprocess plugin**: orca spawns the `[[bin]]`
//! (`src/main.rs`) and speaks the UDS wire protocol to it. There is no dlopened
//! cdylib anymore.
//!
//! - [`embedded`] / [`commands`] — the compiled-in roster. The `.md` prompts
//!   under `src/agents/` and `src/commands/` are embedded at build time by
//!   `build.rs` and surfaced as `AgentDef` / `CommandDef`s.
//! - [`registration`] — advertises the `domain = "agents"` backend and, on the
//!   first orca-driven call, pushes the whole roster to core through the seam.

pub mod commands;
pub mod def;
pub mod embedded;
pub mod registration;
