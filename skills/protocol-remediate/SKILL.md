---
name: protocol-remediate
description: Use when an already-authorized change fixes evidence or hygiene without changing contract meaning. Remediation never commissions validation, retry, repair, or another workflow.
node: protocol-remediate
class: skill
edges:
  - type: cites
    target: git-policy
    provenance: declared
  - type: cites
    target: protocol-repair
    provenance: declared
metadata:
  short-description: Fix evidence or hygiene under existing authority
---

# Remediate

Fix evidence or hygiene and leave contract meaning exactly where it was, only
inside the active commission or one exact human instruction. Post changed
state under `$git-policy` before it is consumed.

Remediation is not a commission. It creates no validation pass, retry, repair,
route, successor, or later lane. Any review requires remaining active-Closure
admission or an express human review request.

Refusal: contract meaning changes, or no existing authority for the change.

Contrast: `$protocol-repair`.

Index: `$git-policy` - The Work Protocol Vocabulary.
