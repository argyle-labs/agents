<p align="center">
  <img src="assets/icon-256.png" width="120" alt="agents" />
</p>

# agents

orca's base agent roster — the specialized subagents (wolf, otter, owl, crow, …) and slash-commands that give orca its personality and routing.

The canonical **base-agent-roster registrant** for [orca](https://github.com/argyle-labs/orca).

orca core carries **no embedded roster**: it removed its built-in agents, so this repo is the source of truth for the base roster. Without this plugin loaded, orca has no agents to route to or materialize. This repo owns the roster **content**; core owns the composition machinery.

---

## What it does

A **dynamic (subprocess) orca plugin** whose sole job is to register the base roster. orca spawns the plugin binary and speaks the plugin wire protocol to it over a Unix-domain socket; the plugin then pushes its whole roster into orca core's `agents` domain through the `plugin_toolkit::agents` seam (the `agents.register` capability).

Concretely:

- It advertises a `domain = "agents"` backend so orca's plugin-loader drives it after the handshake.
- On the first such call — with the `agents.register` capability channel live — it builds a `plugin_toolkit::abi::AgentRegistration` (provider name `argyle-labs/agents`) from the embedded roster and calls `plugin_toolkit::agents::register(reg)`.
- orca core parses that registration into its own roster registry and composes it. `orca install` then materializes the agents into `~/.claude/agents/*.md` and the slash-commands into `~/.claude/commands/*.md`, and the internal chat roster routes against them.

The roster currently contributes **agents** and **slash-commands**; hooks, skills, and CLAUDE.md prompt-fragments are registered as empty and reserved for future content.

## With orca

Load the plugin and orca composes its contributions automatically — nothing agent-specific is hardcoded in core. A later-registered provider (e.g. a per-profile override) wins on name collision, so this roster is a baseline you can override, not a ceiling.

Set `$ORCA_AGENTS_DIR` to a directory of `<name>.md` files to override individual agent prompts during development without rebuilding.

## Layout

- `src/main.rs` — the subprocess entrypoint (`serve_tool_plugin!` boot loop).
- `src/registration.rs` — builds the `AgentRegistration` and pushes it via `plugin_toolkit::agents::register`.
- `src/embedded.rs`, `src/commands.rs` — the embedded roster, surfaced as roster defs.
- `src/def.rs` — thin serialize-only mirrors of core's roster def wire shape.
- `src/agents/`, `src/commands/` — the embedded agent + slash-command `.md` prompts (embedded at build time by `build.rs`).
- `src/templates/` — reference agent-authoring templates (not part of the registered roster).
- `assets/` — plugin icon.

## Build

The sole dependency is `plugin-toolkit`, pinned to an orca `main` commit that carries the `agents.register` seam. orca is private, so cargo resolves the git dependency through the git CLI (`.cargo/config.toml` sets `net.git-fetch-with-cli`); CI authenticates the same fetch with a repo token.

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```
