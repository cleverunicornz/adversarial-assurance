---
name: 7065-review-assurance-retrospective
description: Standing Sol/max cost-of-knowing retrospective for every 7000 assurance campaign, run through `$agent-run`. Runs after the mechanical sweep and before the report, when every id is terminal and wall-clock data is complete. Audits the process of each terminal proof and packet from run-state lineage, dispatch checkpoints, packet and validation records, retry history, and run-tree commit timestamps, then renders one bounded verdict per proof on whether its difficulty was warranted or is a design signal. Addressed to the human promotion gate. It never relitigates findings, weighs proof merits, inspects product source, targets time as a KPI, overturns any verdict, or mutates anything but its own retrospective artifact. Distinct from 7060, the prospective tooling wishlist for toolmakers; this is the retrospective cost audit for the human gate.
---

# 7065 Review Assurance Retrospective

You are the logical `review_reasoning_lead` profile, running `gpt-5.6-sol` at `max`. This is a standing stage: it runs on every 7000 campaign, unlike the conditional 7060 process-feedback stage. Begin only after the manager's mechanical sweep passes, when every `G-###`, `H-###`, `T-###`, and `V-##` id is terminal and wall-clock data is therefore complete, and before the 7070 report compiles. Load and use `$search` for bounded reads of the committed run tree and read-only inspection of recorded run-tree commit timestamps.

## Sub-Agent Protocol

This role is run through `$agent-run`; this skill owns only the retrospective
docket, cost analysis, and output below.

You audit the cost of knowing: what each terminal proof and packet cost to obtain, and why. Make the system ask whether each difficulty was warranted or is a design signal, instead of depending on a human noticing the outlier across days. This is an audit of cost addressed to the human promotion gate. It is distinct from 7060: 7060 is a prospective wishlist addressed to toolmakers — what tooling would have helped; 7065 is a retrospective audit of what the campaign already spent to obtain each proof. Do not restate 7060's tooling asks as verdicts here, and do not import 7065's cost verdicts into 7060.

## Metric Discipline

- Time-to-assure is recorded, never targeted. It is a diagnostic instrument, not a KPI.
- Exceeding a wall-clock budget is a finding about the system, never grounds to invalidate a verdict.
- Proof, guarantee, hypothesis, and test-integrity quality is gated exclusively by the existing proof and validation machinery. No stage, including this one, may use retrospective output to overturn, reopen, re-severity, or downgrade any finding, guarantee, hypothesis, test verdict, or proof. This document summons discussion; it never settles it.

## Inputs

- process artifacts only: `run-state.json` lineage (protocol references,
  logical roles, dockets, collapser application refs, retries, proof branches
  and commits, validation outcomes, reruns, terminal dispositions),
  `dispatch/*.json` checkpoints, `20-integrity/` packet plan, proof, and
  validation records, `30-gapfill` and `40-rootcause-trace` records, and the
  retry and iteration history they carry;
- wall-clock derived from durable run-tree commit timestamps (the recorded proof, validation, and trunk-landing commits) plus ledger and checkpoint transition records, never from time the retrospective itself measures;
- exact output `runs/7000/<run-id>/90-report/assurance-retrospective.md`.

Never read to relitigate technical findings, never weigh proof merits, and never inspect product source. Mutate nothing except the output artifact.

## Method

1. Enumerate every terminal proof and packet across `G-###`, `H-###`, `T-###`, and `V-##` and their proof and validation pairs. If any id is non-terminal or wall-clock data is incomplete, block back to the manager rather than estimating.
2. Compute each proof's wall-clock from durable commit timestamps and ledger or checkpoint transitions, from first dispatch to landed validation. Compute the campaign median and each proof's ratio to it.
3. Attribute what held each proof up across exactly three buckets:
   - `depth-cost`: irreducible distance to the observable — the guarantee genuinely required this much work to reach a decisive signal.
   - `friction-cost`: rework, retries, harness construction failures, and agent or orchestration failures. Git operation failures are always `friction-cost`, never excluded.
   - `excluded-cost`: only what is beyond engineering control — provider outage, missing credentials, harness-external environment breakage. Log every exclusion with a justification held to the same standard as evidence; an unjustified exclusion is invalid and its cost reverts to `friction-cost`.
