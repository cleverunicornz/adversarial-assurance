---
name: 7070-review-report
description: Final evidence-synthesis stage for a mechanically complete human-invoked adversarial campaign. Compile the guarantee certificate, detector-integrity verdicts, independently reproduced defects, refuted counterexamples, open and blocked risk, coverage, proof lineage, retrospective, and recommended actions. Runs through {{harness}} with {{lead_model}} under {{reviewer_seat}}. Never re-reviews, invents evidence, changes severity, fixes source, or upgrades an unvalidated result.
---

# 7070 Review Assurance Report

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{lead_model}}`, `{{harness}}`, and `{{reviewer_seat}}` from
`situation/assurance/assurance-init.yaml` before acting. You are the logical
`review_reasoning_lead`. Begin only after the manager's mechanical sweep
passes and every declared id is terminal.

## Inputs

- the complete committed Run, Promise, Witness, and Oracle set;
- mechanical-sweep Oracle and required assurance-retrospective Oracle;
- pinned target and risk/architecture registers when present;
- exact stage-owned report Oracle and digest-bound artifact paths.

Missing terminal accounting, retrospective, or immutable lineage is
`BLOCKED`.

## Evidence Rules

- `QUALIFIED` requires every declared obligation and independent validation
  named by the guarantee.
- `FALSIFIED` and every defect require independently replayed proof and
  validation commits.
- Executor-only observation is `UNRESOLVED`.
- Copy the independent validator's exact domain disposition only after
  verifying target, proof candidate, validation attempt, specification,
  evidence, and mapping-row bindings. Missing, stale, conflicting, or
  mismatched validation is unvalidated residue.
- Detector-integrity verdicts cite the deliberate break, expected failure
  signal, and independent replay.
- Severity is copied exactly as validated.
- Coverage compares charter inventory with immutable landed evidence and
  names every residual gap.
- Product status is copied from pinned target registers when present. A
  qualified guarantee never promotes a suspect surface or establishes
  deployment authority.
- Every guarantee accounts for live possibility classes and exact proof
  applications used or attacked. A tool family, green command, or executor
  identity assertion is not a terminal application disposition.
- Carry retrospective cost verdicts and standing questions without using them
  to alter severity or technical disposition.

## YAML-LD Output

Write `situation/assurance/runs/<run-id>/oracles/report.yamlld`, resolving to a
digest-bound report artifact. The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/report.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
# Assurance Report: <run-id>

- Target SHA and authority identity:
- Risk disposition:
- Tracks:
- Budget class and actual usage:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Executive Verdict
- Guarantees: <qualified/falsified/unresolved/blocked/budget-cut counts>
- Defects: <severity counts>
- Detector integrity: <protects/does-not-protect/partial counts>
- Bounded conclusion:

## Guarantee Certificate
### G-###: <title> — QUALIFIED | FALSIFIED | UNRESOLVED | BLOCKED |
  BUDGET_CUT
- Obligations and evidence:
- Possibility classes:
- Proof applications: <identity, controls, applicability, fidelity, oracle,
  Witness, gate posture, residual, validation lineage>
- Counterexamples attempted:
- Proof and validation commits:
- Assumptions and residual limits:

## Reproduced Defects
### D-##: <title> — <validated severity>
- H members, root cause, validated repros, blast radius, variants, and
  regression seed

## Test Integrity
- T-###: <verdict, guarantee, deliberate break, proof lineage>

## Refuted Counterexamples
- H-###: <paper or execution killing evidence>

## Open, Blocked, And Budget-Cut Risk
- <id, missing proof, blocker or budget decision>

## Coverage
- Planned vs completed inventory, tracks, guarantee x possibility x
  application coverage, methods, and gapfill

## Process Feedback
- none | <top bounded improvements>

## Assurance Retrospective
- Campaign median, flags, per-proof TRUE-COST | AVOIDABLE | MIXED | UNCLEAR,
  and standing questions; diagnostic only

## Recommended Actions
- A-##: <what, why, owning surface, evidence, dependency edges>
- Recommend decline: <reason>
```

The report Oracle judges only report completeness against charter-defined
conclusion criteria: `PASS` means every declared id and required section is
accounted for and the bounded conclusion follows those criteria; `FAIL` means
a concrete report-procedure defect; `BLOCKED` means required terminal lineage
is unavailable. The campaign conclusion and all finer dispositions remain in
the body. Witness records bind every proof, validation, trace, retrospective,
and report artifact; the report Promise uses `witnessed_by` and `judged_by`.

Every declared id appears exactly once in terminal accounting. Write only the
report Oracle, digest-bound report artifact, and optional feedback evidence.
Do not re-review evidence, change severity, mutate reviewed source, or author
the human promotion decision.

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
