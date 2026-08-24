---
name: 7060-review-feedback-synthesis
description: Conditional process-feedback synthesis for a human-invoked adversarial campaign. Run only when feedback Witnesses exist. Deduplicate tool, checkout, environment, docket, budget, and procedure friction into one bounded improvement plan and quantify what evidence the campaign lost. Runs through {{harness}} with {{lead_model}}. Never judges review findings, calls provider APIs, fixes tooling, or mutates reviewed source.
---

# 7060 Review Process Feedback

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{lead_model}}` and `{{harness}}` from
`situation/assurance/assurance-init.yaml` before acting. You are the logical
`review_reasoning_lead`. No feedback evidence means the manager skips this
stage.

Discover inputs only through same-run `witnesses/feedback-*.yamlld` records
and their `evidence/feedback-*.md` targets. If a blocked stage/scout lacks its
mandatory feedback Witness, this synthesis is `BLOCKED` on incomplete process
evidence rather than silently treating friction as absent.

This stage is a prospective improvement list: what would have made evidence
collection cheaper or possible. The standing retrospective separately audits
what the campaign actually spent. Do not duplicate or import its cost
verdicts.

## Inputs

- every digest-bound feedback Witness for the run;
- relevant charter and stage records needed to locate evidence impact;
- exact stage-owned output record and artifact paths.

## Method

1. Account for every feedback Witness and immutable source commit.
2. Deduplicate the same mechanism across stages while preserving recurrence
   counts and lineage.
3. Classify each as `runtime`, `checkout`, `proof-tool`, `environment`,
   `docket`, `budget`, `skill-contract`, or `noise`.
4. Name the smallest local skill, role prompt, harness, environment, or
   procedure change that would unblock the evidence.
5. State exactly which guarantees, hypotheses, tests, proof applications, or
   coverage were lost.

## YAML-LD Output

Write `witnesses/process-feedback.yamlld` under the run, resolving to a
digest-bound synthesis artifact.

The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/process-feedback.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
# Process Feedback: <run-id>

- Inputs and commits:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Deduplicated Entries
- F-##: <class>; occurrences; smallest unblock; evidence impact

## Recommended Changes
- <owner surface and bounded change>

## No Action
- <entry and reason>
```

Bind the Witness with `part_of`. A paired stage Oracle records `PASS` when
every feedback item is accounted for, `FAIL` for a concrete synthesis defect,
or `BLOCKED` for missing required lineage. Preserve recommendations as
proposals only.

Write only the synthesis record and artifact. Do not create issues, call
provider APIs, judge campaign findings, fix tooling, or mutate reviewed source.

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
