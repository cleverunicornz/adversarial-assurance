---
name: 7065-review-assurance-retrospective
description: Standing cost-of-knowing retrospective for every human-invoked adversarial campaign. Run after the mechanical sweep and before the final report, when every id and timing record is terminal. Audit each proof and packet from committed YAML-LD lineage, immutable evidence commits, retries, validation outcomes, and commit timestamps, then render one bounded cost verdict per proof. Runs through {{harness}} with {{lead_model}}. Never relitigates findings, weighs proof merits, targets time as a KPI, overturns a verdict, inspects reviewed source, or mutates anything but its own record and artifact.
---

# 7065 Review Assurance Retrospective

Resolve `{{lead_model}}` and `{{harness}}` from
`.assurance/assurance-init.yaml` before acting. You are the logical
`review_reasoning_lead`. This standing stage runs on every campaign after the
mechanical sweep and before the report.

Begin only when every `G-###`, `H-###`, `T-###`, and `V-##` is terminal and
all timing evidence is complete. Audit the cost of knowing from process
records only. The conditional feedback stage proposes tooling improvements;
this stage measures what the campaign already spent. Never move verdicts
between them.

## Metric Discipline

- Time-to-assure is recorded, never targeted. It is diagnostic, not a KPI.
- Exceeding a time budget is a finding about the system, never grounds to
  invalidate technical evidence.
- Proof quality remains governed exclusively by immutable proof and
  independent validation records. This stage never reopens, re-severities,
  downgrades, or overturns a finding.

## Inputs

- committed Run, Promise, Witness, and Oracle records;
- closed dockets, immutable proof and validation commits, retries, reruns,
  terminal dispositions, gapfill records, and root-cause records;
- commit timestamps for dispatch, proof, validation, and integration;
- exact stage-owned Oracle and evidence paths.

Never inspect reviewed product source or run build, test, proof, or provider
commands. Missing terminal ids or timing evidence is `BLOCKED`; do not
estimate.

## Method

1. Enumerate every terminal proof and packet across `G-###`, `H-###`,
   `T-###`, and `V-##`, including its proof/validation pair.
2. Compute wall-clock from durable commits and record transitions, from first
   dispatch to integrated validation. Compute the campaign median and each
   proof's ratio to it.
3. Partition delay into exactly:
   - `depth-cost`: irreducible distance to the decisive observable;
   - `friction-cost`: rework, retries, harness failures, orchestration
     failures, and every version-control operation failure;
   - `excluded-cost`: only events beyond engineering control. Every exclusion
     requires evidence-grade justification; otherwise it is friction.
4. Record what advanced the proof and whether it reused qualified apparatus
   or paid genesis/requalification cost.
5. Count retry loops, spec rewrites, rig rebuilds, and validator reruns.
6. Separate first-proof cost from later requalification cost.
7. Emit exactly one bounded verdict per proof.
8. For `AVOIDABLE` or `MIXED`, rank concrete optimizations with cheap
   counterfactual estimates when visible.
9. Flag outliers and carry each flag to the human gate as a standing question.

## Verdict Semantics

- `TRUE-COST`: no alternative path is visible from evidence available to this
  retrospective; never claim no alternative exists absolutely.
- `AVOIDABLE`: identifiable different choices plausibly reduce cost
  materially; list them.
- `MIXED`: partition true and avoidable portions.
- `UNCLEAR`: evidence is insufficient for a stronger bounded assertion.

Flag wall-clock at or above three times the campaign median, any non-
`TRUE-COST` verdict on a floor-critical guarantee, and every `UNCLEAR`.

## YAML-LD Output

Write `.assurance/runs/<run-id>/oracles/assurance-retrospective.yamlld`
resolving to a digest-bound retrospective artifact. Its body uses:

```yaml
body: |
  # Assurance Retrospective: <run-id>

  - Target SHA and authority identity:
  - Campaign median wall-clock:
  - Proofs and packets audited:
  - Rubric status: complete | partial(<missing>) | blocked(<why>)

  ## Metric Discipline
  - Time-to-assure is recorded, not targeted. Nothing here overturns a
    finding, guarantee, hypothesis, detector verdict, or severity.

  ## Campaign Footer
  | proof/packet | wall-clock | ratio vs median | verdict | flag |
  |---|---|---|---|---|
  | <id> | <hh:mm> | <r>x | <verdict> | <flag or -> |

  ## Per-Proof Ledger
  ### <proof/packet id> | <wall-clock> | <r>x median
  - What held it up:
    - depth-cost:
    - friction-cost:
    - excluded-cost: <item> — justification: <evidence>
  - What advanced it:
  - Apparatus reuse: reused qualified | requalified | new qualification | none
  - Iterations and redesigns:
  - First-proof vs requalification:
  - Cost verdict: TRUE-COST | AVOIDABLE | MIXED | UNCLEAR
  - Optimizations for AVOIDABLE or MIXED, ranked:
    1. <move> — counterfactual: <bounded estimate>
  - Standing question: <only when warranted; otherwise omit>
```

The Oracle records `PASS` when every proof has one bounded cost verdict,
`FAIL` for a demonstrated accounting defect, or `BLOCKED` for incomplete
terminal/timing evidence. Witness records bind all consumed process records
and timestamps. Link with `part_of` and the stage Promise through
`witnessed_by` and `judged_by`.

Write only this Oracle and its digest-bound artifact. Never inspect reviewed
source, execute technical checks, call provider APIs, or alter any technical
disposition.
