## Core work discipline

- Work in batches: make a meaningful set of related changes, then build, test, lint, and validate once.
- If validation fails, fix and validate again until it passes.
- Never build or test after every individual change.
- Fully validate (build, test, lint) before any push.

## Fleet access

- Fleet actions go through orca MCP tools (`mcp__orca__*`; load via ToolSearch).
- Never ssh/scp/rsync to fleet hosts; a hook blocks it.
- If no orca tool covers the need, report it as a capability gap (logged to `orca-capability-gaps.md`) for the orchestrator to build. Do not work around it.
