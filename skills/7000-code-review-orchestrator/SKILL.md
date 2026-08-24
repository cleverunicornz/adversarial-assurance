---
name: 7000-code-review-orchestrator
description: Orchestrate a human-invoked adversarial assurance campaign against a pinned repository surface. Use for delivery-proof audit, detector integrity, bug hunting, subsystem qualification, composed change-bank review, or any explicit request to falsify advertised behavior and retain replayable evidence. The manager owns stage order, durable YAML-LD run records, and the human gate; it never fixes reviewed source or becomes a completion dependency.
---

# 7000 Code Review Assurance Orchestrator

Resolve every {{variable}} from situation/assurance/assurance-init.yaml before acting.

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

Before intake, self-check the active manager identity, resolved model, actor
seat, and strongest configured reasoning posture against these bindings. A
mismatch is `BLOCKED`: tell the human exactly which binding/posture must be
used and stop. Never silently continue under a substitute manager.

Launch each child with no conversation history and one closed docket. Carry
dependencies through committed records and typed Passback, never hidden chat
state. The manager owns follow-on authority; a child may request work but may
not launch descendants or silently widen its assignment.

When the repository is substrate-formed, treat `situation/definition/`
invariants and adopted-protocol vertices as a standing lens beside every stage
rubric. Read `situation/risk/` and `situation/architecture/` by default. A
blocking protocol violation enters the run as a confirmed-candidate finding
routed to the human gate. Repository git policy, build graphs, search tools,
environment receipts, and operational runbooks remain optional capabilities;
use them when present and required by the charter, never invent substitutes.

## Assurance Record Contract

All campaign state lives under:

```text
situation/assurance/runs/<run-id>/
  run.yamlld
  promises/*.yamlld
  witnesses/*.yamlld
  oracles/*.yamlld
  evidence/**                     # nested regular non-record files; no symlinks
  graph.trig                      # generated
```

Run ids and every record filename are lowercase kebab-case. Use only the nouns
and verbs in `situation/assurance/schema/vocabulary.yaml`.

Every record carries quoted `@context`, placement-derived `@id`, allowed
`@type`, `schema_version: 1`, non-negative `sequence`, non-empty `label`, and a
non-empty block-scalar `body` no larger than 16 KiB.

- Promise adds `polarity`, non-empty `envelope`, non-empty `witnessed_by`,
  non-empty `judged_by`, and `part_of`.
- Witness adds one file `resolves_to` using
  `urn:assurance:path/<repo-relative>`, its exact SHA-256, `producer`, and
  `part_of`.
- Oracle adds `actor`, `PASS | FAIL | BLOCKED`, one file `resolves_to`, and
  `part_of`. Ordinary manager/stage Oracles use `{{reviewer_seat}}`; only
  independent proof-validation Oracles use `{{final_validator_seat}}`.
- Run adds `lane: "7000"`, `state: OPEN | CLOSED`, and
  `human_invoked: true`. Other vocabulary lanes are reserved for future
  expansions; this pack exercises 7000 only.

Bodies are bounded summaries, not whole reports. Full charter, survey, proof,
validation, trace, retrospective, and report documents are committed under
`evidence/<stage>.md` and named by a digest-bound Witness. Every committed
evidence file is the target of a same-run Witness. A bulky external payload is
represented by a committed `*.external-artifact.yaml` manifest containing
`version`, immutable `external_uri`, payload `sha256`, `size`, and
`provenance`; the Witness hashes the manifest, not the remote payload.

`ci_run` is optional. Set it only to an already-completed immutable CI URL;
omit it for the current not-yet-known run. Never author a future or
self-referential URL.

House-style minimum records:

```yaml
"@context": "urn:assurance:context/v1"
"@id": "urn:assurance:record/example-review/run"
"@type": "urn:assurance:ontology/Run"
schema_version: 1
sequence: 0
label: "Example adversarial review"
body: |
  Human-invoked review of the pinned target; stage: chartering.
lane: "7000"
state: OPEN
human_invoked: true
```

