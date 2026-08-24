---
name: jump-gate
description: Enforce one block-local Situated Closure by folding its exact events, admitting only block-correct rounds and responses already commissioned inside its finite allowance, and stopping without invented recovery at exhaustion. Use around 3020/3030 or 4030 passes; never share ordinals across blocks, expand Closure, or create another pass, lane, successor, route, or proof subject.
node: jump-gate
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: underwrite
    provenance: declared
metadata:
  short-description: Enforce one finite block-local Closure
---

# Jump Gate

The gate is the mechanical edge of one human-agreed Closure. Its semantic
owner is `architecture/development-lifecycle/closure.md`. It counts and stops;
it never judges a finding's merit, edits a governing artifact, or creates
work.

## Exact Position

Before opening a pass, resolve from pushed Git state:

- block, Closure id, and exact governing Closure commit SHA;
- exact candidate SHA;
- the block-specific counted acts and finite limit; and
- every prior Closure event bound to that same block, Closure id, and
  governing SHA.

Fold rounds numerically. A 3000 position includes only 3020
`contract-validation` and 3030 `contract-counterfactual` passes. A 4000
position includes only 4030 `implementation-validation` passes. A cross-block
act or inherited ordinal is refused rather than counted.

Each pass admitted by the active Closure or by an express human review
commission consumes one round, including PASS. One finding and many findings
in a pass cost the same round. Author responses,
implementation responses, schema checks, edits, commits, pushes, human
decisions, and findings themselves consume no round.

## Admission

Open a pass only when all of these are true:

- its kind belongs to the active block;
- its exact candidate is committed and pushed;
- the next ordinal is within the Closure limit; and
- the pass is either the next act already admitted by the active Closure loop
  or an express human review commission.

A prior result, finding, permission, route, event, Post, merge, or available
capacity is not a commission. Refuse a pass that lacks the final condition
even when capacity remains.

## Finding Classification

After the pass, the block orchestrator records each ordinary assurance finding
as exactly one of:

- **inside:** it contradicts a named Promise-to-acceptance relation and may
  select at most the one response already commissioned for this round;
- **outside:** it concerns an unnamed proposition or supporting means and is
  retained without blocking or work authority; or
- **insufficiency:** it demonstrates that the accepted evidence cannot
  establish a named Promise and is retained for the human without suspending
  or expanding Closure.

An adopted-protocol conformance BLOCK remains a separate platform-validity
gate. It cannot be used to certify the protocol, qualify its provider, expand
Closure, or manufacture a review.

## Close And Exhaust

Close the round with `pass`, `response-applied`, or `challenge-recorded` and
the exact response reference when one exists. If an admitted next pass remains
inside the finite loop, it may later be opened against its own pushed
candidate. Nothing about close creates that candidate or pass.

At the limit, append `round-exhausted` and stop. There is no automatic final
pass, repair, retry, route, successor, or edge ritual. Return the exact
candidate, evidence, findings, accepted uncertainty, and round history to the
human. The human may accept, hold, stop, edit Closure, or expressly request a
review. A Closure edit becomes operative only after commit and push under
`$git-policy`; the edit itself still does not invoke review.

## Legacy Boundary

Campaign-wide Underwriting, fix-loop tallies, shared spike budgets,
JUMP/HOLD verdicts, delta-review inheritance, and successor-generation
ordinals retain historical meaning only in artifacts created under that model.
They are not active Situated Closure mechanics and are never projected into a
3000 or 4000 account.
