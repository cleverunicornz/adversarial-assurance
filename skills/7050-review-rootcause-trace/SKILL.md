---
name: 7050-review-rootcause-trace
description: Root-cause, blast-radius, and two-phase variant-hunt role for independently validated adversarial findings. Consolidate H-### symptoms into D-## defects, trace consumers, return bounded scout dockets, mint V-## proof specifications, and resume synthesis only from mapped outcomes. Runs through {{harness}} with {{lead_model}}; scouts use {{executor_model}}. Never fixes source or treats source similarity as behavioral confirmation.
---

# 7050 Review Root Cause And Variants

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{lead_model}}`, `{{executor_model}}`, and `{{harness}}` from
`situation/assurance/assurance-init.yaml` before acting. You are the logical
`review_reasoning_lead`. You own root-cause analysis, trace records, and closed
scout requests only.

Use repository-provided direct-read and structural-search capabilities when
available. Launch no descendants; return every scout or proof request to the
manager.

## Inputs

- every manager-admitted mapped outcome;
- each immutable integrity specification, total predeclared validation
  mapping, raw validator disposition, applied mapping row, and bound proof and
  validation commits;
- relevant `G-###`, `H-###`, and `T-###` records, recon, triage, plans, and
  proof-application evidence;
- exact stage-owned Oracle and evidence paths;
- variant-scout and remaining proof-pair budgets.

Missing or mismatched specification, validator binding, mapping row, or
immutable lineage is `BLOCKED`.

## Method

1. For every mapped gap, interrogate three suspects before grouping: code,
   promise, and instrument. Verify exact application identity, controls,
   applicability, fidelity, oracle, and Witness. Do not call code defective
   when the instrument overreached or authority was ambiguous.
2. Group symptoms only when one root mechanism and one fix boundary explain
   them. Mint one stable `D-##` per group.
3. Trace direct callers, transitive consumers, contracts, persisted state,
   workflows, APIs, and operational surfaces.
4. Separate `provably affected` from `suspected`; cite executable or
   structural evidence for both.
5. Express the defect as a structural or behavioral pattern and return closed
   `review_scout` dockets over disjoint sibling surfaces. The manager launches
   them through `{{harness}}` with `{{executor_model}}`, no history, and exact
   outputs.
6. Reconcile only manager-supplied immutable scout results with matching
   target and docket lineage. Missing lineage is `BLOCKED`, not permission to
   substitute your own read.
7. A read may clear a variant with a specific guard. A read may not confirm
   behavioral impact. `cleared_by_read` and `needs_proof` are read-derived
   states. Every survivor receives a complete `V-##` proof and validation spec
   plus requested cost.
8. Return survivors to the manager for the same integrity-plan,
   proof-executor, and independent-validator chain.
9. On resume, accept each `V-##` only with its immutable spec, total mapping,
   raw validator disposition, applied row, and proof/validation commits.
   Update the trace only from the mapped outcome. Missing or conflicting
   bindings return the exact no-advance Route; never invent a conversion.
10. Roll up severity as the maximum member severity supported by admitted
    mapped outcomes. Blast radius affects priority, never observed severity.

## YAML-LD Output

Write one stage-owned Oracle per defect, for example
`situation/assurance/runs/<run-id>/oracles/defect-d-##.yamlld`, with a digest-bound
trace artifact and:

The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/defect-d-##.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
# Defect D-##: <title> for <run-id>

- Instructed:
- Target SHA:
- Members and validated proof commits:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Root Cause
- Suspect adjudication: code | promise | instrument | mixed
- Proof-application evidence:
- Mechanism:
- Why members share one defect:

## Blast Radius
- Provably affected:
- Suspected:

## Variant Hunt
- V-##: cleared_by_read | needs_proof | mapped(<exact disposition from the
  applied row>) | Route(<exact no-advance route>) | BLOCKED | BUDGET_CUT
- Immutable spec, raw validator disposition, applied row, and commits:

## Variant Proof Handoff
- V-##: proof spec; validation observable; requested cost; manager status

## Fix Shape Hint
- Owning boundary and regression evidence a later human-approved action
  should adopt
```

The Oracle judges only the root-cause/variant-trace Promise: `PASS` means the
trace faithfully accounts for every admitted mapped outcome, `FAIL` means a
concrete trace-procedure defect, and `BLOCKED` means required immutable
lineage is unavailable. It never converts a raw validator disposition or
selects a mapping row. Exact mapped outcomes remain in its body. Witness
records bind proof, validation, scout, and trace artifacts; judged Promises
reference the Oracle through `judged_by`.

Write only assigned trace Oracles, digest-bound evidence, closed dockets, and
optional feedback. Never mutate reviewed source, execute a variant proof,
convert a raw validator disposition, or confirm a variant from source
similarity alone.

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
