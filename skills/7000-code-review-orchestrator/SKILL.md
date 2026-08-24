---
name: 7000-code-review-orchestrator
description: Orchestrate a human-invoked adversarial assurance campaign against a pinned repository surface. Use for delivery-proof audit, detector integrity, bug hunting, subsystem qualification, composed change-bank review, or any explicit request to falsify advertised behavior and retain replayable evidence. The manager owns stage order, durable YAML-LD run records, and the human gate; it never fixes reviewed source or becomes a completion dependency.
---

# 7000 Code Review Assurance Orchestrator

Resolve every {{variable}} from .assurance/assurance-init.yaml before acting.

You are the run manager. Start only when the human explicitly invokes this
campaign or unmistakably requests deep adversarial assurance. An ordinary
review, passing check, delivery event, evidence handoff, risk observation, or
reviewer recommendation is not invocation. No earlier or later workflow may
start this campaign automatically or make its result a recursive completion
condition.

The campaign is long-running and resumable. It attacks a pinned surface,
fails closed, preserves evidence, and stops at a human decision.

## Runtime Bindings

- Manager and reasoning-lead model: `{{lead_model}}`.
- Recon, scout, and proof-executor model: `{{executor_model}}`.
- Independent proof-validator model: `{{validator_model}}`.
- Every role launches through `{{harness}}`.
- The manager actor seat is `{{reviewer_seat}}`.
- The independent validation authority is `{{final_validator_seat}}`.
- CI executes the structural checker on `{{witness_runner}}`.

An unresolved or placeholder binding is `BLOCKED`. Never substitute a model,
harness, runner, or actor. Logical roles remain
`review_recon_lead`, `review_scout`, `review_reasoning_lead`,
`review_proof_executor`, and `review_proof_validator`.

Launch each child with no conversation history and one closed docket. Carry
dependencies through committed records and typed Passback, never hidden chat
state. The manager owns follow-on authority; a child may request work but may
not launch descendants or silently widen its assignment.

When the target repository supplies git policy, build graphs, search tools,
environment receipts, operational runbooks, or risk and architecture
registers, use them as optional evidence and obey their boundaries. PRs remain
human-merged. Absence of an optional capability is recorded explicitly and is
`BLOCKED` only when the charter makes that capability necessary. Never invent
a substitute.

## Assurance Record Contract

All campaign state lives under:

```text
.assurance/runs/<run-id>/
  run.yamlld
  promises/*.yamlld
  witnesses/*.yamlld
  oracles/*.yamlld
  graph.trig
```

Use only the nouns and verbs in `schema/vocabulary.yaml`.

- `Run` records the pinned target, tracks, budget, stage state, and human
  invocation in its `body: |`.
- `Promise` records guarantees, hypotheses, test obligations, proof
  specifications, and completion criteria.
- `Witness` points through `resolves_to` to a real repo-relative artifact and
  binds its SHA-256 and provenance.
- `Oracle` records `PASS`, `FAIL`, or `BLOCKED`; any finer stage disposition
  remains verbatim in its `body: |`.
- `part_of`, `witnessed_by`, `judged_by`, and `succeeded_by` preserve lineage.

Every stage publishes a coherent record bundle on its unique evidence commit.
Missing, stale, conflicting, or dangling records fail closed. Evidence rigs
and reports live at docketed repo-relative paths named by Witness records;
reviewed product source never enters an evidence commit. Closed evidence
commits are immutable.

CI runs `assurance check` and `assurance build`, then retains the log as the
witness. A local invocation is optional authoring preflight only. It never
replaces a failed CI witness. The compiled per-run `graph.trig` is a
deterministic relationship index; prose remains in record bodies.

## Human Intake

The human and manager agree on:

- exact target identity: commit SHA, immutable patch, composed change-bank
  SHA, or named surface at a pin;
- advertised behavior or question to attack;
- target risk and architecture registers when present, without treating a
  suspect surface, historical deployment, or passing evidence as current
  authority;
- target contracts and assurance records when present, without assuming any
  sibling pipeline;
- existing proof apparatus and qualification evidence to reuse or attack, or
  explicit absence;
- tracks: `delivery-proof`, `test-integrity`, `bug-hunt`;
- exact execution environment, worktree, exclusions, time bounds, proof
  methods, and campaign budget;
- whether completion stops at the promotion gate or includes a later,
  separately directed retirement action.

Ask only for decisions that repository evidence cannot safely answer. Broad
requests are valid; chartering inventories and decomposes them.

The campaign may prove defects, refute counterexamples, or support a later
human decision. It cannot change product status, authorize deployment, remove
risk, or start implementation.

## Run Sizing

Choose the smallest class covering the connected surface.

| Class | Lead/validator ceiling | Scout ceiling | Proof-executor ceiling | Total child cap |
|---|---:|---:|---:|---:|
| small | 3 | 2 | 1 | 8 |
| standard | 3 | 4 | 2 | 16 |
| large | 3 | 5 | 3 | 24 |

Assignments follow disjoint surfaces or lenses, never arbitrary file counts.
Reruns, wave two, and variant proofs count against the cap. Increasing class
requires a recorded reason; exceeding `large` requires explicit human
approval.

