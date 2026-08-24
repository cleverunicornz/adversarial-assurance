---
name: protocol-commission
description: Use when binding an explicit human-requested obligation or an act already admitted by a human-agreed Closure to an owner, judge, and finite envelope; never infer commission from workflow state.
node: protocol-commission
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-promote
    provenance: declared
metadata:
  short-description: Bind explicit work with its judge and envelope
---

# Commission

Bind a typed obligation to an owner, naming the judge that resolves it and the
finite envelope it holds within.

Work-bearing provenance is explicit: a human request, or a specific act
already admitted by a human-agreed active Closure. A finding, route, event,
permission, refusal, edit, commit, push, merge, result, or available budget is
never a commission.

Refusal: no resolving judge; no finite envelope; self-grounding or inferred
commission.

Contrast: `$protocol-promote` converts non-binding material through a human
gate. Commission is the intake direction of that boundary.

Index: `$git-policy` - The Work Protocol Vocabulary.
