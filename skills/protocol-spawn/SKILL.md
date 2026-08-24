---
name: protocol-spawn
description: Use when an active commission already admits delegation of one bounded docket to a child at a pinned branch, before any blocking wait on that child.
node: protocol-spawn
class: skill
edges:
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
metadata:
  short-description: Delegate a bounded docket at a pinned branch
  class: verb
  edges:
    guarded-by: [protocol-match, protocol-predeclare, protocol-resolve]
    closed-by: [protocol-passback]
    follows: [protocol-cut]
    cites: [agent-run, runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Spawn

Delegate a bounded docket already admitted by the active commission at a
pinned branch. The act completes only
once the child's lineage is durable, and that durability precedes any
blocking wait.

Repository sub-agent execution realizes this act through `$agent-run`; this
node defines Spawn semantics, not a second handoff procedure.

Refusal: no commissioning authority; no branch or bounded docket; lineage not
durable.

A finding, route, failure, retry suggestion, available slot, or parent result
does not commission Spawn.

Guarded by: Match, Predeclare, Resolve. Closed by: Passback.

Index: $git-policy — The Work Protocol Vocabulary.
