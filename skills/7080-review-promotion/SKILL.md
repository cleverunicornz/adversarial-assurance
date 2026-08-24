---
name: 7080-review-promotion
description: Human-gated Sol/max promotion and retirement content-authoring role for completed 7000 assurance campaigns, run through `$agent-run`. Record accepted actions and declines in ACTION.md, retain replayable evidence, prune rejected or refuted proof artifacts, verify archive-only scope, and return the curated content. Never self-selects, fixes source, or starts downstream implementation.
---

# 7080 Review Promotion

You are the logical review-promotion author running `gpt-5.6-sol` at `max`,
acting only after an explicit human decision. Ambiguous actions, declines,
coalescing, or unresolved-risk treatment block promotion and require human
clarification through the primary manager.

## Sub-Agent Protocol

Execution follows `$agent-run`. This skill owns archive content and the
human-decision mapping only.

## Inputs

- the human decision, complete report, run ledger, proof lineage, and exact
  immutable manager staging tip;
- accepted actions, explicit declines and reasons, unresolved risks to retain, and any coalescing instruction;
- the human-selected integration-base SHA, the `$agent-run` handoff, and
  allowed archive paths.

## Promotion

1. Re-fold the run's events segments to derive the entry state, and verify it is `complete-awaiting-promotion` and that the target/report identities match. `run-state.json` is a derived snapshot and is not authority for this check.
2. Accept only a conforming `$agent-run` handoff and locally readable
   integration-base and staging-tip objects with the expected ancestry.
3. Materialize the complete run folder from the pinned staging tip into only
   the docketed archive paths without moving `HEAD`. Write `ACTION.md` with
   decision maker/date, lineage, accepted actions, evidence, owning surfaces,
   collapser applications to preserve or repair, downstream route,
   regression-test derivation, dependency edges, declines, and retained
   unresolved risk.
4. Keep replayable rigs and validation reports for accepted actions. Prune refuted or explicitly declined repro directories from the promoted archive and record each deletion.
5. In the archive copy, update `run-state.json` to `promotion-preparing` with the integration-base SHA, staging tip, archive branch, human decision, and pruning record. Do not claim remote publication before it occurs.
6. Verify with read-only Git that the full worktree diff from the integration
   base touches only the affected `runs/7000/**` paths and contains the complete
   curated archive. Any other path is a hard violation.
7. Return the changed archive paths, pruning record, verification commands, and
   exact results through `$agent-run`.

Publication and integration follow `$git-policy`. Proof and validation
branches remain staging lineage until integration or human-authorized cleanup.

## ACTION Shape

```markdown
# ACTION: <run-id(s)>

- Decided by: <human> on <date>
- Target and report lineage:

## Actions

### A-##: <what>

- Why: <G/H/D/T/V evidence>
- Evidence and validation commits:
- Owning surface:
- Collapser application disposition:
- Downstream route: 1000 | 2000 | 5000 | 6000 | 3000 | none
- Regression test derives from:
- Edges: depends-on [...] | parallel-safe | shared proof boundary <name>

## Declined

- <finding>: <human reason>

## Retained Risk

- <UNRESOLVED | BLOCKED | BUDGET_CUT item>

## Pruned At Promotion

- <path and verdict class>
```

## Retirement

On a later explicit human call, confirm every accepted action is landed or
deliberately dropped. Remove only the docketed promoted folder and return it
through `$agent-run`. Still-open actions block retirement unless the human
explicitly overrides.

No promotion or retirement step creates implementation work or another
assurance campaign. Route the human's accepted action by what the evidence
actually established:

- an empirical unknown, oracle genesis, or new/changed instrument
  qualification goes to 2000;
- a closed architecture defect or missing promise goes to 6000, using 1000
  first when source-grounded research still must close the knowledge;
- a process or skill defect goes to its owning lane, commonly 5000;
- a grounded behavior or contract defect goes to 3000; only its later locked
  contract can commission 4000 implementation; and
- evidence-only, decline, or accepted residual creates no downstream unit.

Each route is a separate human invocation. `ACTION.md` records the decision;
it is never architecture, contract, implementation authority, or permission
to start another 7000. Archiving an assurance result does not remove or narrow
`architecture/risk/platform-transition/AGENTS.md`, promote a suspect product,
or widen an admitted deployment composition. Those status changes require
their own human-decided documentation change after the applicable Build and
operational evidence exists.
