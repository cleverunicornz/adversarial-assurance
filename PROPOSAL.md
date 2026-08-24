# Proposal v3 — the Promise/Witness/Oracle pipeline

Status: **PROPOSAL v3**. Supersedes v2. v3 restores the frame after two
adversarial reviews drifted it into a software-contract problem: this is
**agent orchestration** — semantic entities — with exactly one deterministic
seat (the CI witness). v2's record model (YAML-LD), two-lane split, and
actor-driven init survive; their execution is simplified to v3's pointer +
init-file model.

Prior documents remain as history: `PLAN.md`, `PLAN-REVIEW.md`,
`PROPOSAL-REVIEW.md`.

## 1. What this is

A **self-contained, repo-agnostic agent-orchestration spec**. It is not a
program, not a toolset, not a CI product. It is a semantic pipeline: roles
(agents), a record language (YAML-LD), and a single deterministic witness
(the workflow) that keeps everyone honest. Everything else is semantic and
flexible, bound per-adoption by a small init file.

Three layers:

| Layer | Where | What it is |
|---|---|---|
| **The pack** | this repository (remote, general) | the agnostic layer: skills, YAML-LD record vocabulary/schema, the `assurance` crate, the workflow template, laws. Everyone consumes it unchanged. |
| **`.assurance/`** | per-adopting-repository | created by bootstrap: the init file, materialized schema, records, registry state. |
| **The init file** | `.assurance/assurance-init.{yaml,md}` | the binding surface — the only per-adoption specificity. |

## 2. Skills point; init binds

Skills never hardcode a model, a harness, an actor, or a runner. Whenever a
skill needs to know *who or what* to launch, it carries a **pointer** to the
init file. The skill says "run an agent"; the pointer resolves to
`.assurance/assurance-init`, which says — **relative to itself**:

- which models, per role (lead, worker, validator, reviewer);
- which harness(es) are used to run agents;
- which runners CI uses;
- who fills each actor seat (e.g. who is the final validator, who is the
  reviewer);
- anything else the adoption needs.

An agent reads the skill, follows the pointer, reads the init file, and
**knows exactly whom to launch**. Nobody guesses, nothing is hardcoded in the
general layer.

The init file is produced by bootstrap and **committed to the adopting repo**:
- created the first time by the agent, who asks the human "what models, what
  harnesses, what runners?" and codifies the answers;
- versioned by git — a change is a visible diff that propagates;
- editable at any time by an agent or a human — change the final validator
  by editing one file;
- replaceable at any time; the pack never changes, the init file does.

Agnosticism means the **template is generic and the init file binds**. The
remote repository stays the general layer for everybody; the init file is the
point where generic becomes specific, per repository.

## 3. The record language: YAML-LD with prose

- All pipeline records are **YAML-LD** documents (`@id`, `@type`, `@context`,
  typed edges/linksets) — graph relationships are native to the
  representation.
- The pack defines the **vocabulary** — the verbs and nouns of the pipeline —
  as an **exact schema**. The schema is the comprehension contract: an agent
  reads it and knows how to author records.
- Records are **not** pure typed fields. Within the schema, scalar/typed
  members coexist with **free-text bodies via block scalars** (`|` literal,
  `>` folded): an agent writes *declarative statements* — prose assertions —
  inside a document that is simultaneously a graph node the checker can
  verify. This is the "schema with prose" entity: structured at the outer
  layer, expressive inside.

## 4. The core vocabulary: Promise, Witness, Oracle

The pipeline's semantic triad, standardized:

- **Promise** — the assertion / commitment being made; the thing to be
  verified (this is what was called "claim"; the word "claim" is retired).
  A promise carries polarity and an envelope.
- **Witness** — the attestation: "yes, that exists / that ran." Evidence,
  artifacts, and the CI log itself are witnesses, linked by content digest
  and provenance.
- **Oracle** — the judge: the authority that evaluates the promise against
  the witness and renders a disposition (pass / fail / blocked), with
  rationale.

Record edges in YAML-LD: `promise witnessed_by witness`,
`promise judged_by oracle`, `promise resolves_to …` — the nouns and verbs are
the schema. The existing pack skills already named `promise`, `witness`,
`oracle` align to this triad; record types/schema ids that used "claim"
(`assurance-claim/v1`) migrate to `assurance-promise/*` in the RECORD ledger.

## 5. The witness: CI is the checker's only seat

- The **`assurance` crate** lives in this repository (`assurance/`), built and
  executed by the workflow on the runners bound in the init file. This is the
  main (only) consumer; no separate distribution repository. The workflow
  clones/builds it at a pinned ref of this repo.
- The workflow **is the witness**: its log is the verifiable record — "that
  ran, on these inputs, with this result." That witnessing is the entire
  purpose of running it in CI. A local CLI invocation produces no witness,
  so **agents do not run the checker, do not fall back to it, and do not
  round-trip through it locally**. If the workflow fails, it fails.
