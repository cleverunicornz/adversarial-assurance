---
name: promise
description: The authoring practice for the promise plane — how an architecture document is written so contracts can mint oracles from it, how promoted knowledge becomes grounded direction, and where a promise chain must terminate. Use when writing or amending anything under architecture/ (except the risk register), when a 1000 verdict or 2000 promotion needs its knowledge converted into stated direction, or when a 3000 unit blocks on an unwritten promise. The charter at architecture/AGENTS.md owns what a document there is; this skill owns how one gets written. A statement from which no oracle can be minted is prose, not a promise — you do not get blood from a stone.
node: promise
class: skill
edges:
  - type: cites
    target: collapser
    provenance: declared
  - type: cites
    target: oracle
    provenance: declared
metadata:
  short-description: Author promises oracles can be minted from
---

# Promise

The architecture document is the top of the collapse chain: the
overarching definition of behavior — the user story in its binding
form — from which everything downstream derives. Contracts mint claims
and oracle refs from it, then bind collapse applications without changing
what was promised. If the minting fails, the failure is the document's,
and it is repaired here, never worked around downstream.

## When Promises Get Written

- After knowledge exists: a 1000 verdict, a 2000 campaign's promotion,
  or direct human design. A promise written ahead of its knowledge is a
  question wearing a promise's clothes. Classify the missing knowledge through
  the lifecycle router without creating 1000 or 2000 work.
- A research verdict, promotion package, or human promotion event is input to
  promise authoring, never promise-plane authority itself. Promotion adopts an
  obligation; it does not make the evidence tree a second architecture home.
- Where to check whether a promise already exists: the `architecture/`
  document owning the surface. Absent and load-bearing means a promise
  gap. Block per the 3010 entry gate and name this as a possible predecessor;
  the block creates no 6000 unit.
- Before anything claims it: a contract or build reaching past an
  unwritten promise has no oracle to answer to (charter law — the
  change lands here first).
- On the 6000 lane, through the standard unit flow.

## Grounding And Termination

Every downstream commissioned claim traces to a promise already landed in
`architecture/`. The handoff records the exact pinned architecture source and
the claim-to-promise mapping; a nearby document, similarly worded finding, or
resolving oracle is not a substitute.

The grounding chain terminates at that architecture source. None of these can
stand in for it:

- a 1000 report or 2000 promotion package;
- a human promotion event without the resulting architecture change;
- a 3000 contract, claim row, review finding, or proposed exclusion;
- a future 5000, 6000, implementation, deployment, or 7000 obligation; or
- another promise that merely says the missing promise will later be written.

That last form is promissory recursion: a promise of a promise. Refuse it. A
validator finding an unwritten or unmintable promise blocks the downstream
unit and returns to the missing predecessor; it never commissions another
claim, oracle, validator, or follow-up promise to make the gap appear closed.

Disposition is classification only:

- existing mintable architecture source: it may be pinned inside an existing
  3000 commission;
- missing source with closed knowledge and decided direction: possible 6000
  Promise work;
- missing knowledge answerable by reading and reasoning: possible 1000 work,
  whose separate result may later classify 6000 or 2000;
- a missing-promise question known to require execution or oracle genesis:
  possible 2000 work followed by human Promotion and possible 6000 work; and
- an open human-owned design choice: stop for the human.

No item above creates a branch, unit, pass, review, or follow-on act. Each
begins only under an express human commission.

The 3000 unit is never the author of its own premise. If it discovers the gap
late, it records the blocker and stops its review loop. A predecessor landing
does not resume 3000; refresh and re-entry require an already-admitted act or an
express human resume.

## The Mint Test

Every promise a document states must pass this before the unit lands.
For each stated behavior, draft — without writing it into the doc — the
claim row a contract would mint: statement, polarity, envelope, and a
resolvable oracle ref. If you cannot, the promise is missing one of its
four parts:

- **Owner** — the single writer of this truth.
- **Seam** — the typed surface where others meet it.
- **Domain** — where it holds, and the typed rejection owed outside.
- **Exclusivity** — what forbids a second writer or a second meaning.

Apply the mint test to the whole document, not isolated sentences. For one
owner, seam, domain, and input, the promise must resolve to one behavior or
typed rejection everywhere it speaks. Two individually mintable sentences
that assign different outcomes to that same case fail exclusivity. Distinct
outcomes are coherent only when their domains are explicitly disjoint.

A sentence that fails the mint test is removed from the candidate. If the
failure exposes empirical uncertainty, record only the possible 2000
classification under `Disposition` above; it creates no spike or continuation.
Prose that survives without mintability is the documentation layer failing
silently.

Do not solve the mint test by naming a test framework, harness, checker,
or review campaign in the promise. The promise defines the truth and its
domain; `$collapser` binds the strongest applicable mechanism later in
the contract. Name an instrument here only when ownership of that
instrument is itself part of the architecture promise.

Schematic (placeholders, not a claim about the tree):

- *Unmintable:* "The store handles large objects robustly." No owner,
  no seam, no domain edge, nothing to judge — prose.
- *Mintable:* "The artifact store is the sole writer of its bucket
  (**owner**). Clients reach it only through its one typed seam (**seam**).
  PUTs to 5 GiB succeed; larger receive the typed `RejectedTooLarge`
  (**domain**). No other service writes this bucket (**exclusivity**)."
- Mint check: elicit — "a 5 GiB PUT succeeds," oracle draftable as a
  seam-level test; exclude — "oversize is refused with the exact
  type," oracle draftable from the same harness. Both mintable → the
  promise passes.

## Relationships

- Downstream: `$oracle` (contracts mint judges from what you write);
  `$collapser` (contracts bind mechanisms to the minted claims); `3010`
  (the entry gate that blocks on unwritten promises).
- Beside: the risk register owns present-state risk — a promise carries
  no status and restates no entry (charter law).
- Direction of time: this plane states the future; where the tree
  disagrees with a promise, the code is the defect — which is exactly
  why an unmintable promise is dangerous: it indicts code against a
  standard nobody can measure.

## This Is NOT

- Not the charter: what a document is, and how it cites risk, live at
  architecture/AGENTS.md — point, never restate.
- Not the contract plane: this skill mints nothing; it makes minting
  possible.
- Not a status or history surface, and never a place for open questions
  (closed artifacts only, per root).
