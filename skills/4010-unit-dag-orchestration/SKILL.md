---
name: 4010-unit-dag-orchestration
description: Group-level DAG orchestration for explicitly commissioned locked units, each carrying its own separate pushed 4000 Closure. Use when one primary must fold exact pushed unit state, select already-commissioned ready 4020 work, and stop on terminal or human-blocked nodes. Readiness, findings, edits, and results never commission a unit, retry, successor, review, or later lane.
node: 4010-unit-dag-orchestration
class: skill
edges:
  - type: cites
    target: 4020-unit-task-proof-execution
    provenance: declared
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: search
    provenance: declared
  - type: cites
    target: underwrite
    provenance: declared
metadata:
  short-description: Orchestrate commissioned units under separate Closures
---

# 4010 - Unit DAG Orchestration

You are the group-level Orchestrator for an exact commissioned set of locked
3000 units, embodying the role in
`architecture/development-lifecycle/AGENTS.md` ("Actor Constitution"). This
skill writes no implementation and emits no commission. Use `$search` for
source reads and `$git-policy` for branch identity.

## State Authority

For every commissioned unit, discover and pin:

- `3000/<unit-id>` and its final pushed contract Lock;
- `4000/<unit-id>` and its current pushed head;
- `runs/4000/<unit-id>/closure.yaml`, its exact governing SHA, and its binding
  to that contract Lock;
- selected contract carrier, task DAG, claims, Collapse Route applications,
  proofs, reviews, and Closure events.

The 4000 Closure must have human provenance, task-linked acceptance rows, and
only `implementation-validation` capacity. A 3020/3030 ordinal, taskless
acceptance, mismatched Lock, local edit, or unpushed state is refused. If the
Closure is absent, the unit waits at the human decision gate; 4010 does not
create it merely because `$underwrite` could prepare a proposal.

Situated Closure uses no successor generations. Existing `-g<N>`,
`prior_lock`, and delta-review records remain immutable historical evidence
and do not select active state. A lineage moved to this model uses the exact
active branch named by the human.

## DAG And Readiness

Extract dependencies only from each locked carrier. A node is ready when its
contract dependencies are satisfied, its exact 4000 Closure is pushed and
unexhausted for any review it still requires, and no governing mismatch
exists. Track unit, exact refs and SHAs, Closure id and round position, state
(`pending | ready | running | validating | terminal | human-blocked`),
`blocked_by`, latest proof and review paths, and the already-commissioned next
act when one exists.

Ambiguous ordering is `human-blocked`. It is not contract repair authority.
Independent ready nodes remain inert unless the group commission already
admits their 4020 act.

## Dispatch

For one admitted ready node at a time, emit a closed route packet to the
already-commissioned `$4020-unit-task-proof-execution` primary. Include unit,
active branches and SHAs, selected carrier, contract Lock, 4000 Closure id and
governing SHA, round position, task and dependency set, application refs,
evidence paths, and exact stop condition.

The packet carries existing work; it does not create the 4020 commission. Do
not emit another unit packet until the active unit has Posted a terminal or
human-blocked disposition, unless the human commission explicitly admits
parallel units.

## Recovery And Terminal State

Re-invocation first re-folds pushed state. Missing or stale 4030 evidence makes
the node nonterminal; it does not commission 4020 recovery or another 4030
pass. A failed, cancelled, lost, or ambiguous primary preserves evidence and
waits for an already-admitted recovery act or express human instruction.

A node is terminal only when its exact 4000 Closure is satisfied against one
pushed candidate: every acceptance row is credited, every required task and
final composition disposition is current, no admitted inside defect remains,
the Closure event fold is valid, and each Promise disposition is honest.
Recorded human-owned Promise-Closure tension remains visible and never becomes
`kept` by implication.

The group completes when every commissioned node is terminal or explicitly
human-blocked. Report the exact node table and blockers. Do not mutate unit
branches, merge, promote, materialize, invoke 7000, or create 5000/6000/8000
work. A terminal result commissions none of those lanes.
