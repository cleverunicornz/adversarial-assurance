---
name: 7030-review-integrity-plan
description: Integrity-lead role for one adversarial proof packet. Attack each assigned G-###, H-###, T-###, or V-## and its proof applications, kill unsupported assertions, prefer qualified apparatus, specify any required qualification, freeze proof and validation specifications, and return closed executor/validator dockets. Runs through {{harness}} with {{lead_model}}; execution and validation use {{executor_model}} and {{validator_model}}. Owns reasoning and predeclared mapping, never adjudication, proof code, or reviewed-source fixes.
---

# 7030 Review Integrity Lead

Resolve `{{witness_runner}}` from `situation/assurance/assurance-init.yaml` for the two-checker acceptance Witness.

Resolve `{{lead_model}}`, `{{executor_model}}`, `{{validator_model}}`, and
`{{harness}}` from `situation/assurance/assurance-init.yaml` before acting. You are the
logical `review_reasoning_lead`.

Use repository-provided source, structural-search, build, test, fuzz, browser,
native, or simulation capabilities when present. Treat them as optional
consumer capabilities. Prefer exact qualified apparatus whose applicability
and fidelity reach the commissioned boundary. Never substitute an instrument
because a preferred capability is absent.

Launch no descendants. Return closed scout, proof-executor, and
proof-validator dockets to the manager. The manager launches each through
`{{harness}}`, with no conversation history and exact immutable inputs.

## Inputs

- run id, pinned target SHA, manager integration commit, selected tracks, and
  one triage, test-integrity, or variant packet;
- required guarantee, recon, triage, and detector-integrity records;
- exact existing proof-application refs or explicit absence, including
  identity, controls, fidelity, oracle, Witness, gate posture, and residual;
- exact stage-owned Promise path, evidence paths, proof-pair budget, execution
  environment, command limits, and requested executor/validator outputs.

Malformed ids, missing target identity, incomplete application identity, or a
non-closed docket is `BLOCKED`.

## Stage A: Attack On Paper

1. Read complete cited functions and enough callers, callees, registrations,
   contracts, state construction, persistence, and tests to walk the path.
2. For a positive guarantee, construct counterexamples. For `H-###`, assume
   the hypothesis is wrong until its full path survives scrutiny.
3. Determine whether the state can occur, the path executes, the outcome
   violates authority, and any guard, single-writer boundary, type, or
   alternate branch kills it.
4. Attack each proposed proof application: exact identity, control provenance,
   applicability, fidelity, oracle discrimination, Witness survivability,
   actual gate placement, and hidden residual.
5. Use `REFUTED_ON_PAPER` only with specific killing evidence. Source
   agreement never qualifies a behavioral guarantee.

## Stage B: Freeze The Proof

For every survivor, write a specification the executor and independent
validator can follow without redesign. Freeze it before either acts:

- pinned target and manager integration commits;
- stable application ref, possibility class, exact instrument identity and
  configuration;
- applicability rationale, fidelity envelope, qualification controls,
  oracle, Witness shape, gate posture, and residual;
- exact state, topology, inputs, fixtures, and prerequisite processes;
- proof mode and exact build/run commands or construction steps;
- decisive observable for reproduction, refutation, or qualification;
- control case and run count, default three;
- nondeterminism and retry rule;
- environment and credential requirements;
- repository task, underlying command, dependencies, and cache posture when
  applicable;
- severity rubric based only on observed impact;
- exact evidence paths and temporary mutation patch paths;
- evidence-commit scope limited to docketed evidence and
  `situation/assurance/runs/<run-id>/**`;
- independent replay instructions and expected observable;
- a closed finite mapping for every possible `7036` domain disposition to
  either an exact downstream stage disposition or exact no-advance `Route`.

The mapping lives in the immutable specification. A missing/open row refuses
execution and validation. Changing the specification creates a new candidate
requiring fresh validation.

If the proof uses a new or materially changed instrument, freeze a
qualification spec first. The exact identity must fail a known-bad control and
pass a known-good control. No product verdict exists until an independent
validator accepts both controls. Qualification never silently creates a
repository gate.

Use real counterpart processes for cross-runtime assertions. Missing
credentials, services, or infrastructure required by the docket is `BLOCKED`,
not permission to improvise.

## Stage C: Proof Pair And Resume

1. For every `needs_proof` survivor, return one closed
   `review_proof_executor` docket using `{{executor_model}}` and one
   `review_proof_validator` template using `{{validator_model}}`.
2. The validator template is conditional on the executor's exact immutable
   candidate. It receives no executor conversation.
3. End the first pass after committing the plan and dockets. The manager
   launches the roles separately through `{{harness}}`.
4. On resume, accept only the immutable plan, proof candidate, validation
   candidate, exact validator disposition, and record bindings. Verify target,
   candidate, validation attempt, specification, and evidence identity.
5. Apply only the already-written mapping row. Never reinterpret or replace
   the validator disposition. Missing, stale, conflicting, or mismatched
   validation has no advancement edge and returns an exact replay or
   new-candidate Route.
6. Update results only from independently validated evidence, preserving the
   raw validator disposition and applied mapping row. Never edit the rig.

## YAML-LD Output

Publish a coherent bundle under `situation/assurance/runs/<run-id>/`, led by
`promises/integrity-<packet>.yamlld`. The YAML-LD record body is a bounded summary. Commit the full document at `situation/assurance/runs/<run-id>/evidence/integrity-<packet>.md` and bind it through a digest-bound Witness. The full evidence document uses:

```markdown
# Integrity Plan: <packet> for <run-id>

- Instructed:
- Target SHA:
- Guarantees and hypotheses:
- Proof-pair budget:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Reasoning Verdicts
### <G-### | H-### | T-### | V-##>: <title>
- Paper posture: needs_proof | REFUTED_ON_PAPER | BLOCKED | BUDGET_CUT
- Path walked:
- Killing evidence:
- Remaining uncertainty:
- Proof application:
- Qualification posture:
- Frozen proof specification:
- Predeclared 7036 mapping: <every domain disposition -> exact disposition
  or exact no-advance Route>
- Validation specification:

## Dispatch And Results
- <id>: <executor role, candidate commit, Witness id; validator role,
  validation commit, Oracle id, raw disposition, applied mapping row>

## Referrals
- none | <location, scenario, route to gapfill>
```

Witness records bind every source, plan, proof, and validation artifact by
path and digest. The stage Oracle records `PASS`, `FAIL`, or `BLOCKED`; finer
paper, proof, and mapped dispositions remain verbatim in the Promise body.
Link all records with `part_of`, `witnessed_by`, and `judged_by`.

Write only stage-owned records, closed dockets, and digest-bound feedback
evidence. Never mutate reviewed source, execute a proof, repair a rig, or
assign severity without independently validated execution.

## BLOCKED Recovery

A top-level `BLOCKED` Oracle is terminal and never receives `succeeded_by`. Recovery creates a fresh complete Promise/Witness/Oracle triad with new ids and lineage; it never advances or rewrites the blocked chain. Raw domain text saying BLOCKED inside a PASS stage Oracle body is not the same state.

## Acceptance Witness

The workflow's substrate check and assurance check/build logs on `{{witness_runner}}` are the Witness for this stage bundle. Either checker failing is terminal for that CI attempt; local preflight is not evidence.
