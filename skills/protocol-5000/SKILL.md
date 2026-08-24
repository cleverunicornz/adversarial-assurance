---
name: protocol-5000
description: Use when an expressly commissioned change is to shared agent skills. The unit receives an initial independent validation and up to two Disposition-to-Disposition corrective loops before an unresolved result returns to the human; no earlier or later lane result creates it.
node: protocol-5000
class: skill
edges:
  - type: cites
    target: 5020-skill-validation
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-trunk-and-leaf
    provenance: declared
metadata:
  short-description: Change and independently validate shared skills
---

# Lane 5000 - Skills

The index of every shared-skill mutation. A 5000 unit begins only from its
express commission and pinned parent SHA. It is not created by a behavior,
documentation merge, finding, route, or validation result.

The express unit commission admits an initial independent
`$5020-skill-validation` invocation after the Implementer Posts the exact
candidate. Its verdict establishes the starting Disposition and consumes no
corrective loop. `PASS` permits the merge branch to be presented at the human
gate.

A retained `FAIL` returns exact findings through the Orchestrator to the
Implementer. While fewer than two corrective loops have completed, the same
unit commission admits repair, a new exact Posted candidate, and a fresh 5020
invocation. The fresh retained Disposition completes one loop. Thus the full
default envelope is `V0 -> repair -> V1 -> repair -> V2`: at most three
Validator invocations and two repairs.

`PASS` stops the loop early. A `FAIL` at `V2` exhausts the default allowance
and stops at the human decision boundary; only the human may extend it. Each
5020 invocation owns one verdict and exits. A verdict, finding, remaining
count, repair, Post, or available actor never self-commissions another act.

When a skill describes newly delivered behavior, that behavior must already
exist on landed `main`. A standalone correction to the skill system may begin
from its own human commission.

No 5000 result commissions 4000, 6000, another 5000 unit, or work outside the
current unit's admitted correction envelope.

Index: `$git-policy` - Lanes.
