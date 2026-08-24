# 7000-series normalization fidelity record

Scope: the 15 one-level `SKILL.md` files from the campaign orchestrator through
promotion. This record states the semantic contract preserved and the
repository-specific mechanics replaced.

## Preserved procedure

- Human invocation is the sole admission authority. The campaign is never
  automatic and never a completion dependency.
- Stage order and duties remain: charter/guarantees, recon, triage,
  detector-integrity, integrity planning, proof execution, independent proof
  validation, evidence integration, one bounded gapfill wave, root-cause and
  variants, conditional feedback, mechanical sweep, standing retrospective,
  report, and human promotion.
- Every gate fails closed. Campaign Oracles use `PASS`, `FAIL`, or `BLOCKED`;
  finer domain dispositions remain verbatim in record bodies.
- Identifier taxonomies remain stable: `G-###`, `H-###`, `T-###`, `D-##`, and
  `V-##` (plus packet/wave/action ids where their stages already used them).
- Evidence remains pinned, digest-bound, replayable, and committed immutably.
  Proof authors and independent validators remain disjoint.
- Review roles never fix reviewed source. Temporary falsification mutations
  live only in disposable rigs and evidence commits.
- Report and promotion terminal accounting, pruning records, retained risk,
  action structure, and the final human gate remain intact.

## Replaced mechanics

- Fixed model, harness, runner, and actor names became explicit init variables.
- All role launches now go through the bound harness with closed dockets, no
  conversation history, typed Passback, unique output ownership, and
  manager-owned follow-on authority.
- Repository git policy, search/build graphs, test/fuzz/browser tooling,
  environment services, and operational runbooks are optional
  consumer-provided capabilities. PRs remain human-merged.
- Sibling-pipeline contracts and assurance artifacts became optional target
  repository contracts and records. A campaign reviews a pinned surface
  regardless of what produced it.
- Fixed risk and architecture paths became the target repository's registers
  when present.
- Campaign state moved to `.assurance/runs/<run-id>/` using Run, Promise,
  Witness, and Oracle YAML-LD records. Prose uses `body: |`; Witnesses resolve
  to real repo-relative artifacts with SHA-256 and provenance.
- Side ledgers and ad hoc state snapshots were removed. Git history plus
  committed YAML-LD records are durable recovery authority.
- CI runs `assurance check` and `assurance build`; its log is the witness.
  Local invocation remains optional authoring preflight only.

## Variable contract

Syntax: `{{variable}}`. The word `variable` denotes the syntax in explanatory
text; it is not a binding name.

Canonical closed set, vocabulary version 2:

```text
{{lead_model}}
{{executor_model}}
{{validator_model}}
{{harness}}
{{witness_runner}}
{{reviewer_seat}}
{{final_validator_seat}}
```

The adopting agent asks the human one grouped question and writes exactly
these values under `variables:` in `.assurance/assurance-init.yaml`. Pack
repository and immutable commit pin remain machine-resolved fields outside the
variable set. Every skill resolves its variables before acting; silent
substitution is forbidden.

`assurance check` enforces this contract under `A001`:

- every canonical variable must exist, be non-empty, and contain no bootstrap
  placeholder before status can be `CONFIGURED`;
- the init file may not invent variables;
- every `{{variable}}` reference found in a YAML-LD record must belong to the
  canonical set and be declared/configured in the init file.

## Per-skill changes

- `7000-code-review-orchestrator` — replaced fixed runtime/spawn/state machinery
  with explicit variables, bound-harness dockets, YAML-LD recovery, and the CI
  witness while retaining intake, sizing, 15-stage flow, authority, and human
  completion gate.
- `7005-review-charter-guarantees` — retained `G-###` decomposition,
  falsification observables, completeness, and scout return; emits a coherent
  charter Promise/Witness/Oracle bundle.
- `7010-review-recon` — retained connected-surface and apparatus inventory,
  risk separation, disjoint scouting, and coverage routing; emits a recon
  Witness bundle.
- `7011-review-recon-scout` — retained bounded full-path discovery,
  counterexample/referral discipline, and no-proof boundary; emits one
  digest-bound Witness.
- `7020-review-triage` — retained complete accounting, mechanism-only dedupe,
  `H-###` minting, ranking, budget cuts, and packet formation; emits a triage
  Promise bundle.
- `7025-review-test-integrity` — retained detector-to-guarantee mapping,
  faithful break design, controls, `T-###`, and independent sensitivity proof;
  emits a detector-integrity Promise bundle.
- `7030-review-integrity-plan` — retained paper attack, exact apparatus
  qualification, immutable specs, total validation mapping, closed proof pair,
  and no-adjudication boundary; emits packet Promises.
- `7035-review-integrity-execute` — retained exact-spec execution, disposable
  mutations, replayable rigs, receipts, immutable evidence scope, and
  executor-observation-only authority; emits proof Witnesses.
- `7036-review-proof-validation` — retained author-independent replay, fidelity
  and diff checks, raw domain disposition authority, and never-repair rule;
  emits validation Oracles.
- `7040-review-gapfill` — retained guarantee-by-possibility-by-application
  audit, terminal residue, bounded scouting, one second wave, and explicit
  budget risk; emits a gapfill Promise bundle.
- `7050-review-rootcause-trace` — retained three-suspect adjudication, `D-##`
  grouping, blast radius, two-pass `V-##` hunt, immutable mapping, and
  read-never-confirms boundary; emits defect Oracles.
- `7060-review-feedback-synthesis` — retained conditional dedupe,
  classification, smallest-unblock proposals, and evidence-loss accounting;
  emits a feedback Witness bundle.
- `7065-review-assurance-retrospective` — retained standing cost audit,
  diagnostic-not-KPI discipline, three cost buckets, bounded verdicts, flags,
  and no-technical-overturn boundary; emits a retrospective Oracle.
- `7070-review-report` — retained evidence admissibility, terminal guarantee
  certificate, defect/test/risk/coverage structure, retrospective carriage,
  actions, and no-upgrade rule; emits the final report Oracle.
- `7080-review-promotion` — retained explicit human authority, immutable
  lineage, curated evidence/pruning, ACTION structure, semantic routing,
  retirement, and no-self-commission boundary; emits the promotion Oracle.