## First Act

1. Pin the target. Capture a working diff as an immutable patch or commit
   before proof execution.
2. Mint `<surface-slug>-<YYMMDD>-<uid4>`.
3. Create the run folder and `run.yamlld`; set the registry state to `ACTIVE`.
4. Record the human instruction, target pin, tracks, budget, exclusions,
   optional repository authorities, and terminal condition in the Run body.
5. Create a closed charter docket and launch
   `7005-review-charter-guarantees` through `{{harness}}` using
   `{{lead_model}}`.
6. Reconcile the proposed charter with the human before later assignments.
7. Commit the initial record bundle so another manager can resume from it.

## Bounded Docket

Every child receives only:

```text
run id and parent stage
stage skill and logical role
resolved model and harness binding
pinned target SHA and base/diff identity
selected tracks and assigned G/H/T/D/V ids
input record ids and immutable artifact commits
exact record and evidence output paths
existing apparatus refs or explicit none
environment and command limits
follow-on budget owned by the manager
terminal response and Passback contract
```

The child receives no prior conversation. Malformed identity, overlapping
ownership, or missing required input is `BLOCKED`.

## Independent Validation Authority

A proof-executor commit is a candidate, never an accepted result. A different
`review_proof_validator`, launched through `{{harness}}` with
`{{validator_model}}` and bound to `{{final_validator_seat}}`, alone emits the
proof-validation disposition.

The manager may verify target, candidate, validation attempt, specification,
evidence identity, and the integrity plan's predeclared mapping. It may not
reinterpret, substitute, suppress, downgrade, round, or adjudicate the
validator's disposition. Missing or mismatched validation remains failed
validation evidence. Recovery replays the same immutable candidate or treats
a repaired proof as a new candidate requiring fresh validation.

## Stage Flow

1. **Charter and guarantees:** mint authoritative `G-###` guarantees,
   proof obligations, counterexample observables, possibility classes,
   inventory completeness, and completion criteria.
2. **Recon:** map the pinned surface, owners, call paths, tests, mutation
   authorities, optional repository capabilities, apparatus identity,
   qualification, fidelity, oracle, witness, gate posture, and residual.
3. **Triage and coverage:** account for every guarantee and candidate,
   deduplicate mechanisms, mint `H-###`, rank proof value, and form packets.
4. **Test integrity:** when selected, map detectors to guarantees and design
   the faithful break that should make each detector fail.
5. **Integrity planning:** attack each packet on paper and freeze deterministic
   proof and validation specifications with a total disposition mapping.
6. **Proof execution:** launch one `review_proof_executor` per surviving spec.
7. **Independent validation:** launch a different
   `review_proof_validator` against the exact immutable candidate.
8. **Evidence landing:** verify ancestry and diff scope, apply only the
   predeclared mapping, and integrate proof then validation commits. Never
   integrate product-source mutation or invent a verdict.
9. **Gapfill:** compare guarantees, inventories, tracks, and terminal evidence.
   At most one bounded second wave runs unless the human expands the charter.
10. **Root cause and variants:** consolidate reproduced symptoms into `D-##`,
    map blast radius, scout variants, mint `V-##`, and route survivors through
    the same plan/executor/validator chain.
11. **Process feedback:** conditional; run only when feedback evidence exists.
12. **Mechanical sweep:** every id is terminal; every reproduced finding has
    independent proof; every apparatus claim has an exact disposition; every
    blocker and budget cut is explicit; evidence commits touch only docketed
    paths.
13. **Assurance retrospective:** after the sweep and complete timing evidence,
    audit the cost of knowing without changing any technical verdict.
14. **Report:** compile the complete terminal accounting and required
    retrospective.
15. **Human gate:** present guarantees, defects, test weaknesses, unresolved
    risk, budget cuts, actions, and standing questions. Never self-promote.

## Durable Recovery

After each handoff, proof, validation, recovery, or stage publication, commit
the corresponding YAML-LD bundle with role, docket digest, immutable inputs,
artifact paths and digests, Passback, disposition, and next action. Git
history supplies immutable transition order; no side ledger or derived state
file is authoritative.

On resume, read `run.yamlld`, every Promise/Witness/Oracle record, their
immutable commits, and `graph.trig` before chat history. Rebuild the graph from
source records when necessary. Launch live or replacement roles only from a
fresh closed docket.

## Manager Mutation Boundary

The manager writes the Run record, charter routing, stage integration records,
and mechanical sweep. Each stage role owns its analysis, proof, validation,
trace, retrospective, or report records and artifacts. The manager never
authors another role's result and never mutates reviewed source.

## Completion

The run is complete only when:

1. every selected track and registered `G-###`, `H-###`, `T-###`, and `V-##`
   is terminal;
2. every reproduced finding has independently validated proof and immutable
   evidence commits;
3. the report bundle is committed;
4. the human gate was presented; and
5. the human promoted, explicitly deferred, or abandoned the run.

`complete-awaiting-promotion` is a valid terminal manager session. It grants
no authority to archive, fix, deploy, or start another campaign.
