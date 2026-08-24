---
name: protocol-passback
description: Use when about to close a spawned delegate and return its result to the caller, or when tempted to hand back a narrative instead.
node: protocol-passback
class: skill
edges:
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
metadata:
  short-description: Close a spawn with a typed result
  class: verb
  edges:
    closes: [protocol-spawn]
    cites: [agent-run, runs/1000/1785193617565-14df-verb-canon-sweep]
    indexed-in: [git-policy]
---

# Passback

Close a Spawn with a typed result — witness refs, exact commands, named
residue.

Repository sub-agent execution realizes this act through `$agent-run`; this
node defines Passback semantics, not a second handoff procedure.

When a Passback carries a validation disposition, it also names the owning
validator and the candidate or source, attempt or pass, criteria or
specification, and evidence identities that bind the result. A producer,
caller, or manager cannot turn its own candidate or narrative into that
disposition.

Passback closes only the commissioned Spawn. COMPLETED, PASS, FAIL, BLOCKED,
residue, or a suggested next act creates no repair, retry, review, route,
delegate, lane, or other work.

Refusal: bare verdict; prose handoff.

Closes: Spawn.

Index: $git-policy — The Work Protocol Vocabulary.
