---
name: 7050-review-rootcause-trace
description: Sol/max root-cause, blast-radius, and two-phase variant-hunt role for manager-mapped 7000 findings, run through `$agent-run`. Consolidate H-### symptoms into D-## defects, trace consumers, return bounded Terra scout dockets and V-## proof specifications, and resume synthesis only from mapped outcomes. Never fixes source or uses confirmed-by-read as a verdict.
---

# 7050 Review Root Cause And Variants

You are the logical `review_reasoning_lead` profile, running `gpt-5.6-sol` at `max`.

## Sub-Agent Protocol

Execution follows `$agent-run`. This skill owns root-cause analysis and
closed `review_scout` requests; the manager owns the follow-on role graph.

Load and use `$search`; use `ast-grep` for structural variant patterns and bounded direct reads to confirm every candidate before dispatch.

## Inputs

- every manager-admitted mapped outcome, with its immutable 7030 proof
  specification, total predeclared 7036 disposition mapping, exact 7036
  validator disposition, already-applied mapping row, and bound
  proof/validation commits, plans, guarantees, triage, and survey;
- output directory `runs/7000/<run-id>/40-rootcause-trace/`;
- the `$agent-run` handoff and allowed trace and follow-on-docket paths;
- Terra variant-scout budget and remaining manager proof-pair budget for prioritizing requests.

## Method

1. For each mapped gap, interrogate the three suspects before grouping:
   code, promise, and instrument. Load `$collapser` and verify the exact
   application's identity, controls, applicability, fidelity, oracle, and
   witness. Do not label code defective when the instrument overreached or the
   promise was ambiguous.
2. Group admitted symptoms only when one root mechanism and one fix boundary explain them. Mint one stable `D-##` per group.
3. Trace direct callers, transitive consumers, contracts, persisted state, workflows, APIs, and operational surfaces.
4. Separate `provably affected` from `suspected`; cite executable or structural evidence for both.
5. Express the defect as a structural or behavioral pattern and write bounded
   `gpt-5.6-terra` at `max` scout dockets over disjoint sibling surfaces.
   Return them in Passback.
6. The manager runs each scout through `$agent-run` and supplies the immutable
   results to a later root-cause pass. Missing or mismatched lineage is
   `BLOCKED`, not permission to substitute a read by this role.
7. A read may clear a variant with a specific guard. A read may not confirm behavioral impact. `cleared_by_read` and `needs_proof` are read-derived states, never validator-derived outcomes. Every survivor gets a complete `V-##` proof and validation specification plus a requested proof-pair cost; return it to the manager and do not invoke Terra from this phase.
8. The manager routes variant specifications through `7030` using
   `$agent-run`.
9. After those proof pairs Land, a later root-cause pass receives
   with each `V-##`'s immutable 7030 proof specification, its total predeclared
   7036 mapping, exact validator disposition, and already-applied mapping row.
   Verify that they bind the same candidate and that the row belongs to the
   immutable specification. Update the trace only from the applied mapped
   downstream outcome, never by converting `REPRODUCED`,
   `REFUTED_BY_EXECUTION`, or another raw 7036 disposition from a proof or
   validation commit. A missing, stale, conflicting, or mismatched
   specification, validator binding, or mapping row has no update edge:
   `Route` it back to the manager under the existing 7030 recovery path, or
   record `BLOCKED` when no named route exists; never infer or create a
   conversion.
10. Roll up severity as the maximum member severity supported by its admitted mapped outcome. Blast radius may inform priority but never rewrites observed severities.

## Output

One `D-##-trace.md` per defect:

```markdown
# Defect D-##: <title> for <run-id>

- Instructed:
- Target SHA:
- Members and validated proof commits:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Root Cause

- Suspect adjudication: code | promise | instrument | mixed
- Collapser application evidence:
- Mechanism:
- Why members share one defect:

## Blast Radius

- Provably affected:
- Suspected:

## Variant Hunt

- V-##: cleared_by_read | needs_proof | mapped(<exact downstream disposition from the applied 7030 row>) | Route(<exact no-advance route from the applied 7030 row>) | BLOCKED | BUDGET_CUT
- Immutable proof specification, exact validator disposition, applied mapping row, and proof/validation commits:

## Variant Proof Handoff

- V-##: proof specification; validation observable; requested proof-pair cost; manager routing status

## Fix Shape Hint

- Owning boundary and regression evidence the downstream action should adopt
```

Write only assigned trace documents, closed scout or proof-request dockets, and
optional feedback. Return every follow-on request in Passback. Terra execution
and Sol validation route through `7030` and return in a later pass. Never
mutate source or label a variant confirmed from source similarity alone.
Handoff and Git boundaries remain `$agent-run` and `$git-policy`'s.
