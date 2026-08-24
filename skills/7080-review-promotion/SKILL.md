---
name: 7080-review-promotion
description: Human-gated promotion and retirement content-authoring role for a completed adversarial campaign. Record accepted actions, declines, retained risk, curated replayable evidence, and pruning decisions in YAML-LD without starting downstream work. Runs through {{harness}} with {{lead_model}} under {{reviewer_seat}} only after an explicit human decision. Never self-selects, fixes source, changes product status, or commissions implementation.
---

# 7080 Review Promotion

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{lead_model}}`, `{{harness}}`, and `{{reviewer_seat}}` from
`situation/assurance/assurance-init.yaml` before acting. You are the logical promotion
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
   maker/date, lineage, accepted actions, evidence, owners, proof applications,
   substrate proposal routes, regression derivation, dependencies, declines,
   and retained risk.
4. Keep replayable rigs and validation artifacts for accepted actions. A
   curated copy may omit refuted or explicitly declined repro artifacts only
   when the promotion Oracle records every omission and immutable source
   commits remain addressable.
5. Record `promotion-preparing`, integration base, staging tip, human decision,
   and pruning manifest in the promotion body. Never claim publication before
   it occurs.
6. Author complete substrate vertices only as proposal files in the promotion
   PR. Never auto-add them, edit existing base vertices, or rewrite earlier
   mount records/evidence.
7. Compare all pre-existing mount bytes before/after projection. Only the new
   promotion bundle may change inside the mount; outside it, only the exact
   proposed base files are allowed. Any reviewed-source mutation is `FAIL`.
8. Return proposal paths, promotion records, pruning manifest, exact
   verification commands/results, and Oracle id in Passback.

When target repository git policy exists, follow it. Publication remains a PR
with human merge. Proof and validation commits remain immutable lineage until
integration or explicit human cleanup.

## YAML-LD ACTION Shape

Write `situation/assurance/runs/<run-id>/oracles/promotion.yamlld`, resolving
to the digest-bound curated manifest.

The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/promotion.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
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
- Substrate proposal route: risk-vertex | plan-vertex |
  definition-or-architecture-vertex | reference-document | none
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

## Substrate Projection — Proposals Only

Every proposal cites the exact assurance graph-manifest SHA-256 and relevant
run-graph SHA-256, never a mutable directory as evidence identity. Use only
existing bedrock `source`, `path`, and `consumes` edges. These are complete
drafts for human review, not commissioned work.

Retained risk proposal:

```yaml
"@context": "urn:bedrock:context/v1"
"@id": "urn:bedrock:vertex/risk-<slug>"
"@type": "urn:bedrock:ontology/Risk"
label: "<risk title>"
statement: >
  <present-tense retained risk>; assurance graph manifest sha256
  <manifest-digest>; run graph sha256 <run-graph-digest>.
source: "urn:bedrock:path/situation/assurance/runs/<run-id>"
```

Accepted action proposal:

```yaml
"@context": "urn:bedrock:context/v1"
"@id": "urn:bedrock:vertex/plan-<slug>"
"@type": "urn:bedrock:ontology/Plan"
label: "<action title>"
intent: "<human-accepted bounded intent>"
acceptanceCriteria:
  - "<observable completion criterion>"
consumes:
  - "urn:bedrock:path/situation/assurance/runs/<run-id>"
tasks:
  - "<ordered task one>"
disposition:
  state: draft
  residual: "proposal only; not commissioned"
source: "urn:bedrock:path/situation/assurance/graph-manifest.yaml"
```

A grounded behavior or contract defect routes to a complete draft Plan
proposal. Only a later separate human invocation activates it and may
commission implementation. Evidence-only, declined, or accepted-residual
findings create no vertex.

Campaign-close proposal:

```yaml
"@context": "urn:bedrock:context/v1"
"@id": "urn:bedrock:vertex/reflect-verdict-<run-id>"
"@type": "urn:bedrock:ontology/ReflectVerdict"
label: "Assurance campaign <run-id>"
statement: >
  Campaign accounting closed against assurance graph manifest sha256
  <manifest-digest> and run graph sha256 <run-graph-digest>.
subject: "urn:bedrock:path/situation/assurance"
criteria:
  - "<charter conclusion criterion>"
witnesses:
  - "https://<completed-two-checker-ci-run-url>"
disposition:
  state: done
  residual: "<declared unassured remainder>"
source: "urn:bedrock:path/situation/assurance/graph-manifest.yaml"
```

The ReflectVerdict subjects the registered mount path, cites charter criteria,
and uses only an already-completed two-checker CI URL. Projection adds these
proposal files plus the promotion bundle; it never edits pre-existing mount
source/evidence to mirror a base vertex.

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

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
