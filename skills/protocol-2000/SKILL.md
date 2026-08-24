---
name: protocol-2000
description: Use when the open question is empirical and only a rig can answer it — feasibility, characterization before a refactor, oracle genesis, or instrument qualification.
metadata:
  short-description: The spike lane — proof or refutation by rig
  class: noun
  edges:
    guarded-by: [protocol-trunk-and-leaf]
    follows: [protocol-1000]
    precedes: [protocol-6000]
    routes-to: [protocol-6000, protocol-3000]
    cites: [runs/1000/1785166581381-d3c4-work-protocol-epoch]
    indexed-in: [git-policy]
---

# Lane 2000 — Spikes

The lane that answers by building. Sacrificial code stays in quarantine, the
criteria are fixed before the run that answers them, and the unit ends in a
verdict rather than a deliverable. A spike moves no documentation and never
reaches the default branch.

Refusal: settled direction needing no proof does not enter — it routes
onward to the lane that consumes it.

A spike may invoke or mutate Nix only when its charter records the human's
exact commission for one Nix question whose answer requires that rig and the
rig runs outside the development environment. That permission is never
inferred. The result stays quarantined knowledge and may not publish a fleet
output, promote a release, activate a generation, or create durable host state.

Flow: reached from 1000 when the answer needs execution; because its answer
changes what is true, 6000 records that before 3000 opens.

Index: $git-policy — Lanes.
