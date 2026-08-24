---
name: protocol-mint
description: Use when an active commission requires a new identity such as a unit id, run ref, or key and its because-edge must be recorded. Mint never turns a finding, route, result, or edit into work.
node: protocol-mint
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-resolve
    provenance: declared
metadata:
  short-description: Create an identity with its because-edge
  class: verb
  edges:
    guarded-by: [protocol-resolve]
    precedes: [protocol-cut]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Mint

Create an identity required by the active commission - a unit id, run ref, or
key - and carry with it the edge naming what it came from. Mint records
identity; it does not commission the object or any act that may use it.

Refusal: no active commission or no provenance edge.

Guarded by: Resolve.

Index: `$git-policy` - The Work Protocol Vocabulary.
