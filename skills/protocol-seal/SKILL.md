---
name: protocol-seal
description: Use when an already-commissioned run, log, or unit is ready for terminal closure and every obligation is collapsed or declared. A terminal disposition never commissions promotion or follow-on work.
node: protocol-seal
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
    target: protocol-declare
    provenance: declared
  - type: cites
    target: protocol-gate
    provenance: declared
  - type: cites
    target: protocol-verify
    provenance: declared
metadata:
  short-description: Terminally close a run or unit
  class: verb
  edges:
    guarded-by: [protocol-verify, protocol-gate, protocol-attack, protocol-completion-gate]
    follows: [protocol-declare]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Seal

Terminally close a run, log, or unit under exactly one disposition —
promote, discard, or park.

Seal records that disposition only. It never creates a Promotion, later lane,
review, repair, retry, successor, or other act.

Refusal: obligations neither collapsed nor Declared.

Guarded by: Verify, Gate, Attack, and the Completion Gate.
Follows: Declare.

Index: `$git-policy` - The Work Protocol Vocabulary.
