---
name: protocol-gate
description: Use when a named check stands between an already-commissioned act and its next admitted act, or when a gate reports a disposition that must not manufacture recovery.
node: protocol-gate
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-predeclare
    provenance: declared
metadata:
  short-description: Decide one named check without creating the next act
---

# Gate

A named check that must accept before an already-commissioned next act may run.
Its verdict values are its own and are never protocol words. The human gate is
a Gate whose criterion is a human decision.

## Validation Authority

Producer output is a candidate, never accepted evidence by itself. Only the
owning validator emits the commissioned validation disposition, bound to the
exact candidate, pass identity, criteria, and evidence identity. An
orchestrator may verify and apply those bindings; it may not reinterpret,
suppress, downgrade, replace, round, or adjudicate the verdict.

An accepting disposition enables only the next act already present in the
commission. A non-accepting, missing, stale, conflicting, or mismatched record
stops that act and preserves evidence. It does not commission repair,
revalidation, a replacement validator, a route, or another lane. Any later act
requires remaining active-Closure admission or an express human commission.

Refusal: the guarded act, until the named check accepts. The refusal itself
creates no work.

Guarded by: `$protocol-predeclare`.

Index: `$git-policy` - The Work Protocol Vocabulary.
