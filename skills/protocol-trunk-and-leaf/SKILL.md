---
name: protocol-trunk-and-leaf
description: Use when an active commission requires opening a unit trunk, creating or landing a leaf, or refreshing delegated leaf work it already admits. Owns branch topology only; staleness, governing edits, findings, and branch events never create another unit, leaf, review, or successor.
node: protocol-trunk-and-leaf
class: skill
edges:
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
metadata:
  short-description: One trunk and bounded delegated leaves
---

# Trunk And Leaf

The procedure every unit runs, in every lane. The orchestrator cuts one trunk
per unit and is its only writer. A non-fast-forward rejection on that trunk
means a second writer exists and stops mutation. The trunk is the unit's
last-known-good viewport and the only absorber of newer `main`.

Every delegated mutation follows `$agent-run`. This node owns only the branch
topology supplied to that handoff: the primary cuts a child leaf from the
pinned trunk SHA, remains its Git writer, and lands it only when the lane's
already-commissioned gate accepts.

Refusal: a leaf never absorbs `main` or the trunk. Stale delegated leaf work is
refreshed only when the active commission admits another handoff: cut a new
leaf from the current trunk, merge the old leaf into it, and preserve the old
ref as evidence. Staleness, failure, or a finding does not commission that
refresh.

Post-Lock contract or Closure edits do not create branch generations. Under an
exact human instruction, the active lineage's sole writer commits and pushes
the changed governing bytes on the active 4000 branch before work resumes. Git
ancestry supplies provenance; the edit creates no 3000 re-entry, successor,
leaf, or review.

Where a unit exits and which way merges may flow are `$git-policy`'s own. A
unit result or merge never commissions a later lane.

Index: `$git-policy` - Lanes.
