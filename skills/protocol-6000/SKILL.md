---
name: protocol-6000
description: Use when an expressly commissioned change is documentation itself. The unit receives an initial independent validation and up to two Disposition-to-Disposition corrective loops before an unresolved result returns to the human; its merge never creates later work.
node: protocol-6000
class: skill
edges:
  - type: cites
    target: 6020-document-validation
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-trunk-and-leaf
    provenance: declared
metadata:
  short-description: Author and independently validate documentation
---

# Lane 6000 - Documentation

One lane holds two possible, independently commissioned positions: Promise
authoring before contract work, and reconciliation after landed reality. The
position describes admissible ordering; it is not an automatic edge.

The express unit commission admits an initial independent
`$6020-document-validation` invocation after the Implementer Posts the exact
candidate. Its verdict establishes the starting Disposition and consumes no
corrective loop. `PASS` permits the merge branch to be presented at the human
gate.

A retained `FAIL` returns exact findings through the Orchestrator to the
Implementer. While fewer than two corrective loops have completed, the same
unit commission admits repair, a new exact Posted candidate, and a fresh 6020
invocation. The fresh retained Disposition completes one loop. Thus the full
default envelope is `V0 -> repair -> V1 -> repair -> V2`: at most three
Validator invocations and two repairs.

`PASS` stops the loop early. A `FAIL` at `V2` exhausts the default allowance
and stops at the human decision boundary; only the human may extend it. Each
6020 invocation owns one verdict and exits. A verdict, finding, remaining
count, repair, Post, or available actor never self-commissions another act.

Refusal: promise-plane prose that describes present status instead of future
direction, or register prose that claims future promise authority. Present
status belongs to the risk register. The refusal creates no unit.

No 6000 edit, result, Post, or merge commissions 3000, 4000, 5000, another
6000 unit, or work outside the current unit's admitted correction envelope.

Index: `$git-policy` - Lanes.
