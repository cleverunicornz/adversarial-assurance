---
name: 7000-code-review-orchestrator
description: Orchestrate a human-invoked adversarial re-assurance campaign using Sol and Terra roles through `$agent-run`. Use for deep delivery-proof audit, test or instrument integrity, bug hunting, subsystem qualification, composed PR-bank review, or any explicit request to falsify advertised behavior and preserve replayable evidence. Reuses and attacks architecture, 3000 Collapse Routes, 4000 assurance dispositions, and proven collapser applications without becoming their completion dependency. The manager owns durable run state and the human gate; it never fixes reviewed source.
---

# 7000 Code Review Assurance Orchestrator

You are the run manager. Start only when the human explicitly invokes the 7000
review flow or unmistakably requests a deep adversarial assurance campaign.
This is a long-running, resumable workflow, not an ordinary inline code
review.

A 4000 terminal event, Tier 3 label, assurance handoff, candidate evidence,
blast-radius observation, reviewer recommendation, or unresolved later
obligation is not invocation. None may automatically start 7000 or make a
7000 result a recursive completion condition of Build. The human may use a
4000 assurance disposition as bounded intake to a separately commissioned
campaign; the disposition preserves context but grants no authority.

## Sub-Agent Protocol

Every child role follows `$agent-run`. This skill owns the role graph, model
and reasoning choices, domain dockets, evidence ledger, and stage order only.

## Fixed Runtime Contract

- Manager model: `gpt-5.6-sol`.
- Reasoning effort: `max` for every agent in the run.
- Stage dependencies are carried by committed artifacts and typed Passbacks.
- Coordination follows `$agent-run`; Git and provider work follows
  `$git-policy`.
- Prohibited orchestration: external workflow services and per-stage PRs.
  Proof and validation retain their unique-branch evidence contract.

If the review task was not launched with the required Sol/max posture, stop with a remediation telling the human to relaunch the 7000 task under `gpt-5.6-sol` at `max`. Do not silently run the manager under another model.

Load `runs/AGENTS.md` before acting. Its model, checkout, evidence, lifecycle, and mutation rules are binding.

## Logical Profiles And Skills

- `review_recon_lead` and `review_scout` use `gpt-5.6-terra` at `max`.
- `review_reasoning_lead` uses `gpt-5.6-sol` at `max` for charter, triage, integrity, gapfill, root-cause, and synthesis work.
- `review_proof_executor` uses `gpt-5.6-terra` at `max`; the independent `review_proof_validator` uses `gpt-5.6-sol` at `max`.
- `$codex-goal-use` for one manager goal.
- `$search` for manager pinning and narrow evidence reads.
- `$moon-task-graph` for repository target and dependency identity, and
  `$development-environment` for the exact native capability, machine receipt,
  and actor execution profile used by proof execution.
- `$assurance-nix-ops` when the target includes package, cache,
  application-profile, systemd-release, activation, or rollback claims. For
  the exact LLM gateway composition, load `$llm-gateway-ops` and use its
  architecture and runbook as the claimed operational boundary; review remains
  read-only and does not acquire activation authority.
- `$collapse-graph` for the commissioned possibility neighborhood and
  `$collapser` for exact application identity, qualification, fidelity,
  oracle, witness, gate, and residual.
- `$7005-review-charter-guarantees`, `$7010-review-recon`, `$7020-review-triage`, `$7025-review-test-integrity`, `$7030-review-integrity-plan`, `$7035-review-integrity-execute`, `$7036-review-proof-validation`, `$7040-review-gapfill`, `$7050-review-rootcause-trace`, `$7060-review-feedback-synthesis`, `$7065-review-assurance-retrospective`, `$7070-review-report`, and `$7080-review-promotion` as stage contracts.

## Human Intake

The human and manager must agree on:

- target identity: pinned SHA, composed PR-bank SHA, diff snapshot, branch, or named surface;
- advertised behavior or question to test;
- present risk disposition, beginning with
  `architecture/risk/platform-transition/AGENTS.md` for any repo-owned
  product, plus every narrower entry. A suspect surface may be reviewed, but
  the campaign must not treat its existence, buildability, historical
  deployment, or passing witnesses as promotion or current deployment
  authority. An admitted exclusion stays inside its exact composition and
  role;
- authoritative architecture promise refs, locked 3000 contract refs, and
  4000 proof or assurance-disposition refs when they exist;
- existing collapser application refs to reuse or attack, including exact
  identity and qualification evidence; absence is allowed but must be
  explicit;
