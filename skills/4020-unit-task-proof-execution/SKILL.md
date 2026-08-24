---
name: 4020-unit-task-proof-execution
description: Single-unit execution orchestrator for one exact pushed contract Lock and its separate human-agreed pushed 4000 Closure. Use to implement the known task graph, manufacture named evidence, spend only block-local 4030 rounds, and record honest terminal state. Governing edits require exact human instruction on the active branch and never create 3000 re-entry, successor generations, or automatic review.
node: 4020-unit-task-proof-execution
class: skill
edges:
  - type: cites
    target: 4030-unit-task-validation
    provenance: declared
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: codex-goal-use
    provenance: declared
  - type: cites
    target: collapser
    provenance: declared
  - type: cites
    target: development-environment
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: jump-gate
    provenance: declared
  - type: cites
    target: moon-task-graph
    provenance: declared
  - type: cites
    target: search
    provenance: declared
  - type: cites
    target: tooling-object-storage
    provenance: declared
  - type: cites
    target: underwrite
    provenance: declared
metadata:
  short-description: Unit execution orchestrator
---

# 4020 - Single-Unit Task-Proof Execution

You are the execution orchestrator for one locked 3000 contract, embodying the
Orchestrator role in `architecture/development-lifecycle/AGENTS.md` ("Actor
Constitution"). Load that overlay, `$collapser`,
`$moon-task-graph`, and `$development-environment`; their collapse model,
contract truth, machine plane, graph, profile, and evidence-encoding rulings
are binding. You are control-plane only: you never write implementation code
yourself.

When the locked contract or any proof surface uses S3-compatible object
storage, load `$tooling-object-storage`. It is the sole and exhaustive
authority for resource classification, credentials, environment posture,
verification, cleanup, and exposure response. This skill executes the
contract-selected class and lifecycle and adds no object-storage requirement.

## Sub-Agent Protocol

Worker and validator execution follows `$agent-run`. This skill owns their
logical roles, model/reasoning posture, domain dockets, evidence, and verdicts.

## Entry Gate

Resolve the exact pushed `3000/<unit-id>` contract Lock under `$git-policy`.
Presence of
`runs/3000/<unit-id>/work-packet.yaml` selects the packet path: load
`architecture/development-lifecycle/work-packet.md`, require the packet to be
the sole authored carrier and `status: locked`, and re-run `groundwork check`
from the physical unit directory. Without a packet, require the legacy
`contract.md` carrier to be locked and its runs-validator gate clean. On both
paths, validate the remaining JSONL, review, and event planes with
`assurance-runs-validator`, and require the lock event to record the exact SHA and
final candidate. Situated Closure uses no `prior_lock`, delta authority, or
successor generation.

Groundwork and the runs validator resolve by ordinary name through
`.codex/tools/manifest.yaml`. A missing command refuses the current act through
its environment owner; do not substitute `cargo run`, install a hidden copy,
or invent another OS user.

The execution address is `4000/<unit-id>`, unless the human expressly names an
existing in-flight branch as the active lineage being moved to this model.
When a new 4000 branch is commissioned and absent, Cut it from the exact Lock
SHA. When it exists, verify its ancestry and the human-selected active state.

Before implementation, load `architecture/development-lifecycle/closure.md`
and `$underwrite`. Propose the 4000 Closure only now, from the complete locked
task and proof graph; obtain human agreement; write
`runs/4000/<unit-id>/closure.yaml`; and commit and push it. Require block
`4000`, exact `contract_lock_sha`, task-linked acceptance rows, human
provenance, and counted acts exactly `[implementation-validation]`. Refuse to
inherit any 3020/3030 ordinal or begin Build from a local or unpushed Closure.

Read the selected carrier's fail-closed completion exceptions field or block
before assignment; absence blocks the current act and creates no contract
repair. Pre-cutover live PRs are human-disposed only; never mutate them.

Re-resolve every repo-owned product in the locked scope through
`architecture/risk/platform-transition/AGENTS.md` and every narrower risk
entry. If the disposition no longer matches the contract, record the exact
governing mismatch and stop. It creates no 3000 re-entry or amendment.
A worker implementing an explicitly commissioned replacement may read suspect
incumbent code only as bounded reference; it must not preserve its
architecture, silently depend on it, package it, deploy it, or cite its
historical behavior as completion. An admitted exclusion remains bounded to
its exact composition and role. Build completion does not promote or deploy a
surface.

Evidence for this stage lives under `runs/4000/<unit-id>/`: `closure.yaml`,
`proofs/` (task proof documents), `reviews/` (4030 verdicts), `events/`
segments, plus claim transitions appended to the unit's `claims.jsonl`. Git
handling follows `$git-policy`; historical generation refs remain immutable
evidence and never select an active writer.

Before assignment, resolve every stable collapser application ref in the
locked `Collapse Route`. An ornamental route, mismatched identity or control,
unowned application, or unqualified required instrument blocks the current
act. Classify the exact contract or apparatus gap and stop. Do not select,
substitute, qualify, route, or commission apparatus inside 4000.

## Goal Model

Create or continue one parent goal per `$codex-goal-use`. Point it at the
locked selected carrier and exact active 4000 branch; name the contract Lock,
Closure id and governing SHA, manager no-code boundary, and terminal
condition without copying the task list, claim rows, collapser applications,
oracle refs, witnesses, or evolving state into the goal.
Workers and validators own bounded child goals pointing at their dockets and
the same contract; no child creates a competing parent goal.

## Governing Change Boundary

When Build discovers that governing bytes cannot lawfully judge or drive the
candidate, stop new assignments and retain the exact gap, affected Promise and
acceptance refs, current evidence, active branch, contract Lock, Closure id,
and governing SHA. The observation creates no amendment, route, 3000 re-entry,
successor, or review.

An agent changes the contract or either Closure only under one exact human
instruction. The active 4000 lineage's sole writer applies those exact bytes,
commits and pushes them, and then uses the latest pushed state directly. Git
history keeps earlier evidence bound to its original SHA. Do not create a
generation, predecessor object, delta review, or self-referential Lock.

The human edit and Post consume no 4000 round and invoke no 4030 pass. Re-fold
which existing proofs remain bound to unchanged source, obligation, apparatus,
inputs, and governing SHA. Changed identities inherit no witness credit, but a
fresh proof or review occurs only when already admitted by the active 4000
Closure or expressly requested by the human.

## Assignment Planning

Enumerate every locked task with its scope, deliverable, claim ids, invariants,
proof surface, seams, and out-of-scope notes. A batch is valid only when tightly
coupled tasks share one worker without hiding proof obligations.

On the packet path, prepare one task at a time. After the assigned `$agent-run`
leaf is clean and before its worker handoff, invoke the compiler from the
physical 3000 unit directory. A v2 packet consumes the exact pushed 4000
Closure carrier; a retained v1 packet has no Closure carriage:

```bash
(cd runs/3000/<unit-id> && groundwork build Tn --closure-sha <4000-Closure-governing-SHA>)
# Retained v1 packet only:
(cd runs/3000/<unit-id> && groundwork build Tn)
```

Require the canonical JSON success result, verify its task id and digest path
against the packet and its tree digest against the generated bytes, and Post
the generated bootstrap, test and proof skeletons, and digest record to the
leaf before Spawn. A typed
`GROUNDWORK_UNRUNNABLE` or output conflict records the exact obstruction and
stops the act. It does not commission environment preparation, leaf
restoration, packet repair, or 3010 work. No refusal is green, and this manager
never provisions around one.

Each worker's `$agent-run` docket adds: unit id, exact active branch, contract
Lock, 4000 Closure id and governing SHA, selected carrier reference, assigned
task ids, mapped claim
ids with polarity and oracle refs, assigned collapser application refs,
applicable Moon targets and execution-profile refs, existing
development-environment capability and receipt or exact receipt path to
produce, current diff context when needed, goal identifiers, prior
proof/review paths as evidence only, allowed surfaces, and explicit instructions to use
`$search` for discovery, to execute every assigned application through its
named door regardless of language or surface class, and to deliver witnesses
as docketed artifacts. When a proof surface uses S3-compatible object
storage, the docket also carries its `$tooling-object-storage` class, scope
lifetime, and cleanup-witness path without restating credential posture.

For a packet task, replace carrier excerpts with the packet-generated bootstrap
path, generated skeleton paths, digest record, exact Groundwork result, and
allowed product target paths. The bootstrap is the worker's task world. The
worker never receives mutation authority over the packet or generated tree and
never invokes Groundwork.

## Implementation Worker Contract

Workers use the logical `agent-code` profile under `gpt-5.6-terra` at
`max` through `$agent-run`. A worker must: own its child goal; enumerate the
full required surface before editing; implement the behavior and **author the
apparatus as a co-equal deliverable** — rigs, seeds, fixtures, tests at the
contract-named witness paths; wire applicable built or refactored surfaces into
Moon unless the locked contract records the governing exception; run each Moon
target through one coherent profile while retaining the resolved task and owner
command; create only bounded actor-owned non-Nix apparatus the work requires
and retain its receipt; preserve the Linux x86_64 server domain; run every
assigned collapser application with the exact identity and within the locked
fidelity envelope; elicit the wanted events and force the commissioned
exclusions within their budgets; and return the standard `$agent-run` Passback
plus the environment receipt, Moon targets and profiles, application refs,
identities exercised, control and decisive outputs, oracle verdicts, and
witness paths. A worker must not: invoke Nix directly or through another task,
use `sudo moon`, share writable state across actors, substitute an instrument
or configuration, broaden fidelity, qualify a new instrument, claim temporary
apparatus as fleet convergence, select or change a machine capability or
environment, invent ARM or multi-platform server builds, append events, write
proof or review documents, or touch product surfaces outside the assignment.
On a packet task it must also not invoke Groundwork or edit the work packet,
bootstrap, generated test/proof skeletons, or digest record. It implements
product behavior only in its allowed target paths and returns witnesses to the
orchestrator; generated skeletons are immutable traversal guides, not authored
contract truth.

A worker passback and the manager's task-proof document are validation
candidates, not a 4030 disposition. The manager may inspect them to decide
whether an already-admitted 4030 pass can consume the candidate. A readiness
failure stops with evidence; it does not commission validation or repair.

## Task Proof Documents

After a worker passback, independently inspect enough source and witness
evidence to judge readiness. If it is not ready, retain the exact gap and stop;
the observation does not commission a fix assignment.
On the packet path, re-run Groundwork `build Tn` as the orchestrator before
writing proof, using the same discriminator-correct command and exact
`--closure-sha` required by the initial v2 build. Any digest change, Closure
carriage change, or refusal means the generated task world drifted and cannot
advance. Require exact-set equality between every declared waypoint dimension
and observed traversal in both directions, while preserving the semantic
remainder for 4030.

For each task or validated batch, write
`runs/4000/<unit-id>/proofs/<task-id>.md` (scaffold with
`assurance-runs-validator init review --unit-id <unit-id> --stage 4020 --actor
agent-manager --dir runs/4000/<unit-id>/proofs --out <task-id>.md`): the
locked requirement; the Groundwork result, generated-tree digest, and waypoint
set comparison when applicable; per-claim rows (claim id, polarity, witness
path, exact command, result); and per-application rows (stable ref, exact
identity, applicability and fidelity used, control provenance, command, oracle
verdict, witness, gate result, residual), plus implementation evidence as
file:line references covering every path the task names or implies, deviations,
and limits. Broad claims such as "implemented", "tests pass", a bare tool name,
or a complete waypoint set are invalid without per-claim witnesses and
per-application disposition. Commit the worker's changes plus the proof
document, append a `proof` event, and append the claim transitions
(`commissioned -> collapsed`, with `witness_ref` and rung) to `claims.jsonl`.

## Validation Rounds And Responses

4030 runs only when `$jump-gate` admits the next block-4000
`implementation-validation` round against an exact pushed candidate. Each
task, batch, or final-composition pass consumes one round from the same 4000
Closure allowance. No 3020/3030 ordinal enters this account.

Run `$4030-unit-task-validation` through `$agent-run` using the logical
`agent-validation` profile under `gpt-5.6-sol` at `max`. Its closed docket
names unit and active branch, contract Lock, Closure id and governing SHA,
round ordinal, exact candidate SHA, selected carrier, task-linked acceptance
rows and subject depths, task and claim ids, collapser applications, proof
document, packet bootstrap/digest bindings when applicable, accepted
uncertainty, and explicit exclusions.

The validator writes
`runs/4000/<unit-id>/reviews/4030-<scope>-<round>.md` plus one Closure event.
The manager verifies the bindings and applies the disposition without
reinterpretation. It classifies every ordinary finding as inside, outside, or
Closure insufficiency before selecting any response. At most one
implementation response to admitted inside findings belongs to that round.
Outside findings and insufficiency arguments are retained without blocking or
work authority.

`FAIL`, `BLOCKED`, a stale review, or a missing review stops the candidate. It
does not commission repair, return the unchanged candidate to a validator, or
create a fresh 4030 pass. A response may create a new candidate only when it
was already admitted by the current round; another validation pass requires a
new Post and remaining active-Closure admission or an express human request.

Final composition validation is another 4030 round, not a free terminal pass.
Invoke it only if capacity and commission remain. It checks that the task chain
composes into the locked outcome and all 4000 acceptance rows, later work did
not invalidate credited proof, every claim disposition is honest, and every
collapser application has a current reproduced disposition. At exhaustion,
append the exhausted event and return exact state to the human; do not run a
final pass by reflex.

## Development And Materialization Boundary

This 4000 unit delivers a runnable artifact, object, or service and proves the
locked behavior at its development boundary. Workers may use ordinary commands
already available in the execution environment and bounded actor-owned non-Nix
apparatus to compile, run, test, debug, and assure it. Follow
`$development-environment`: the assigned workspace actor owns the checkout and
every mutable toolchain/cache root, temporary apparatus leaves a receipt, and
server work stays Linux x86_64. Moon may orchestrate those tasks where the
surface is wired. Follow `$moon-task-graph` and retain the target, resolved
task, underlying command, and execution profile. Moon is the native development
deployment surface, not durable machine or staging/production authority.

Do not invoke Nix, directly or through Moon, or create, modify, accept, or
require any Nix file, development shell, flake output, package, host profile,
derivation, closure, cache publication, service realization, activation,
staging/production deployment, durable machine-membership change, or Nix-based
completion evidence in 4000. A wired native Moon development-deploy task may
place the exact artifact on an already commissioned development destination
for assembled proof while preserving its capability, packages, credentials,
and service boundary. Running an ordinary installed command is not Nix work
and does not authorize capability/environment selection. If a durable machine
capability is missing, record it as a possible later 8000 classification and
stop the affected act. It creates no 8000 unit, and no future 8000 disposition
can become a predecessor that Build must satisfy.

## Unit Terminal State

The exact active 4000 candidate is terminal only when its Closure is satisfied:
every acceptance row has credited evidence; every required locked task has a
current proof and applicable 4030 PASS; commissioned final-composition
validation, when named by the acceptance set, is PASS;
every commissioned claim row is `collapsed` (or `residual` by explicit
contract lineage); every commissioned collapser application has a latest
4030-validated disposition carrying its exact identity, fidelity, oracle,
witness, gate result, and residual; every packet waypoint has exact-set
traversal with its semantic remainder still independently judged; targeted
clippy evidence exists for every affected Rust package; Groundwork `check` is
clean for a packet unit; `assurance-runs-validator check
runs/4000/<unit-id>` reports no unwaived blockers; the exact current branch is
pushed; no admitted inside defect remains; the Closure event fold is within its
limit; every Promise disposition is honest; and the terminal event names the
contract Lock, Closure id, governing SHA, and candidate SHA.

The terminal event carries an **assurance disposition**, derived without
inventing authority:

- the contract's declared tolerance tier;
- every collapser application ref and its delivered or residual disposition;
- the exact guarantees, instrument identities, oracle refs, witnesses,
  fidelity limits, gate placements, and named residue a later independent
  review would need to preserve; and
- whether the human has separately and explicitly invoked a 7000 campaign.

This disposition is a handoff, not another completion condition. Build reaches
terminal state whether the next human action is ordinary promotion or
deployment, no independent review, or a separately invoked 7000 campaign. A
Tier 3 label, retained evidence, blast-radius observation, reviewer
recommendation, or retained obligation never auto-invokes 7000 and never
permits this manager to infer the human decision.

Then stop: **promotion and independent re-assurance invocation are not
yours.** The
distillation commit to `main`, any integration PR, deployment, and any 7000
campaign happen only through their separately authorized paths, and nothing is
ever agent-merged. Terminal state commissions none of them.

## Mandatory Fail-Closed Completion Gate

Gate id: `mandatory_fail_closed_completion_gate_v1`, exactly as root policy
states it: the packet's `fail_closed_completion_exceptions` field or the legacy
contract section governs. With `none`, any fail-closed, stubbed, placeholder,
`unimplemented`, `unsupported`, `available: false`, catalog-without-handler,
mock/fake/synthetic, rejection-only, compile-only, or silently-wrong behavior in
the changed or claimed surface blocks completion -- silently-wrong (valid
input, incorrect/frozen/hardcoded result) is the most severe class because it
presents as done. Baseline-missing behavior named as target is implementation
scope, never grounds to refuse mutation.
