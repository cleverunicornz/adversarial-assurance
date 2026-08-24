---
name: protocol-match
description: Use when about to mutate anything and the actual state has been assumed rather than compared with the state you intend.
metadata:
  short-description: Actual equals intended before any mutation
  class: guard
  edges:
    guards: [protocol-post, protocol-spawn, protocol-land]
    cites: [runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Match

Actual state equals intended state, established by comparison before any
mutation.

Refusal: the bound verb, whenever actual differs from intended.

Guards: Post (mandatory), Spawn, Land.

Qualified instrument: `branch-match`, the promoted identity from pull
request 1924.

Index: $git-policy — The Work Protocol Vocabulary.
