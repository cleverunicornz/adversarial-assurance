---
name: state-modeling
description: The modeling rubric — how an application's state protocol is derived from its use and authored as a design package the promise plane can mint from. Use when a human commissions a state protocol for an application ("we need a storage protocol for X"), when a surface leaves the outgoing engine and needs its forward state home authored, when lineage decomposition or vocabulary questions arise, or when a 3000 unit blocks on missing state architecture. A specialization of $promise — the output is promise-plane material for one application, authored on the 6000 lane, consumed by 3010. It derives and authors; it does not implement, gauge, or lock.
metadata:
  short-description: Derive a state protocol from use — the rubric
---

# State Modeling — The Rubric

Load `$use-derived-state` first; the canon at
`architecture/use-derived-state/` governs everything here. This skill is a
specialization of `$promise`: it produces the application's **design
package** — promise-plane material stating that application's state
protocol — and every sentence of it must pass the mint test. Routing is
promise routing: closed knowledge authors directly on a 6000 unit;
empirical unknowns route through 2000 first; contracts consume the landed
package via the 3010 entry gate and never author their own premise.

Load `$search` for the inventory work: the use is read from the
application's actual operations, seams, and evidence — `rg`, `git grep`,
`fd`, bounded reads, `ast-grep` — never assumed from its description.

## Question Zero — Jurisdiction And Direction

Before any structure exists, two gates:

- **Jurisdiction.** Does this domain collapse well — natural write
  authority per lineage, append and revision semantics, rebuildable
  projections, bounded hydratable state, known operations? If it genuinely
  does not (arbitrary ad-hoc relational querying, contention no seam
  decomposes, cross-record invariants resisting choreography), record that
  verdict and stop for the human. "A database earns its place" is a
  legitimate outcome — and any engine naming that follows is the human's
  act under the shelf's law, never this skill's.
- **Direction.** The use inventory must exist before any structure is
  chosen. A record in which a schema, engine, layout, or framework store
  shape appears before the demonstrated-use inventory fails authoring on
  the spot — that is the old geometry's founding move.

## The Ten Questions

The rubric is fixed; the answers are always private to the one
application (the application owns its language outright). Answer all ten
in the design package; an unanswered question is an open decision, and
open decisions stop for the human before authoring completes.

1. **Truth inventory.** What happened-facts does this application own?
   What may happen next, and what must be refused? Verbs come from
   demonstrated practice, not from imagination or another protocol.
2. **Mutation grain.** What is the smallest act that must publish
   atomically? That grain bounds the lineage; batching is spelling, not
   meaning.
3. **Lineage decomposition.** Asked in both directions: what must publish
   atomically together shares a lineage; what need not may part and write
   in parallel under its own fence. One-lineage-unasked builds SQLite on
   S3; sharding past the atomic unit builds a distributed monolith with
   undeclared choreography. Both fail.
4. **Identity and provenance.** How is a lineage named and discovered?
   What identity rides every transition — actor, cause, prior state —
   so history is forensically walkable?
5. **Durability unit.** What is acknowledged, and when? Consistency
   promises are the application's own words; no promise defaults
   silently, and no acknowledgement front-runs the head swap.
6. **Access shape.** How is state read — by the writer, by stale
   readers, by projections? Shapes are derived here, after use; a
   KV-like, graph-like, or vector-like shape earned by the use is
   legitimate spelling.
7. **Freshness and failure.** What staleness is admissible, declared,
   and bounded? What happens under substrate unavailability — which
   operations continue, which refuse honestly, with what types?
8. **Projection graph.** Every derived structure named, its generation
   fencing declared, its rebuild procedure stated. A projection without a
   rebuild procedure is a truth claim and fails the package.
9. **Evolution.** How do words get added, deprecated, translated?
   Recorded meaning is never reinterpreted retroactively; translation is
   an explicit versioned act.
10. **Proof surface.** The decisive observables that constitute the
    protocol's meaning — replay determinism, typed refusals, fencing
    under contention, stale-reader coherence — stated so `$state-gauge`
    can build gauges from them. A protocol whose proof surface names no
    decisive observable has stated nothing.

## Vocabulary Discipline

- Every distinction the application acts on gets a word; every word
  carries an observable consequence — a distinct fold result, projection,
  refusal, or replay outcome. The rule kills only synonyms; a rich domain
  earns a rich vocabulary without apology.
- Membership heuristic: what the world can legitimately do to the
  application is a word in history; what only a buggy or unauthorized
  speaker could produce is a typed refusal, and refusals never enter
  history.
- No generic blobs: an untyped accept-anything payload is a violation
  signature, not flexibility.
- **External framework boundaries.** Where the application composes an
  external framework whose interfaces carry their own store or state
  shapes, whose words enter canonical history is an explicit authored
  decision. The default is: canonical vocabulary is the application's
  own; the framework's contracts are edge seams served by adapters. If
  the rubric run finds reason to let framework words into history, that
  is an open human decision — surface it, never default it. Coupling
  history to a pinned framework's schema makes every upgrade a
  translation event in recorded meaning; keeping words owned makes an
  upgrade adapter work.

## The Design Package — Output Grammar

One package per application, landed as that application's promise-plane
material. Sections, each mintable:

- **Verb inventory** — the transition vocabulary with each word's
  observable consequence.
- **Transition grammar** — admissibility: which transitions may follow
  which states, and the typed refusal owed outside.
- **Object grammar** — how transitions, heads, and checkpoints spell to
  substrate objects; publication topology per the canon's commit law.
- **Projection graph** — question 8's answers, generation fencing and
  rebuild procedures included.
- **Consistency matrix** — durability and freshness declarations per
  operation family; no silent defaults.
- **GC and redaction policy** — what may be forgotten, how logical
  deletion advances the frontier, what redaction means against immutable
  history.
- **Proof surface** — question 10's answers, gauge-ready.

Completeness gate before the unit lands: every invariant lives inside one
lineage or carries declared choreography; every projection is
generation-fenced with a rebuild procedure; every refusal is typed; the
evolution section exists; every section passes the mint test. The
package names its acknowledged-risk line for every register entry its
surfaces intersect — `architecture/risk/use-derived-state/` included
while that entry stands.

## Relationships

- Governs: `$use-derived-state` (posture), the canon (law).
- Specializes: `$promise` — its grounding, termination, and mint-test law
  apply unchanged.
- Feeds: `$state-gauge` (the proof surface), 3010 (contracts mint from
  the landed package), the application's build lanes.
- Status: `architecture/risk/use-derived-state/`.

## This Is NOT

- Not the canon and not an amendment path to it.
- Not implementation, contract authoring, or gauge execution.
- Not floor authoring: nothing here creates shared state artifacts; the
  two-protocols-plus-human-gate law stands.
- Not an engine selector: question zero's "database earns its place"
  verdict routes to the human; the shelved engine is named only by a
  human, through the shelf entry's law.
