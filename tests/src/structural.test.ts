/**
 * Structural tests — no LLM required. Validate the agents plugin's roster
 * source-of-truth in-repo: `../src/agents`, `../src/commands`, `../src/templates`
 * (the .md files `build.rs` embeds into the plugin binary).
 *
 * Ported from the orca repo, where it validated a legacy `~/brain/ai/claude`
 * vault that no longer exists. The brain-shared-docs, `example-*` skill, and
 * per-user `MEMORY.md` checks were dropped — their targets are not part of this
 * plugin. What remains is roster integrity: frontmatter, command/template
 * presence, cross-references, and drift between the roster and the behavioral
 * fixture.
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, existsSync, readdirSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const HERE = dirname(fileURLToPath(import.meta.url))
const SRC = join(HERE, '..', '..', 'src')
const AGENTS = join(SRC, 'agents')
const COMMANDS = join(SRC, 'commands')
const TEMPLATES = join(SRC, 'templates')
const FIXTURES = join(HERE, '..', 'fixtures')

const read = (p: string): string => readFileSync(p, 'utf-8')
const mdFiles = (dir: string): string[] => readdirSync(dir).filter(f => f.endsWith('.md'))

/** Slice the `---`-delimited YAML frontmatter, or null if absent. */
function frontmatter(content: string): string | null {
  if (!content.startsWith('---')) return null
  const end = content.indexOf('---', 3)
  return end > 3 ? content.slice(3, end) : null
}

// ---------------------------------------------------------------------------
// Agent frontmatter
// ---------------------------------------------------------------------------
describe('agent frontmatter', () => {
  const agentFiles = mdFiles(AGENTS)

  it('roster is non-empty', () => {
    expect(agentFiles.length, `no agent .md files in ${AGENTS}`).toBeGreaterThan(0)
  })

  for (const file of agentFiles) {
    it(`${file} has valid frontmatter with name, description, tools`, () => {
      const fm = frontmatter(read(join(AGENTS, file)))
      expect(fm, `${file}: missing --- frontmatter block`).not.toBeNull()
      expect(fm!, `${file}: missing "name:" field`).toMatch(/^name:\s*.+/m)
      expect(fm!, `${file}: missing "description:" field`).toMatch(/^description:\s*.+/m)
      expect(fm!, `${file}: missing "tools:" field`).toMatch(/^tools:\s*.+/m)
    })
  }
})

// ---------------------------------------------------------------------------
// Commands + templates present and non-empty
// ---------------------------------------------------------------------------
describe('commands and templates', () => {
  it('ships at least one command', () => {
    expect(mdFiles(COMMANDS).length).toBeGreaterThan(0)
  })

  for (const file of mdFiles(COMMANDS)) {
    it(`command ${file} is non-empty`, () => {
      expect(read(join(COMMANDS, file)).trim().length, `${file} is empty`).toBeGreaterThan(0)
    })
  }

  for (const file of mdFiles(TEMPLATES)) {
    it(`template ${file} is non-empty`, () => {
      expect(read(join(TEMPLATES, file)).trim().length, `${file} is empty`).toBeGreaterThan(0)
    })
  }
})

// ---------------------------------------------------------------------------
// Cross-references: agents that reference a `/command` point to a real one
// ---------------------------------------------------------------------------
describe('cross-reference resolution', () => {
  it('agents referencing a /command point to an existing command file', () => {
    const commandNames = mdFiles(COMMANDS).map(f => f.replace(/\.md$/, ''))
    const broken: string[] = []

    for (const file of mdFiles(AGENTS)) {
      const content = read(join(AGENTS, file))
      for (const cmd of commandNames) {
        // A slash-reference to a command whose file is (impossibly) absent.
        if (content.includes(`/${cmd}`) && !existsSync(join(COMMANDS, `${cmd}.md`))) {
          broken.push(`${file} → /${cmd}`)
        }
      }
    }

    expect(broken, `Broken command references:\n  ${broken.join('\n  ')}`).toHaveLength(0)
  })
})

// ---------------------------------------------------------------------------
// Behavioral fixture ↔ roster coherence
// ---------------------------------------------------------------------------
describe('behavioral test fixtures', () => {
  const requiredFixtures = [
    'agents-summary.txt',
    'claude-md-rules.txt',
    'sample-memory.txt',
    'sample-stale-memory.txt',
    'long-context-padding.txt',
    'sample-agent-def.txt',
  ]

  for (const fixture of requiredFixtures) {
    it(`fixture ${fixture} exists`, () => {
      expect(existsSync(join(FIXTURES, fixture)), `Fixture not found: ${fixture}`).toBe(true)
    })
  }

  it('agents-summary.txt only names agents that exist in the roster', () => {
    const roster = new Set(mdFiles(AGENTS).map(f => f.replace(/\.md$/, '')))
    const summary = read(join(FIXTURES, 'agents-summary.txt'))
    // Table rows: `| name | role |`
    const named = [...summary.matchAll(/^\|\s*([a-z][\w-]*)\s*\|/gm)]
      .map(m => m[1])
      .filter(n => n !== 'Agent') // skip the header row
    const unknown = named.filter(n => !roster.has(n))
    expect(
      unknown,
      `agents-summary.txt names agents not in the roster (stale — run npm run gen:summary): ${unknown.join(', ')}`,
    ).toHaveLength(0)
  })
})
