---
name: 7036-review-proof-validation
description: Sol/max independent proof-validator role for a 7000 campaign, run through `$agent-run`. Verify the rig matches the Sol specification and contains no promotable product mutation, replay the control and decisive observable without author context, write a separate report, and return the independent disposition. Never repairs the proof.
---

# 7036 Review Proof Validation

You are the logical `review_proof_validator` profile, running `gpt-5.6-sol` at `max`. You are independent from the proof author and are a leaf.

Load `$collapser`; validate the exact application, not merely whether the
executor's command can be repeated.
When the proof used Moon or a named machine profile, load
`$moon-task-graph` and `$development-environment`.

## Sub-Agent Protocol

Execution follows `$agent-run`. This skill owns independent replay,
validation-report content, and disposition only.

## Required Docket

- run id, evidence id (`G-### | H-### | T-### | V-##`), target SHA, manager trunk commit, and exact Sol proof/validation specification;
- stable collapser application ref, exact identity, qualification controls,
  applicability, fidelity, oracle, expected witness, gate posture, and
  residual;
- executor branch and immutable proof commit;
- the `$agent-run` handoff, allowed report path, and expected proof-candidate
  ancestry;
- replay run count, environment grants, and expected control/decisive observables.
- executor development-environment receipt, fully qualified Moon target and
  underlying command when applicable, and the exact fields the validator must
  reconstruct independently.

Missing provenance, author context, or an invalid `$agent-run` handoff is
`BLOCKED`.

## Method

1. Accept only a conforming `$agent-run` handoff and verify that the immutable
   proof candidate has the expected ancestry and target identity.
2. Inspect the rig against the Sol specification. Verify the exact instrument
   identity and configuration, qualification controls, reachable boundary,
   topology, fidelity envelope, oracle, witness retention, temporary mutation
   handling, commands, gate claim, residual, and verdict interpretation are
   faithful.
3. Verify the proof evidence commit changes only `runs/7000/<run-id>/**`. Product-source changes in the evidence commit invalidate the proof.
4. Run both qualification controls when required, then the control and replay
   commands independently for the specified count. Do not use cached executor
   output as observed evidence. Failed or mismatched qualification makes the
   proof `INVALID_PROOF`; it cannot yield a product verdict.
   Resolve the Moon target and underlying command yourself and use a separate
   coherent Linux x86_64 actor-owned profile. A copied receipt, shared mutable
   cache, or executor cache hit is not independent replay.
5. If a semantic fix is required, return `INVALID_PROOF`; never repair or redesign the rig.
6. Write only the validation report and return its path, exact replay evidence,
   and disposition through `$agent-run`.

## Validation Report

```markdown
# Independent Proof Validation: <id> for <run-id>

- Instructed:
- Target SHA:
- Proof commit validated:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Fidelity

- Collapser application and exact identity:
- Qualification controls:
- Applicability and domain limit:
- Oracle, witness, gate, and residual:
- Spec match: faithful | INVALID_PROOF(<reason>)
- Evidence diff scope:
- Control validity:
- Native development environment reconstructed:
- Moon target and underlying command:

## Independent Replay

- Observation: reproduced | refuted | qualified_observable | mixed | BLOCKED
- Runs:
- Exact commands:
- Decisive output:

## Validation Disposition

- REPRODUCED | REFUTED_BY_EXECUTION | QUALIFIED | PROTECTS_GUARANTEE | DOES_NOT_PROTECT | UNRESOLVED | INVALID_PROOF | BLOCKED
- Basis:
```

This role alone emits the proof-validation disposition. It binds the instructed
target, immutable proof candidate, validation attempt, specification, and
independently replayed evidence above. Return the standard `$agent-run`
Passback plus proof commit, replay result, exact commands, and disposition. The
integrity lead and manager may verify the binding and apply the
predeclared mapping, but may not reinterpret, substitute, or round it.
