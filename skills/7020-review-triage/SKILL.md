---
name: 7020-review-triage
description: Triage and coverage stage for a human-invoked adversarial campaign. Consume the guarantee registry and all recon outputs, account for every candidate and inventory item, deduplicate counterexamples, mint stable H-### hypotheses, rank by expected risk and proof value, and form integrity packets within budget. Runs through {{harness}} with {{lead_model}}. Allocates proof work but never confirms, refutes, executes, assigns severity, or fixes source.
---

# 7020 Review Triage And Coverage

Resolve `{{lead_model}}` and `{{harness}}` from
`.assurance/assurance-init.yaml` before acting. You are the logical
`review_reasoning_lead`. You own triage, coverage accounting, and integrity
packet authoring only.

## Inputs

- pinned target SHA and human-approved charter;
- the `G-###` registry, recon survey, every scout Witness, every referral, and
  every known missing scout;
- proof-application inventory with exact identity, qualification, fidelity,
  oracle, Witness, gate posture, and residual;
- selected tracks and remaining lead, scout, proof-pair, and total budgets;
- prior evidence identified by immutable commit and exact target relation;
- exact stage-owned record paths.

Prior conclusions never transfer because titles match. Prior evidence is
lineage only; it kills a current candidate only when it still applies to the
pinned target and authority.

## Method

1. Account for every `G-###`, possibility class, proof application or explicit
   absence, inventory item, scout candidate, referral, and coverage gap.
2. Merge candidates only when they assert the same counterexample mechanism
   and proof path. Preserve every candidate lineage.
3. Mint stable `H-###` entries for surviving counterexamples.
4. Rank by potential contract impact, likelihood, and proof value. This is
   priority, never severity.
5. Mark each hypothesis `hunt` or `BUDGET_CUT`; never silently discard work.
6. Group one to five related hypotheses into `P-##` proof packets by shared
   authority, state construction, or harness. Carry exact application refs to
   reuse, attack, or qualify; never reduce them to a method family or force
   two leads to reread the same core path.
7. Leave vague residue visible with a reason and route missing coverage to
   gapfill.

Missing required lineage or unaccounted inventory is `BLOCKED`, not permission
to guess.

## YAML-LD Output

Publish a coherent bundle under `.assurance/runs/<run-id>/`, led by a
`promises/triage.yamlld` record. Its `body: |` contains:

```yaml
body: |
  # Triage And Coverage: <run-id>

  - Instructed:
  - Target SHA:
  - Inputs:
  - Rubric status: complete | partial(<missing>) | blocked(<why>)

  ## Guarantee Coverage
  - G-###: <inventory complete|gap; possibility classes; applications or
    absence; candidate ids; planned proof packets; current posture>

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
  - Applications to reuse, attack, or qualify:
  - Required methods:
  - Suggested proof-pair budget:

  ## Residue And Coverage Gaps
  - <item>: <reason and route>
```

Witness records bind every consumed recon artifact and immutable prior-evidence
commit. The stage Oracle is `PASS` only when all inputs and coverage are
accounted for; malformed lineage or omitted work is `FAIL` or `BLOCKED` with
the exact reason. Link the Promise through `witnessed_by`, `judged_by`, and
`part_of`.

Write only stage-owned records and optional digest-bound feedback evidence.
Do not execute proofs, validate candidates, assign severity, or mutate reviewed
source.
