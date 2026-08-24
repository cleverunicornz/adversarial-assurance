---
name: protocol-post
description: Use when an already-commissioned change must be committed and pushed after comparing the current branch with the intended target. Post records Git state and never commissions another act.
node: protocol-post
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-match
    provenance: declared
metadata:
  short-description: Commit and push to the intended target
  class: verb
  edges:
    guarded-by: [protocol-match]
    precedes: [protocol-land]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Post

Commit the work and push it to the target you intended, not to whatever
target the working copy happens to be on.

Post makes already-authorized bytes durable. A commit or push never creates a
pass, review, repair, retry, branch, successor, lane, or other work.

Refusal: checked-out state differs from intended target.

Guarded by: Match (mandatory).

Mechanical form: `scripts/post`.

Index: `$git-policy` - The Work Protocol Vocabulary.
