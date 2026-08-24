---
name: skill-authoring
description: Use when creating a new shared skill or re-authoring an existing one in this repository — the form and disclosure law skills are written under. Covers the two registers (law that binds, disclosure that guides), how small to write, description-led triggering, the authoring defects to refuse, and the single-home and validation mechanics. Not for using skills — only for writing or changing them. Application is forward-only; existing skills converge when touched for their own reasons, never by sweep.
node: skill-authoring
class: skill
edges:
  - type: cites
    target: 5020-skill-validation
    provenance: declared
  - type: cites
    target: adopted-protocols
    provenance: declared
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-5000
    provenance: declared
metadata:
  short-description: How skills are written — disclose, don't puppeteer
---

# Skill Authoring

Skills are disclosure surfaces, not scripts. A skill exists to give an
acting agent the law of a territory and a thread to pull — not to
replace the agent's judgment with the author's. This skill owns the
authoring form and the shared-surface rules below.

## The Surface

- `.agents/skills/<id>/` is the one shared-skill surface,
  repo-discovered; never mirror into `$CODEX_HOME/skills` or
  home-managed skill trees, and never treat machine-local config as
  source of truth.
- Personal, experimental, machine-local, and upstream skills stay
  out; a deprecated skill moves to
  `.codex/references/deprecated/skills/` so discovery cannot find it.
- Reviewer-only and research-only skills grant no mutation authority.
- A delegating skill names its local role and docket, then points to
  `$agent-run`; it never restates the handoff, Git boundary, Passback, or
  recovery procedure that node owns.
- An automation-only skill says so in its description and runs only
  by explicit human invocation or an automation runner.
- A skill's repo or external tool dependencies ride
  `.codex/tools/manifest.yaml` in the same mutation.

## The Two Registers

Every sentence in a skill is one of two things, and the two must read
differently at a glance:

- **Law** — named invariants and typed refusals. Binds absolutely.
  Exact procedure belongs here only where determinism is genuinely
  required: absolutes, gate mechanics, recovery playbooks, refusal
  tables.
- **Disclosure** — the law of the territory, the distinctions that
  matter in it, and the addresses of its deeper documents. Guides
  judgment. Uptake and reasoned decline are both conforming outcomes.

Scripting the judgment calls *between* invariants is the standing
authoring defect — conformity-push. It converts the agent into an
expensive interpreter of steps and discards the judgment the agent was
deployed for. Where exact execution is truly wanted, that is law (state
it as an invariant with its refusal) or it is code behind a seam — not
a choreographed agent.

## The Form

- Default small: a quick read giving the agent enough to test relevance
  against its live context and pull deeper by reference. Progressive
  disclosure over completeness — point to documents; do not inline
  them.
- **[INVARIANT]** Token cost is first-class: every sentence prices the window
  it will ride; compress wherever meaning survives, judged against the
  intended reader population and with density calibrated to its floor, never
  the author's ceiling. The authoring gate rejects with the same strength it
  admits.
- The frontmatter description, not the name, carries triggering. Write
  it in the reader's situation-vocabulary: when to load, when not to.
- **[INVARIANT]** A skill that assigns its reader a working seat in an
  opening "You are the ..." sentence names, in or adjacent to that sentence,
  the constitutional role the reader embodies by pointing to
  `architecture/development-lifecycle/AGENTS.md` ("Actor Constitution"); it
  never restates the role. Negative-identity walls remain good practice
  beside this positive embodiment, and the requirement applies forward-only:
  existing skills converge when authored or re-authored for their own
  reasons, never by sweep.
- Map the territory: what this skill owns, its invariants, its
  refusals, where its deeper documents live.
- One home per truth: never restate what another surface owns; cite the
  owner. The Documentation Law applies to skills.
- Never name a surface that does not exist or is not yet proven;
  in-flight surfaces live in the risk register until ratified.
- Write named acts in the work-protocol vocabulary (`$git-policy`,
  The Work Protocol Vocabulary). Do not coin synonyms for named acts
  or use a canon word against its sense.

### Doctrine Graph Node Header

- **[INVARIANT]** A doctrine-graph node header lives in the skill's existing
  YAML frontmatter and conforms to
  `architecture/doctrine-graph/schemas/skill-node-header.schema.yaml`: `node`
  is the stable identity, `class` is `skill`, and each `edges` entry carries a
  type owned by `architecture/doctrine-graph/doctrine-graph.md` ("Edge Model"),
  a target, and `provenance: declared`. Existing `name`, `description`, and
  `metadata` harness fields remain; by default `description` is also the node
  summary until a successor surface exists.
- **[INVARIANT]** New and re-authored skills carry the header; existing skills
  converge only when touched for their own reasons, never by sweep. Headerless
  skills remain admitted through the transitional front end owned by
  `architecture/doctrine-graph/compiler.md` ("Source Declarations" and
  "Class Front Ends").
- **[INVARIANT]** In a header-bearing skill, every `$<skill-id>` citation in
  the body has a declared edge, and every declared edge target resolves in the
  repository. The future doctrine compiler owns mechanical token coverage and
  endpoint closure. Until that compiler lands, no standalone checker is an
  active skill gate and no agent may advertise those checks as implemented.

### Naming Acts, Events, And Artifacts

- **[INVARIANT]** `architecture/development-lifecycle/AGENTS.md`
  ("Lifecycle Word Formation") owns the word-formation rule. This section
  applies it to skill authoring and restates no rationale.
- When a skill mints an act word, form its event name by the owner's
  participle rule: a "V-ed event" is a typed occurrence of act V.
- An artifact produced by an act carries its owned artifact name; that name
  need not be a participle. Do not force participle artifact names.
- Before minting any act word, check whether it can form a sensible
  participle event. If it cannot, it does not name an act; name the noun it
  actually is instead.
- Sweep every minted spelling against the reserved names: the work-protocol
  verb canon, `llm_gateway.*` identifiers, adopted protocol names, and
  existing skill ids.

## Authoring Defects — refuse at review

- Step-scripts across judgment territory (conformity-push).
- The seven-league read: so long the thread cannot be found.
- Name-led triggering; a vague or ornamental description.
- Restated truths owned by other surfaces (a straddle).
- Justification prose and derivable facts.
- References to unbuilt or unproven surfaces.
- Synonyms for named protocol acts; canon words against their sense.
- On a plane governed by an adopted protocol (root Protocols; register:
  `architecture/protocols/AGENTS.md`): any sentence that respells, narrows,
  or asserts authority over the adopted grammar. Resolve
  `$adopted-protocols` before Seal on such a skill; a BLOCK refuses the
  change.

## Mechanics

- Shared skills have one home: `.agents/skills/<id>/SKILL.md`. The
  `.claude/skills` surface resolves there by symlink — never maintain
  a second copy.
- Validate changed skill source:
  `cargo run -p assurance-skill-validator -- validate .agents/skills/<id>`.
- `architecture/risk/doctrine-graph/` records the doctrine compiler as
  unbuilt. Do not invoke, retain, or replace a standalone node-header checker;
  schema, token-coverage, and endpoint-closure enforcement enter only with the
  commissioned compiler implementation.
- Skill mutations ride the 5000 lane under `$git-policy` and reach their
  merge branch only through a current-candidate `$5020-skill-validation`
  `PASS` within the default correction envelope owned by `$protocol-5000`.
- Derivation for the disclosure law:
  `runs/1000/1785176498125-0eda-collapse-as-disclosure/`; for the
  vocabulary canon: `runs/1000/1785193617565-14df-verb-canon-sweep/` —
  evidence, cited for reasoning, never as authority.
