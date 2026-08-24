---
name: underwrite
description: Prepare one block-local Situated Closure for human decision. Use after 3000 Promise preflight to bound only contract authoring and 3020/3030, or after the contract Lock exposes the complete task graph to bound only implementation and 4030. Produces no campaign-wide grade or shared budget, never predicts the later block, and never commissions work by itself.
node: underwrite
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: jump-gate
    provenance: declared
metadata:
  short-description: Prepare one human-agreed block Closure
---

# Underwrite

This skill helps an orchestrator prepare one `assurance-closure/v1` object. The
semantic owner is
`architecture/development-lifecycle/closure.md`; this skill supplies only the
block-local elicitation and recording practice. It never decides Closure for
the human.

## Block Boundary

- **3000:** prepare Closure only after every Promise source resolves and before
  contract decomposition or review. It names terminal contract evidence,
  accepted proof depth and uncertainty, and one finite allowance whose counted
  acts are exactly `contract-validation` and `contract-counterfactual`.
- **4000:** prepare a separate Closure only after the final pushed contract
  Lock exposes the complete task graph. It names that exact Lock SHA, maps
  acceptance rows to tasks, and carries one finite allowance whose only
  counted act is `implementation-validation`.

Refuse a combined Closure, a 3000 estimate of 4000 work, a cross-block ordinal,
or any attempt to spend one block's allowance in the other.

## Elicitation

For the selected block, propose the smallest closed object that records:

- the exact architecture Promise paths;
- positive acceptance rows with terminal witness paths and explicit
  `subject_depth`;
- the accepted uncertainty;
- the finite adversarial-round limit; and
- human decision provenance.

Every unnamed proposition and every supporting means below the named subject
depth is an accepted premise, not an assurance subject. Do not add a compiler,
runner, Cargo invocation, network, cache, host, user, filesystem, dependency,
harness, or assembler merely because a terminal witness uses it.

Do not ask for a universal risk grade, priority label, clock label, fix-loop
budget, or spike budget. Those campaign-wide inputs are legacy Underwriting,
not Situated Closure.

## Human Decision And Post

The agent may propose and scribe; the human agrees or changes the boundary in
live conversation. Record `decision.principal: human`, the decision receipt,
and its time. Then commit and push the governing object under `$git-policy`
before the block consumes it:

- block 3000 Closure lives in the v2 packet's `phase: closure` form; and
- block 4000 Closure lives at `runs/4000/<unit-id>/closure.yaml`.

The Post makes the agreed bytes operative. It does not commission a review,
another lane, a successor, or proof of the means used to produce evidence.
Human-directed later changes use the same direct commit-and-push rule. Git
history supplies lineage; no self-referential Lock, predecessor object, or
successor generation is authored.

## Relationship To The Round Gate

`$jump-gate` folds only events already commissioned inside the selected
block. Agreement to the Closure admits its named finite loop; this skill does
not open passes, classify findings, apply responses, or decide satisfaction.

Existing `Underwriting` records retain their historical meaning. They are not
converted, backfilled, or interpreted as Closure.
