---
name: protocol-cut
description: Use when an already-commissioned branch identity must be created from an exact pinned parent SHA. A finding, edit, result, route, or failure never commissions the Cut.
node: protocol-cut
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-mint
    provenance: declared
metadata:
  short-description: Branch from a pinned parent SHA
  class: verb
  edges:
    follows: [protocol-mint]
    precedes: [protocol-spawn]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Cut

Create an already-commissioned branch from a parent taken at a pinned SHA,
never from a branch name standing in for one.

The branch identity and because-edge must already have been Minted inside the
active commission. Checkout and switching inside the assigned worktree need no
additional approval once those facts hold. A wrong physical worktree, stale
parent, edit, finding, route, result, or failed branch is a blocker to report,
never grounds to create another.

Refusal: branch not commissioned; identity not Minted; parent unpinned or
drifted.

Index: `$git-policy` - The Work Protocol Vocabulary.