```yaml
"@context": "urn:assurance:context/v1"
"@id": "urn:assurance:record/example-review/promise/charter"
"@type": "urn:assurance:ontology/Promise"
schema_version: 1
sequence: 1
label: "Charter completeness"
body: |
  Every instructed behavior has a G-### guarantee or named ambiguity.
polarity: ELICIT
envelope: "The human instruction and pinned target define the complete scope."
witnessed_by:
  - "urn:assurance:record/example-review/witness/charter"
judged_by:
  - "urn:assurance:record/example-review/oracle/charter"
part_of: "urn:assurance:record/example-review/run"
```

```yaml
"@context": "urn:assurance:context/v1"
"@id": "urn:assurance:record/example-review/witness/charter"
"@type": "urn:assurance:ontology/Witness"
schema_version: 1
sequence: 2
label: "Charter artifact"
body: |
  Digest-bound charter written by the configured reviewer seat.
resolves_to: "urn:assurance:path/situation/assurance/runs/example-review/evidence/charter.md"
artifact_sha256: "0000000000000000000000000000000000000000000000000000000000000000"
producer: "{{reviewer_seat}}"
part_of: "urn:assurance:record/example-review/run"
```

```yaml
"@context": "urn:assurance:context/v1"
"@id": "urn:assurance:record/example-review/oracle/charter"
"@type": "urn:assurance:ontology/Oracle"
schema_version: 1
sequence: 3
label: "Charter gate"
body: |
  PASS: inventory and falsification criteria are complete.
actor: "{{reviewer_seat}}"
disposition: PASS
resolves_to: "urn:assurance:path/situation/assurance/runs/example-review/evidence/charter.md"
part_of: "urn:assurance:record/example-review/run"
```

