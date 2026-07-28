# agents-plugin-tests

Tests for this plugin's agent roster (`../src/agents`, `../src/commands`,
`../src/templates`). Ported from the orca repo, where they lived before the
roster moved into this plugin. orca core now keeps only the agent *primitives*
(the registration seam); the roster and its tests live here.

## Suites

- **structural** (no network): frontmatter validity, command/template presence,
  cross-references, and drift between the roster and the behavioral fixture.
  Runs in CI.
- **behavioral** (needs `ANTHROPIC_API_KEY`): feeds `fixtures/agents-summary.txt`
  as routing context to a model and validates routing decisions against
  `test-cases.json`. Skips without a key.
- **local** (needs a local OpenAI-compatible runtime): the same cases against
  LM Studio / Ollama. Skips if unreachable.

## Usage

```sh
npm install
npm run gen:summary     # regenerate fixtures/agents-summary.txt from ../src/agents
npm test                # structural only (CI default)
npm run test:behavioral # requires ANTHROPIC_API_KEY
npm run test:local      # requires LOCAL_LLM_URL (default LM Studio :1234)
```

`fixtures/agents-summary.txt` is generated from the roster — never hand-edit it.
`npm run check:summary` fails if it has drifted from `../src/agents`.