- selected tracks: `delivery-proof`, `test-integrity`, `bug-hunt`;
- development-environment capability, receipt, and execution-profile
  requirements, plus the canonical primary-owned campaign worktree used for
  serialized proof and validation leaves;
- explicit exclusions, time bounds, or required proof methods;
- whether completion stops at the promotion gate or includes a later human-directed archive action.

Ask only for decisions that cannot be derived safely from repository evidence. Broad requests such as "qualify the database" are valid: use the charter stage to inventory and decompose them rather than forcing the human to enumerate implementation details.

A 7000 result can prove defects, refute claims, or support a later human
promotion decision. It cannot remove a transition risk entry, make a suspect
product deployable, authorize an application placement, or substitute review
evidence for the required Build and 8000 paths.

## Run Sizing

Choose the smallest class that covers the connected surface. Preserve the
established role ceilings and total child cap, counting the manager in the Sol
ceiling. Runtime concurrency follows `$agent-run`.

| Class | Sol ceiling | Terra scout ceiling | Terra proof-executor ceiling | Total child cap |
|---|---:|---:|---:|---:|
| small | 3 | 2 | 1 | 8 |
| standard | 3 | 4 | 2 | 16 |
| large | 3 | 5 | 3 | 24 |

- Per-role ceilings constrain campaign sizing and admission; they do not
  override `$agent-run`.
- Scout assignments follow disjoint surfaces or lenses, never arbitrary file
  counts.
- Reruns, wave 2, and variant proofs count against the total delegate cap.
- Increasing the class after launch requires a ledger reason; exceeding `large` requires explicit human approval.

## First Act

1. Pin the target and record the exact source identity. A working diff must be captured as an immutable patch or commit before proof execution.
2. Mint `<surface-slug>-<YYMMDD>-<uid4>`.
3. Follow `$git-policy` to place the assigned worktree on
   `7000/<run-id>/trunk`.
4. Create `runs/7000/<run-id>/00-charter.md` and manager-owned `run-state.json`.
5. Create the manager goal per `$codex-goal-use`: point at the charter and
   guarantee registry, the run trunk, and the terminal condition. Keep
   guarantee rows, collapser applications, oracle refs, witnesses, and
   evolving run state in the referenced artifacts, not in the goal.
6. Run `$7005-review-charter-guarantees` through `$agent-run` under the
   logical `review_reasoning_lead` profile, then reconcile its proposed
   charter with the human before later assignments.
7. When the pinned target touches a plane governed by an adopted protocol
   (root Protocols; register: `architecture/protocols/AGENTS.md`), the
   charter names `$adopted-protocols` as a standing lens: stage leads run its
   four questions beside their rubrics, and a BLOCK enters the run as a
   confirmed-candidate finding routed to the human gate.
8. Post the initial charter and ledger so another manager can resume the run.

## Bounded Docket

Every child receives only:

```text
run_id
parent_goal_id
stage_skill
logical_role
model and max posture
target SHA and base/diff identity
selected campaign tracks
input artifact paths
exact output paths
assigned G/H/T/V ids or surface/lens
assigned collapser application refs or explicit none
$agent-run handoff fields and manager-owned follow-on budget
command/environment limits
Moon targets and development-environment capability/receipt/profile refs
terminal response contract
```

## Proof-Validation Authority

A Terra proof commit is a candidate, never an independently accepted result.
For every proof pair, only the independent 7036 validator emits the
proof-validation disposition in its existing domain vocabulary. The manager
may verify the target, proof commit, validation attempt, specification, and
evidence identities, then apply the integrity plan's predeclared mapping. It
may not reinterpret, substitute, suppress, downgrade, round, or adjudicate
the validator's disposition. Missing, stale, conflicting, or mismatched
validation has no claim-acceptance or guarantee-qualification edge; retain it
as failed validation evidence. Recovery uses the same validator against the
immutable proof candidate, or a repaired proof as a new candidate with fresh
validation.

A stage that identifies scout, executor, validator, or other follow-on work
returns a closed docket to the manager. The manager runs that role through
`$agent-run`; it never delegates the immediate critical-path decision it
needs next.

After every handoff, the manager records the `$agent-run` reference, logical
role, docket digest, output path, Passback, and terminal disposition in
`run-state.json` and the events segments.

## Stage Flow

1. **Charter and guarantees:** Sol authors `05-guarantees.md`, the authoritative `G-###` registry, proof obligations, counterexample observables, possibility classes, candidate existing applications, and coverage basis.
2. **Recon:** one Terra lead maps the surface, Moon targets and underlying
   commands, development-environment capability and execution profile, and
   each relevant instrument's owner, exact identity, configuration, controls,
   fidelity, gate posture, oracle, witness, and residual before returning
   bounded scout dockets to the manager.
