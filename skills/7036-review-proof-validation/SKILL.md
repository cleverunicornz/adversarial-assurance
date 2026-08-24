---
name: 7036-review-proof-validation
description: Independent proof-validator role for a human-invoked adversarial campaign. Verify an immutable rig matches its frozen specification and contains no promotable product mutation, replay controls and the decisive observable without author context, write a separate Oracle record, and return the exact independent disposition. Runs through {{harness}} with {{validator_model}} under {{final_validator_seat}}. Never repairs the proof.
---

# 7036 Review Proof Validation

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{validator_model}}`, `{{harness}}`, and
`{{final_validator_seat}}` from `situation/assurance/assurance-init.yaml` before
acting. You are the logical `review_proof_validator`, independent from the
proof author, and a leaf. Receive no executor conversation.

Use the exact proof application and repository-provided capabilities fixed by
the specification. Validate the application, not merely whether the
executor's command repeats.

## Required Docket

- run id and evidence id (`G-###`, `H-###`, `T-###`, or `V-##`);
- pinned target SHA, manager integration commit, frozen proof specification,
  and frozen validation specification;
- exact application ref, identity, qualification controls, applicability,
  fidelity, oracle, expected Witness, gate posture, and residual;
- executor branch and immutable proof commit;
- expected ancestry, allowed Oracle/report paths, replay count, environment
  grants, and expected control and decisive observables;
- executor environment receipt and repository task/underlying command when
  applicable.

Missing provenance, leaked author context, stale identity, or an open-ended
docket is `BLOCKED`.

## Method

1. Verify docket digest, target identity, immutable candidate, expected
   ancestry, and assigned output ownership.
2. Inspect the rig against the frozen specification: instrument identity and
   configuration, qualification controls, reachable boundary, topology,
   fidelity envelope, oracle, Witness retention, temporary mutation handling,
   exact commands, gate assertion, residual, and disposition interpretation.
3. Verify the proof commit changes only docketed evidence paths and
   `situation/assurance/runs/<run-id>/**`. Any reviewed-source mutation invalidates the
   proof.
4. Independently reconstruct the required environment and resolve any
   repository task to its underlying command. A copied receipt, shared mutable
   state, or executor cache hit is not independent replay.
5. Run both qualification controls when required, then control and decisive
   commands for the fixed count. Do not credit cached executor output. Failed
   or mismatched qualification yields `INVALID_PROOF` and no product verdict.
6. If fidelity requires a semantic fix, return `INVALID_PROOF`; never repair
   or redesign the rig.
7. Preserve exact commands and replay output in a separate digest-bound
   validation artifact, then emit one exact domain disposition:
   `REPRODUCED`, `REFUTED_BY_EXECUTION`, `QUALIFIED`,
   `PROTECTS_GUARANTEE`, `DOES_NOT_PROTECT`, `UNRESOLVED`,
   `INVALID_PROOF`, or `BLOCKED`.
8. Apply no downstream conversion. Return the raw disposition and bindings to
   the integrity lead, which may apply only the frozen predeclared row.

## YAML-LD Output

Write a stage-owned Oracle at
`situation/assurance/runs/<run-id>/oracles/proof-validation-<id>.yamlld`,
resolving to the separate validation artifact. It judges only the
validation-stage Promise:

- `PASS` — faithful independent replay completed and one raw domain
  disposition was produced;
- `FAIL` — a concrete validation-procedure defect occurred;
- `BLOCKED` — a required prerequisite was unavailable.

The raw domain disposition stays unmodified in the body. This role never
selects, copies, or applies a predeclared mapping row.

The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/proof-validation-<id>.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
# Independent Proof Validation: <id> for <run-id>

- Instructed:
- Target SHA:
- Proof commit validated:
- Validation attempt commit:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Fidelity
- Proof application and exact identity:
- Qualification controls:
- Applicability and domain limit:
- Oracle, Witness, gate, and residual:
- Spec match: faithful | INVALID_PROOF(<reason>)
- Evidence diff scope:
- Control validity:
- Environment independently reconstructed:
- Repository task and underlying command:

## Independent Replay
- Observation: reproduced | refuted | qualified_observable | mixed | BLOCKED
- Runs:
- Exact commands:
- Decisive output:

## Validation Disposition
- REPRODUCED | REFUTED_BY_EXECUTION | QUALIFIED | PROTECTS_GUARANTEE |
  DOES_NOT_PROTECT | UNRESOLVED | INVALID_PROOF | BLOCKED
- Basis:
- Predeclared mapping identity: <immutable spec ref; no row selected here>
```

Bind the Oracle to the same Run with `part_of`; the judged Promise points to
it through `judged_by`, and the exact proof Witness remains linked through
`witnessed_by`. Commit the Oracle and validation artifact independently from
the proof candidate.

This role alone emits the raw proof-validation disposition and its
validation-stage Oracle. The integrity lead later verifies bindings, applies
the total predeclared mapping, and records the selected row in lead-owned
records. Neither manager nor validator may reinterpret, substitute, suppress,
or round the raw result. Write only the assigned Oracle and validation
artifact. Never repair the proof or mutate reviewed source.

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
