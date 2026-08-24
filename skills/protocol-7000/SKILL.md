---
name: protocol-7000
description: Use when a human commissions adversarial re-assurance — attacking advertised behavior, delivery proof, or instrument integrity on source that has already landed.
metadata:
  short-description: The review lane — human-invoked adversarial re-assurance
  class: noun
  edges:
    guarded-by: [protocol-trunk-and-leaf]
    conditionally-meets: [protocol-3000, protocol-4000]
    routes-to: [protocol-4000, protocol-5000, protocol-6000]
    cites: [runs/1000/1785166581381-d3c4-work-protocol-epoch]
    indexed-in: [git-policy]
---

# Lane 7000 — Review

The lane that tries to falsify what delivery claimed, reusing and attacking
proven instruments without becoming any unit's completion dependency. Its
product is replayable evidence and a verdict held at a human gate.

Refusal: fixing reviewed source inside this lane is refused — a reproduced
defect routes to the lane that owns the surface.

Flow: meets 3000 and 4000 conditionally, on human invocation, never as a
step either lane owes.

Index: $git-policy — Lanes.
