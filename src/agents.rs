//! Agent tools — `agent.list` (names + descriptions) and `agent.get` (full
//! prompt for a named agent). The tool domain is `agent` (singular); the crate
//! and plugin are `agents`.

use plugin_toolkit::anyhow;
use plugin_toolkit::clap;
use plugin_toolkit::contract;
use plugin_toolkit::prelude::{orca_tool, plugin_struct};

// ── Typed entities ──────────────────────────────────────────────────────────

#[plugin_struct]
pub struct AgentEntry {
    pub name: String,
    pub description: String,
}

// ── Args / Outputs ──────────────────────────────────────────────────────────

#[plugin_struct(args)]
pub struct ListAgentsArgs {}

#[plugin_struct]
pub struct ListAgentsOutput {
    pub agents: Vec<AgentEntry>,
}

#[plugin_struct(args)]
pub struct GetAgentArgs {
    pub name: String,
}

#[plugin_struct]
pub struct GetAgentOutput {
    pub name: String,
    pub prompt: String,
}

// ── Native tool bodies ──────────────────────────────────────────────────────

/// List all available orca agents with their names and descriptions.
#[orca_tool(domain = "agent", verb = "list")]
async fn list_agents(
    _args: ListAgentsArgs,
    _ctx: &contract::ToolCtx,
) -> anyhow::Result<ListAgentsOutput> {
    let agents = crate::embedded::list_embedded_agents()
        .into_iter()
        .map(|(name, description)| AgentEntry { name, description })
        .collect();
    Ok(ListAgentsOutput { agents })
}

/// Return the full system prompt for a named orca agent.
#[orca_tool(domain = "agent", verb = "get")]
async fn get_agent(
    args: GetAgentArgs,
    _ctx: &contract::ToolCtx,
) -> anyhow::Result<GetAgentOutput> {
    let prompt = crate::resolve::load_agent_prompt(&args.name)
        .ok_or_else(|| anyhow::anyhow!("agent not found: {}", args.name))?;
    Ok(GetAgentOutput {
        name: args.name,
        prompt,
    })
}
