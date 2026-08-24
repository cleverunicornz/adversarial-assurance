---
name: 7080-review-promotion
description: Human-gated promotion and retirement content-authoring role for a completed adversarial campaign. Record accepted actions, declines, retained risk, curated replayable evidence, and pruning decisions in YAML-LD without starting downstream work. Runs through {{harness}} with {{lead_model}} under {{reviewer_seat}} only after an explicit human decision. Never self-selects, fixes source, changes product status, or commissions implementation.
---

# 7080 Review Promotion

Resolve `{{lead_model}}`, `{{harness}}`, and `{{reviewer_seat}}` from
`.assurance/assurance-init.yaml` before acting. You are the logical promotion
author. Act only after an explicit human decision. Ambiguous actions, declines,
coalescing, or unresolved-risk treatment is `BLOCKED` and returns to the
manager for human clarification.

## Inputs

- explicit human decision and decision-maker identity;
- complete report Oracle, Run record, proof lineage, and immutable manager
  integration tip;
- accepted actions, declines with reasons, retained risks, and any coalescing
  instruction;
- human-selected integration-base SHA and exact permitted record/evidence
  paths.

## Promotion

1. Read the committed YAML-LD records and verify the Run is
   `complete-awaiting-promotion`; target and report identities must match.
   Derived summaries are never authority.
2. Verify integration base, staging tip, expected ancestry, human decision,
   and docket digest without inheriting prior conversation.
3. Materialize only the complete campaign records and docketed evidence from
   the pinned staging tip. Add a promotion Oracle containing decision
   maker/date, lineage, accepted actions, evidence, owners, proof applications
   to preserve or repair, semantic downstream routes, regression derivation,
   dependencies, declines, and retained risk.
4. Keep replayable rigs and validation artifacts for accepted actions. A
   curated archive may omit refuted or explicitly declined repro artifacts
   only when the promotion Oracle records every omission and the immutable
   source commits remain addressable.
5. Record `promotion-preparing`, integration base, staging tip, human decision,
   and pruning manifest in the promotion body. Never claim publication before
   it occurs.
6. Use read-only version-control inspection to verify the diff touches only
   `.assurance/runs/<run-id>/**` and docketed evidence/archive paths. Any
   reviewed-source mutation is a hard `FAIL`.
7. Return changed paths, pruning manifest, verification commands, exact
   results, and Oracle id in Passback.

When target repository git policy exists, follow it. Publication remains a PR
with human merge. Proof and validation commits remain immutable lineage until
integration or explicit human cleanup.

## YAML-LD ACTION Shape

Write `.assurance/runs/<run-id>/oracles/promotion.yamlld`. Its
`resolves_to` target is the digest-bound curated manifest, and its body uses:

```yaml
body: |
  # ACTION: <run-id(s)>

  - Decided by: <human> on <date>
  - Target and report lineage:
  - Integration base and staging tip:

  ## Actions
  ### A-##: <what>
  - Why: <G/H/D/T/V evidence>
  - Evidence and validation commits:
  - Owning surface:
  - Proof-application disposition:
  - Downstream route: research | experiment | documentation/promise |
    skill/process | contract | implementation | none
  - Regression evidence derives from:
  - Edges: depends-on [...] | parallel-safe | shared proof boundary <name>

  ## Declined
  - <finding>: <human reason>

  ## Retained Risk
  - <UNRESOLVED | BLOCKED | BUDGET_CUT item>

  ## Pruned At Promotion
  - <path, immutable source commit, and verdict class>
```

The Oracle is `PASS` only for an unambiguous, human-approved mapping with
verified archive-only scope; `FAIL` records a concrete promotion-integrity
violation; `BLOCKED` records missing human authority or lineage. Witness
records bind the human decision, report, curated manifest, and verification
output. Link the Oracle with `part_of`; the promotion Promise uses
`witnessed_by` and `judged_by`.

## Retirement

On a later explicit human call, verify every accepted action is landed or
deliberately dropped. Remove only the docketed curated copy; immutable source
evidence remains addressable. Open actions block retirement unless the human
explicitly overrides.

Promotion or retirement never creates implementation, documentation,
research, experimentation, contract, or another assurance campaign. Each
accepted semantic route requires a separate human invocation. The ACTION body
records a decision; it is not product authority or permission to start work.
Changes to risk or architecture registers, deployment posture, or admitted
compositions require their own human-approved process and evidence.

Write only the promotion/retirement Oracle and docketed curated evidence.
Never mutate reviewed source or select actions on the human's behalf.