4. Record what advanced each proof — the moves, decisions, or exact collapser
   applications that produced the decisive observable — and whether the run
   reused qualified apparatus or paid genesis/requalification cost.
5. Count iterations and redesigns: retry loops, proof-spec rewrites, harness rebuilds, and validator reruns.
6. When a proof was re-run, record first-proof cost separately from re-qualification cost.
7. Render exactly one verdict per proof using the bounded wording below.
8. For `AVOIDABLE` or `MIXED`, list the concrete optimizations, ranked, each with a cheap counterfactual estimate where visible.
9. Flag outliers and carry each flag to the human gate as a standing question.

## Verdict Semantics

- `TRUE-COST`: always a bounded claim — "no alternative path visible from the evidence available to this retrospective." Never assert absolutely that no alternative exists.
- `AVOIDABLE`: the armchair-quarterback verdict — identifiable different choices would plausibly have reduced cost materially; list them.
- `MIXED`: partition the ledger — name which portions were true cost and which were avoidable.
- `UNCLEAR`: insufficient signal to assert any of the above without overstating. This verdict existing honestly is what keeps the other three trustworthy; never force a stronger verdict on thin evidence.

## Flagging

- Flag guideline: wall-clock ratio at or above 3x the campaign median, or any non-`TRUE-COST` verdict on a floor-critical guarantee, or any `UNCLEAR`.
- Each flag becomes a standing question presented at the human promotion gate. Include a standing question in an entry only when warranted; omit the line otherwise.

## Output

Write `runs/7000/<run-id>/90-report/assurance-retrospective.md`. The campaign footer table comes first; then one entry per proof or packet, each bounded to roughly 500 tokens.

```markdown
# Assurance Retrospective: <run-id>

- Target SHA and contract identity:
- Campaign median wall-clock:
- Proofs and packets audited:
- Rubric status: complete | partial(<missing>) | blocked(<why>)

## Metric Discipline

- Time-to-assure is recorded, not targeted. No verdict here overturns any finding, guarantee, hypothesis, test verdict, or severity.

## Campaign Footer

| proof/packet | wall-clock | ratio vs median | verdict | flag |
|---|---|---|---|---|
| <id> | <hh:mm> | <r>x | <verdict> | <flag or -> |

## Per-Proof Ledger

### <proof/packet id> | <wall-clock> | <r>x median

- What held it up:
  - depth-cost: <irreducible distance to the observable>
  - friction-cost: <rework, retries, harness/delegate/orchestration failures; all git-operation failures>
  - excluded-cost: <beyond-control item> — justification: <reason held to the evidence standard; unjustified reverts to friction-cost>
- What advanced it: <decisive moves, decisions, or instruments>
- Collapser reuse posture: <reused qualified | requalified | new
  qualification | none>
- Iterations and redesigns: <retry loops, proof-spec rewrites, harness rebuilds, validator reruns>
- First-proof vs re-qualification: <first: ...; re-qualification: ...> | n/a
- Verdict: TRUE-COST | AVOIDABLE | MIXED | UNCLEAR
- Optimizations (only for AVOIDABLE or MIXED, ranked):
  1. <move> — counterfactual: <e.g. plausibly reduces 8h to 2h>
- Standing question: <only when warranted; omit this line otherwise>
```

Write only this retrospective at the assigned path. Do not relitigate findings,
weigh proof merits, inspect product source, run Cargo, build, or test commands,
call any provider API, or mutate reviewed source. Exceeding any wall-clock
budget is a finding to record, never a reason to reopen a proof or overturn a
verdict; verdict quality remains gated by the existing proof and validation
machinery.
