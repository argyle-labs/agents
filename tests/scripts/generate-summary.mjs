/**
 * Generate `fixtures/agents-summary.txt` from the plugin's real roster
 * (`../src/agents/*.md`). The behavioral/local suites feed this summary to a
 * model as the routing context, so generating it from source guarantees the
 * fixture never drifts from the shipped agents (the orca copy had gone stale —
 * it named `badger`/`service-*`/`app-*` agents that no longer exist and omitted
 * ~18 that do).
 *
 * Usage:
 *   node scripts/generate-summary.mjs          # write fixtures/agents-summary.txt
 *   node scripts/generate-summary.mjs --check   # exit 1 if the file is stale
 */
import { readFileSync, writeFileSync, readdirSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const HERE = dirname(fileURLToPath(import.meta.url))
const AGENTS_DIR = join(HERE, '..', '..', 'src', 'agents')
const OUT = join(HERE, '..', 'fixtures', 'agents-summary.txt')

/** Parse the `name` and a one-line `role` (first sentence of description). */
function parseAgent(content) {
  if (!content.startsWith('---')) return null
  const end = content.indexOf('---', 3)
  if (end < 0) return null
  const fm = content.slice(3, end)
  const name = fm.match(/^name:\s*(.+)$/m)?.[1]?.trim()
  const desc = fm.match(/^description:\s*(.+)$/m)?.[1]?.trim()
  if (!name || !desc) return null
  // First sentence, trimmed to a compact role line.
  const role = desc.split(/(?<=[.!?])\s/)[0].replace(/\s+/g, ' ').trim()
  return { name, role }
}

function build() {
  const files = readdirSync(AGENTS_DIR)
    .filter(f => f.endsWith('.md'))
    .sort()
  const rows = []
  for (const f of files) {
    const agent = parseAgent(readFileSync(join(AGENTS_DIR, f), 'utf-8'))
    if (agent) rows.push(agent)
  }
  const header =
    'You are Brain, an AI orchestrator. You route tasks to specialist agents. ' +
    'Here are the available agents:\n\n| Agent | Role |\n|-------|------|\n'
  const body = rows.map(a => `| ${a.name} | ${a.role} |`).join('\n')
  return `${header}${body}\n`
}

const generated = build()

if (process.argv.includes('--check')) {
  let current = ''
  try {
    current = readFileSync(OUT, 'utf-8')
  } catch {
    /* missing → stale */
  }
  if (current !== generated) {
    console.error(
      'agents-summary.txt is stale — run `npm run gen:summary` and commit the result.',
    )
    process.exit(1)
  }
  console.log('agents-summary.txt is up to date.')
} else {
  writeFileSync(OUT, generated)
  console.log(`Wrote ${OUT}`)
}