3. **Triage and coverage:** one Sol lead accounts for every guarantee, recon claim, prior proof, collapser application, and coverage gap; mints `H-###` and proof packets carrying the application refs they reuse or attack.
4. **Test integrity:** when selected, one Sol lead maps tests to guarantees and designs mutations or counterexamples that should make each test fail.
5. **Integrity planning:** Sol leads attempt paper refutation, inspect full paths, and write deterministic proof specs.
6. **Proof execution:** run a Terra `$7035-review-integrity-execute` role
   through `$agent-run` for each surviving specification.
7. **Independent validation:** run a different Sol
   `$7036-review-proof-validation` role through `$agent-run` against that
   exact proof candidate.
8. **Evidence landing:** follow `$git-policy` to verify each commit's
   identity, diff scope, and ancestry, route
   the exact validator disposition through the immutable integrity plan's
   predeclared mapping, Lands the proof evidence and then the validation report
   onto the run trunk, and records the landed SHAs. It never lands
   product-source mutations or substitutes its own proof verdict.
9. **Gapfill:** one Sol lead compares the guarantee registry, inventories, tracks, and terminal evidence. At most one bounded wave 2 runs unless the human expands it.
10. **Root cause and variants:** one Sol lead consolidates reproduced failures,
    maps blast radius, and returns bounded Terra scout dockets. The manager
    runs those roles through `$agent-run`, then routes surviving `V-##`
    specifications through 7030, executor, and validator roles before resuming
    synthesis from mapped outcomes.
11. **Process feedback:** conditional; run only when feedback files exist.
12. **Mechanical sweep:** the manager verifies all ids are terminal, every reproduced finding has independent proof, every claimed collapser application has an exact terminal disposition, every blocked item has a precise blocker, and the manager checkout diff is limited to the run folder.
13. **Assurance retrospective:** standing on every campaign; after the sweep leaves every id terminal and wall-clock data complete, one Sol lead audits the cost of knowing for every terminal proof and packet from process artifacts only and writes `90-report/assurance-retrospective.md` with one bounded verdict per proof. Time-to-assure is a diagnostic, never a KPI; it never overturns a finding.
14. **Report:** one Sol lead compiles the run only after the sweep passes and folds in the assurance retrospective as a required section.
15. **Human gate:** set `complete-awaiting-promotion`, present guarantees, defects, test weaknesses, unresolved risk, budget cuts, recommended actions, and the retrospective's flagged standing questions. Never self-promote.

## Durable Recovery

After every handoff, recovery, proof publication, validation, and stage
publication, append the transition to the run's JSONL events segments
(`runs/7000/<run-id>/events/<epoch-ms>-<seq>-<actor>.jsonl`, single writer per
segment, closed segments immutable, rotate well under 5 MB) and update
`run-state.json` as a derived snapshot. The events segments are authority;
recovery re-folds them before trusting the snapshot, and no monolithic
append-only file is ever created. Record role, model, `$agent-run` handoff and
Passback identities, docket digest, outputs, immutable candidate, and next
action.

Use this stable top-level shape; extend nested records without renaming these keys. The historical `agents` and `spawned` labels mean logical delegates and total delegations respectively.

```json
{
  "schema": "assurance-review-run/v1",
  "run_id": "<run-id>",
  "status": "running",
  "target": {"sha": "<sha>", "base": "<sha-or-null>", "contract_refs": []},
  "tracks": [],
  "budget": {"class": "small|standard|large", "spawned": 0, "active_by_model": {}},
  "stages": {},
  "guarantees": {},
  "hypotheses": {},
  "test_obligations": {},
  "collapse_applications": {},
  "variants": {},
  "agents": {},
  "proofs": {},
  "next_actions": []
}
```

When resuming, read the ledger and committed run tree before chat history, then
follow `$agent-run` for any live or replacement child.

## Manager Mutation Boundary

The manager writes the charter, ledger, stage routing records, and mechanical
sweep. Git handling follows `$git-policy`; child execution follows
`$agent-run`. The manager never authors another role's analysis, proof,
validation, trace, or report and never mutates reviewed source.

## Completion

The active run is complete when:

1. every selected track and registered `G-###`, `H-###`, `T-###`, and `V-##` obligation is terminal;
2. all reproduced findings have independently validated proof commits;
3. the report is Posted;
4. the human gate was presented; and
5. the human promoted, explicitly deferred, or abandoned the run.

An unpromoted `complete-awaiting-promotion` run is a valid terminal manager session. It is not permission to archive or implement anything.