Replace the example digest with the actual committed file digest. The first
campaign commit contains `run.yamlld`, `evidence/`, and at least one complete
Promise/Witness/Oracle triad together. A run-only or partial-triad first commit
is checker-red.

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`.
Recovery authors a fresh complete triad; it never advances or rewrites the
blocked chain. Raw domain text saying BLOCKED inside an otherwise-PASS stage
Oracle body is not the same state. If any top-level BLOCKED Oracle remains, the
Run may not declare `succeeded_by`.

Every stage publishes a coherent bundle on its unique evidence commit.
Missing, stale, conflicting, dangling, or inverse-uncovered evidence fails
closed. Closed evidence commits are immutable. `assurance build` owns each run
graph plus `situation/assurance/graph-manifest.yaml`.

The workflow's substrate check and assurance check/build logs on
`{{witness_runner}}` are the Witness for the bundle. Local invocation is
authoring preflight only and never replaces either checker. The compiled graph
is a deterministic relationship index; depth remains behind Witness pointers.

## Human Intake

The human and manager agree on:

- exact target identity: commit SHA, immutable patch, composed change-bank
  SHA, or named surface at a pin;
- advertised behavior or question to attack;
- `situation/risk/` and `situation/architecture/` by default, without treating
  a suspect surface, historical deployment, or passing evidence as current
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

The active manager counts inside the lead/validator ceiling. Every scout,
executor, validator, rerun, second wave, and variant proof counts against the
stated total child cap.

Assignments follow disjoint surfaces or lenses, never arbitrary file counts.
Reruns, wave two, and variant proofs count against the cap. Increasing class
requires a recorded reason; exceeding `large` requires explicit human
approval.

## First Act

1. Pin the target. Capture a working diff as an immutable patch or commit
   before proof execution.
2. Mint a lowercase kebab-case `<surface-slug>-<yymmdd>-<uid4>` run id.
3. Create the run folder, evidence directory, and `run.yamlld` with lane
   `"7000"`, state `OPEN`, and `human_invoked: true`; set registry `ACTIVE`.
4. Record instruction, target pin, tracks, budget, exclusions, optional
   authorities, and stage marker in the Run body.
5. Author the initial charter summary/evidence and commit `run.yamlld` plus
   one complete Promise/Witness/Oracle triad together. The manager Oracle uses
   `{{reviewer_seat}}`. Never commit run-only or partial-triad state.
6. Launch `7005-review-charter-guarantees` through `{{harness}}` using
   `{{lead_model}}` and reconcile its charter with the human before later
   assignments.
7. Run authoring preflight, commit generated graph/manifest bytes, and record
   the integration commit so another manager can resume without chat history.

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

`Passback` is the child's terminal typed return: docket digest, target and
input commits, owned output record/artifact paths, artifact digests, observed
domain result, blocker/residual, requested follow-on work, and terminal
response. It grants no descendant or continuation authority.

`Rubric status` means only `complete`, `partial(<missing>)`, or
`blocked(<why>)` against that stage's declared duties. It is not a semantic
campaign verdict.

The manager never delegates the immediate critical-path decision it needs
next. A child may return a closed request; the manager decides whether the
charter and budget admit it.

## Independent Validation Authority

A proof-executor commit is a candidate, never an accepted result. A different
`review_proof_validator`, launched through `{{harness}}` with
`{{validator_model}}` and bound to `{{final_validator_seat}}`, alone emits the
proof-validation disposition.

The validator emits its raw domain disposition in its Oracle body. That
Oracle's top-level `PASS` means faithful independent replay completed and a raw
disposition was produced; `FAIL` means a concrete validation-procedure defect;
`BLOCKED` means a required prerequisite was unavailable. The validator never
selects a mapping row. The integrity lead later verifies the bindings, applies
the total predeclared map, and records the selected row in lead-owned records.
The manager may verify lineage but may not reinterpret, suppress, downgrade,
round, or adjudicate either result.

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
11. **Process feedback:** conditional; run only when discoverable
    `witnesses/feedback-*` records exist.
12. **Mechanical sweep:** verify every id, proof, application, blocker,
    budget cut, evidence path, and immutable lineage. Emit
    `promises/mechanical-sweep.yamlld`,
    `witnesses/mechanical-sweep.yamlld` resolving to
    `evidence/mechanical-sweep.md`, and
    `oracles/mechanical-sweep.yamlld`. The Oracle uses
    `{{reviewer_seat}}` and judges sweep completeness only.
13. **Assurance retrospective:** after the sweep and complete timing evidence,
    audit the cost of knowing without changing any technical verdict.
14. **Report:** compile the complete terminal accounting and required
    retrospective.
15. **Human gate:** present guarantees, defects, test weaknesses, unresolved
    risk, budget cuts, actions, and standing questions. Never self-promote.

## Durable Recovery

After each handoff, proof, validation, recovery, or stage publication, commit
the corresponding YAML-LD bundle with role, docket digest, immutable inputs,
artifact paths/digests, Passback, disposition, and next action. Git history is
transition authority.

The integration commit chain is explicit: target/base pin → manager
integration commit containing admitted stage inputs → role-owned immutable
candidate commit → independent validation commit → manager integration commit
that records the predeclared mapping row. A staging tip is the latest admitted
manager integration commit; an integration base is the human-selected commit
onto which promotion proposals are prepared. None is inferred from chat.

On resume, read `run.yamlld`, every triad record, evidence commits,
`graph.trig`, and `graph-manifest.yaml` before chat history. Recovery from a
top-level BLOCKED Oracle always creates a fresh complete triad and fresh closed
docket.

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
3. the mechanical-sweep and report bundles are committed;
4. the human gate was presented; and
5. the human promoted, explicitly deferred, or abandoned the run.

The Run stays `OPEN` through the `complete-awaiting-promotion` body marker.
That marker is not a Run state. Set state `CLOSED` only after the human
promotes, explicitly defers, or abandons. Closure grants no authority to fix,
deploy, archive, or start another campaign.