- The checker is *deterministic where determinism is valid — and only there*:
  - **structure** — records conform to the YAML-LD schema (vocabulary, edges,
    required members);
  - **witness linkage** — every promised artifact exists at its pinned
    ref/digest (the promise's witness is real and reachable);
  - **disposition legality** — the oracle's verdict is a member of the
    permitted set for its stage; forbidden successors (e.g. progression after
    BLOCKED) are rejected.
  - It does **not** judge the promise's truth. Judgment is the oracle's
    semantic act; the checker witnesses that it happened, is coherent, and is
    on-schema.
- Determinism confined to the witness keeps the rest of the pipeline semantic
  — which is the whole point: agents are semantic entities, CI is the honest
  record.

## 6. Init / bootstrap (actor-driven)

First adoption of the pack into a repository:

1. An agent (or the human) copies/matrializes the pack once — the workflow
   template, the schema, the `assurance` crate reference.
2. The agent runs **bootstrap**: discovers the environment, then **asks the
   human** — "what models do you want for each role? which harness? which
   runners? who is the reviewer / final validator?" — and writes those
   answers into `.assurance/assurance-init`.
3. Bootstrap seeds `.assurance/`: init file, schema, registry in an explicit
   `UNCONFIGURED`/`CONFIGURED_EMPTY` state, and the first run records.
4. Everything human-authored-through-git: the initial commit establishes the
   adoption; all later changes are git diffs on the init file and records.

That is the whole binding surface — one file, one question-answer step. There
is nothing to codify into the remote repository; the remote is general.

## 7. Fidelity and the migration ledger

- **Immutable core**: the promise/witness/oracle semantics, the admission
  graph (1000 research → 6000 document / 2000 spike → 3000 contract → 4000
  proof; 5000/6000 independent lanes; 7000 human-invoked adversarial review),
  gates and dispositions, and the evidence law. Never altered, never scoped
  out.
- The ledger classifies every sentence:
  - **NORM** — retained verbatim (core semantics);
  - **BIND** — becomes a pointer to the init file (`run the agent` →
    `run the agent per `.assurance/assurance-init``); executor-only change,
    meaning retained;
  - **RECORD** — becomes a YAML-LD record under the promise/witness/oracle
    vocabulary (e.g. `assurance-claim/v1` → `assurance-promise/*`);
  - **INSTANCE** — source-repository-only content (its services, graph
    instance, product surfaces, hardcoded models/hostnames) — dropped, the
    generic law kept.

## 8. What the pack ships

```
adversarial-assurance/
  PROPOSAL.md, README.md, SANITIZATION.md, (PLAN* / PROPOSAL-REVIEW as history)
  skills/            # semantic orchestration; pointers to the init file where bindings are needed
  schema/            # YAML-LD vocabulary: nouns + verbs, record shapes, contexts
  assurance/         # the Rust crate (checker) — built/run by the workflow, never by agents
  laws/              # binding authority: root/lifecycle/runs/promise/witness/oracle laws
  workflow/          # the witness workflow template (GitHub reference) + bootstrap procedure
```

The pack is general; the init file is specific; the workflow is the witness.

## 9. Acceptance criteria

1. An empty repo adopts the pack: bootstrap asks the human the questions,
   writes `.assurance/assurance-init`, commits; the workflow runs green on the
   bound runners.
2. Changing a binding (models, reviewer, runners) = editing one init file and
   a git diff; the skills needed no change.
3. Records are YAML-LD on the promise/witness/oracle vocabulary, verified by
   the checker; a deliberately broken record or a forbidden successor fails
   the workflow with a verifiable log.
4. Agents never execute the checker; the witness is CI's log.
5. Zero source-repository references; core (section 7) unchanged; the ledger
   accounts for every sentence.

## 10. Execution order

1. Author the schema: promise/witness/oracle vocabulary, record shapes,
   contexts (the comprehension contract).
2. Write the `assurance` crate (structure, witness-linkage, disposition
   legality) + the witness workflow + the bootstrap procedure.
3. Sweep the 81 skills through the ledger: NORM / BIND→init pointer /
   RECORD→YAML-LD / INSTANCE→drop.
4. Fresh-repo proof: bootstrap → 1000/2000/6000/3000/4000 + a human-invoked
   7000 → workflow witnesses each gate.
5. Adversarial review (Sol), then human review. Nothing commits or pushes
   before review.

## 11. Open for adversarial review

- Is the promise/witness/oracle triad sufficient as the full vocabulary, or
  do records for campaign/contract/closure/run-state need their own nouns?
- How deep does the "claim"→"promise" migration go (record types, schema
  ids, skill text), and does any "claim" usage carry distinct meaning that
  must survive?
- Is limiting the checker to structure/linkage/legality the right boundary,
  or does the witness need one more mechanizable pass?
- The init file is the single binding surface — is anything non-model,
  non-harness, non-runner inherently per-adoption that still needs a home
  there?
