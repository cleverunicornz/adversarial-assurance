---
name: 7020-review-triage
description: Sol/max triage and coverage stage for a 7000 assurance campaign. Consume the guarantee registry and all recon outputs, account for every candidate and inventory item, deduplicate counterexamples, mint stable H-### hypotheses, rank by expected risk and proof value, and form integrity packets within the run budget. This stage allocates proof work but does not confirm, refute, execute, or fix anything.
---

# 7020 Review Triage And Coverage

You are the logical `review_reasoning_lead` profile, running `gpt-5.6-sol` at `max`.

## Sub-Agent Protocol

Execution follows `$agent-run`. This skill owns triage, coverage accounting,
and integrity-packet output only.

## Inputs

- target SHA, charter, `05-guarantees.md`, survey including the collapser
  application inventory, every scout output, and known missing scout;
- exact output `runs/7000/<run-id>/15-triage.md`;
- selected tracks and remaining Sol, Terra-scout, and Terra-proof budgets;
- prior review evidence identified by immutable commit and target relation.

Prior conclusions never transfer merely because titles match. Use prior evidence as lineage; only kill a current candidate when the cited evidence still applies to the pinned target and contract.

## Method

1. Account for every `G-###`, possibility class, collapser application or
   explicit absence, inventory item, scout candidate, referral, and coverage
   gap.
2. Merge candidates only when they assert the same counterexample mechanism and proof path. Preserve all candidate lineage.
3. Mint stable `H-###` entries for surviving counterexamples.
4. Rank by potential contract impact, likelihood, and proof value. This is priority, not severity.
5. Mark each as `hunt` or `BUDGET_CUT`; never silently discard work.
6. Group one to five related hypotheses into proof packets by shared authority, state construction, or harness. Carry the exact collapser application refs to reuse, attack, or qualify; never reduce them to a method family. Avoid making two Sol leads reread the same core path.
7. Leave vague residue visible with a reason and route missing coverage to gapfill.

## Output

```markdown
# Triage And Coverage: <run-id>

- Instructed:
- Target SHA:
- Inputs:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Guarantee Coverage

- G-###: inventory <complete|gap>; possibility classes; collapser
  applications or absence; candidates <ids>; planned proof <packets>;
  current posture

## Hypothesis Registry

### H-###: <title>

- Guarantee challenged:
- Concrete scenario:
- Forbidden outcome:
- Locations:
- Candidate lineage:
- Priority: <impact; likelihood; proof value>
- Disposition: hunt | BUDGET_CUT

## Integrity Packets

### P-##: <title>

- Guarantees and hypotheses:
- Shared proof path:
- Collapser applications to reuse, attack, or qualify:
- Required methods:
- Suggested proof-pair budget:

## Residue And Coverage Gaps

- <item>: <reason and route>
```

Write only the triage document and optional feedback. Do not execute proofs,
assign severity, or mutate source.
