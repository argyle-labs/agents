//! Dynamic (subprocess) entrypoint for the agents roster registrant.
//!
//! The toolkit's `serve_tool_plugin!` emits `fn main`, connecting the
//! orca-provided socket (`$ORCA_PLUGIN_SOCKET`), sending `Hello`, major-checking
//! `Welcome`, and serving `Invoke → dispatch → Result` until `Shutdown`. This is
//! the dynamic replacement for the retired cdylib export — the plugin is a
//! `[[bin]]`, owns no runtime, and reaches orca only through the socket (exactly
//! like the converted arr / dockge / ntfy subprocess plugins).
//!
//! This plugin's sole job is to register orca's base agent roster. It advertises
//! a `domain = "agents"` backend ([`agents::registration::backends_json`]) so
//! orca's loader drives it after the handshake; the first such call — with the
//! `agents.register` capability sink live — pushes the whole embedded roster
//! into core via `plugin_toolkit::agents::register` (see
//! [`agents::registration::backend_dispatch`]). orca core carries no embedded
//! roster, so without this plugin loaded orca has no agents.

plugin_toolkit::serve_tool_plugin! {
    name: "agents",
    target_compat: "",
    backends: agents::registration::backends_json(),
    backend_dispatch: agents::registration::backend_dispatch,
}
