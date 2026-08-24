---
name: 7060-review-feedback-synthesis
description: Conditional Sol/max process-feedback synthesis for a 7000 assurance campaign, run through `$agent-run`. Run only when lane feedback files exist. Deduplicate tool, checkout, environment, docket, budget, and process friction into one local improvement plan and quantify what evidence the run lost. This stage never calls provider APIs, judges review findings, or fixes tooling.
---

# 7060 Review Process Feedback

You are the logical `review_reasoning_lead` profile, running `gpt-5.6-sol` at `max`. No feedback files means the manager skips this stage. This stage is the prospective tooling wishlist addressed to toolmakers — what tooling would have helped; the standing 7065 assurance-retrospective stage owns the retrospective cost audit of each proof addressed to the human gate. Do not duplicate 7065's cost-of-knowing verdicts here.

## Sub-Agent Protocol

This role is run through `$agent-run`; this skill owns only the feedback docket,
synthesis method, and output below.

## Inputs

- every `feedback/*.md`, charter, run ledger, and exact output `90-report/process-feedback.md`.

## Method

1. Account for every feedback file.
2. Deduplicate the same failure across lanes while preserving recurrence counts.
3. Classify as `agent-runtime`, `checkout`, `proof-tool`, `environment`, `docket`, `budget`, `skill-contract`, or `noise`.
4. Name the smallest local skill, delegate-profile prompt, harness, or environment change that would unblock the evidence.
5. State which guarantees, hypotheses, tests, or coverage were lost.

## Output

```markdown
# Process Feedback: <run-id>

- Files ingested:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Deduplicated Entries

- F-##: <class>; occurrences; smallest unblock; evidence impact

## Recommended Changes

- <owner surface and bounded change>

## No Action

- <entry and reason>
```

Write only the synthesis. Do not call provider APIs, create issues, or mutate
source.
