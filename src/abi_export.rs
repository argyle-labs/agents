//! ABI-stable cdylib export for the agents plugin.
//!
//! agents is a **hybrid** plugin: it exposes the `agent.` tool surface AND
//! registers a `domain = "agents"` AgentProvider backend. The toolkit's
//! [`export_tool_plugin!`] hybrid arm generates the metadata fns, the `agent.`
//! manifest filtered from the linked `#[orca_tool]` inventory, and an `invoke`
//! that tries the backend hook first (the `agent.__backend.*` calls the loader
//! makes to drive the AgentProvider) then falls through to tool dispatch.
//!
//! `name: "agent"` (singular) is deliberate — it drives the tool prefix, which
//! must match the `agent.list` / `agent.get` tool names. The crate is `agents`
//! (plural); the backend registers under the loader's `domain = "agents"`.
//!
//! `abi_stable` remains the crate's one direct non-orca dep because
//! `#[export_root_module]` (which the macro invokes) expands to bare
//! `::abi_stable` paths.

plugin_toolkit::export_tool_plugin! {
    name: "agent",
    target_compat: "",
    backends: crate::registration::backends_json(),
    backend_dispatch: crate::registration::backend_dispatch,
}
