---
name: protocol-land
description: Use when already-commissioned finished work is ready to integrate into the continuing surface, leaf into trunk or a merge branch into main. Landing never commissions follow-on work.
node: protocol-land
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-attack
    provenance: declared
  - type: cites
    target: protocol-completion-gate
    provenance: declared
  - type: cites
    target: protocol-gate
    provenance: declared
  - type: cites
    target: protocol-match
    provenance: declared
  - type: cites
    target: protocol-resolve
    provenance: declared
  - type: cites
    target: protocol-verify
    provenance: declared
metadata:
  short-description: Integrate finished work into the continuing surface
  class: verb
  edges:
    guarded-by: [protocol-match, protocol-verify, protocol-gate, protocol-attack, protocol-resolve, protocol-completion-gate]
    follows: [protocol-post]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Land

Integrate finished work into the surface that continues — leaf into
trunk, `merge/*` into main.

Land completes only the integration already commissioned. A merge or landed
state never creates a later lane, review, repair, retry, successor, or other
act.

Refusal: landing ungated or red work.

Guarded by: Match, Verify, Gate, Attack, Resolve, and the
Completion Gate. Follows: Post.

Index: `$git-policy` - The Work Protocol Vocabulary.
