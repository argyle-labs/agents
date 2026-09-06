//! Subprocess entrypoint for the agents roster registrant.
//!
//! This plugin's sole job is to register orca's **baseline** agent roster. It
//! hands the embedded roster to the `Plugin` builder's `.agents()` facet, which
//! advertises the `domain = "agents"` trigger backend and pushes the roster into
//! core over the `agents.register` capability the first time orca drives it. orca
//! core carries no embedded roster, so without this plugin loaded orca has no
//! agents — but any OTHER plugin can contribute its own agents the same way.

plugin_toolkit::instrument::bootstrap!();

use plugin_toolkit::plugin::Plugin;

fn main() -> plugin_toolkit::anyhow::Result<()> {
    Plugin::named("agents")
        .version(env!("CARGO_PKG_VERSION"))
        .agents(agents::registration::registration()?)
        .serve()
}
