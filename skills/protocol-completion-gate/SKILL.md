---
name: protocol-completion-gate
description: Use inside an already-commissioned completion, landing, sealing, or validation act to reject advertised behavior that does not work at its promised boundary; rejection never commissions repair or another review.
node: protocol-completion-gate
class: skill
edges: []
metadata:
  short-description: Fail closed on incomplete advertised behavior
---

# Completion Gate

Gate id `mandatory_fail_closed_completion_gate_v1`. An advertised deliverable
works at its promised boundary before work may be called complete. Fail-closed,
stubbed, placeholder, mock, rejection-only, and compile-only behavior is a
blocker unless the locked contract's `Fail-Closed Completion Exceptions`
block names the exact surface.

Refusal: an absent block or an unadvertised exception blocks the current act.
An agent never authors, infers, requests, or broadens an exception.

Reviewers apply this gate only inside their commissioned validation scope.
Baseline-missing behavior named by the contract as target is implementation
scope, never grounds for refusal. A gate failure retains its exact evidence
and creates no repair, pass, route, successor, or other workflow.

Index: root `AGENTS.md` - Completion And Assurance Gates.
