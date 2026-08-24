---
name: 7025-review-test-integrity
description: Test and detector-integrity stage for a human-invoked adversarial campaign. Map claimed tests or checker applications to G-### guarantees, determine whether exact qualified instruments observe decisive behavior at faithful boundaries, design breaks that should make each fail, and emit immutable proof specifications. Runs through {{harness}} with {{lead_model}}; later execution and validation use {{executor_model}} and {{validator_model}}. Never mutates reviewed source.
---

# 7025 Review Test Integrity

Resolve `{{lead_model}}`, `{{executor_model}}`, `{{validator_model}}`, and
`{{harness}}` from `.assurance/assurance-init.yaml` before acting. You are the
logical `review_reasoning_lead`. Passing tests are assertions, not proof, until
their decisive reach and failure sensitivity are attacked.

Use repository-provided search, build, test, fuzz, browser, native, or
simulation capabilities when present and applicable. They are optional
consumer capabilities, never hard campaign dependencies. Credit no target,
receipt, or tool family without resolving its exact command, identity,
configuration, controls, applicability, and fidelity.

## Inputs

- pinned target SHA, `G-###` registry, recon test inventory, `H-###` registry,
  and selected tracks;
- changed or advertised detectors and their owning production paths;
- exact proof-application refs, identities, controls, fidelity envelopes,
  Witnesses, and gate posture asserted by delivery;
- proof-pair budget and exact stage-owned record paths.

Missing target identity, detector identity, or required authority is
`BLOCKED`.

## Method

1. Map every relevant test or detector to the exact guarantee, possibility
   class, and decisive observable it claims to protect.
2. Inspect identity, qualification controls, fixtures, setup, assertions,
   exclusions, baseline/topology, mocks, process boundaries, negative paths,
   and whether the application reaches the advertised production boundary
   within its stated fidelity.
3. Resolve any repository task target to its owner command, dependencies, and
   cache posture. Independently inspect environment receipts. A cached green
   result is not sensitivity evidence.
4. Design the smallest faithful break that should make the detector fail:
   inverted guard, removed idempotency, wrong route, early acknowledgement,
   authority bypass, altered ordering, dropped persistence, or another
   guarantee-specific mutation.
5. Define the expected failing signal, control run, exact temporary patch or
   fixture, run count, and cleanup. Do not apply it to reviewed source.
6. Freeze executable mutation specs for later
   `review_proof_executor`/`review_proof_validator` pairs. The executor uses
   `{{executor_model}}`; the independent validator uses `{{validator_model}}`;
   both launch separately through `{{harness}}` with no shared history.
7. Classify source-only analysis `PARTIAL`. Only executed, independently
   validated sensitivity may produce `PROTECTS_GUARANTEE` or
   `DOES_NOT_PROTECT`.

## YAML-LD Output

Publish a coherent bundle under `.assurance/runs/<run-id>/`, led by
`promises/test-integrity.yamlld`. Its body uses:

```yaml
body: |
  # Test Integrity: <run-id>

  - Instructed:
  - Target SHA:
  - Tests inventoried:
  - Rubric status: complete | partial(<missing>) | blocked(<why>)

  ## Guarantee-To-Test Matrix
  ### T-###: <detector>
  - Guarantees:
  - Application and exact identity:
  - Qualification controls:
  - Production path reached:
  - Decisive assertion:
  - Boundary fidelity:
  - Gate posture and residual:
  - Proposed break:
  - Expected failure signal:
  - Control command:
  - Repository task and underlying command: none | <task and command>
  - Execution environment: none | <receipt and independently checked fields>
  - Mutation patch or fixture spec:
  - Domain disposition: needs_execution | PROTECTS_GUARANTEE |
    DOES_NOT_PROTECT | PARTIAL | BLOCKED | BUDGET_CUT

  ## Proof Dispatch
  - T-###: <immutable proof specification and required independent replay>

  ## Unprotected Guarantees
  - G-###: <gap>
```

Witness records bind every inspected detector and authority source. The stage
Oracle records `PASS` when the matrix and executable specs are complete,
`FAIL` for a concrete integrity defect, or `BLOCKED` for a missing
prerequisite; finer domain dispositions remain verbatim in the body. Link the
Promise through `witnessed_by`, `judged_by`, and `part_of`.

Write only stage-owned records and optional digest-bound feedback evidence.
Do not execute the proposed break, assign severity, or mutate reviewed source.
