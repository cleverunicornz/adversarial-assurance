# Generalization Plan — adversarial-assurance as a self-contained pipeline template

Status: **PLAN — not yet executed.** Written for adversarial review by an
independent model before any transformation work begins.

## 1. Goal

Turn the extracted `adversarial-assurance` skill pack into a **general,
self-contained pipeline template** that can be run in **any repository** using
only the pack itself plus a standard agent environment. No pointer may resolve
into `yeet-code` or any other external repository. No yeet-code-internal
binary, schema path, registry document, or companion skill may be required.

The standardized funnel stays intact: **research (1000) → spike (2000) →
contract (3000) → proof (4000) → validation (5000/6000) → adversarial review
(7000)**, with protocol gates and evidence records throughout.

## 2. Principles (the fidelity contract)

All transformation work is bound by these four rules:

1. **Normativity is preserved.** Every normative claim — gates, records,
   ledgers, ordering, criteria, verdict dispositions — is retained verbatim.
2. **Executor, not meaning, changes.** Where a sentence names a yeet-specific
   engine or path, the *executor* is replaced by a pack-owned neutral
   mechanism; the *act* it performs is unchanged.
3. **Absorb; don't point.** If generalizable context from an excluded skill is
   needed by the pipeline, that context moves **into** the pack. Nothing is
   left as a pointer.
4. **Drop what is codebase-relational.** Content that exists only because of
   the source codebase (its services, its registries, its hostnames, its
   bespoke product surfaces) is removed, along with the sentences that can
   only mean something inside that codebase. Loss here is intended and
   directed — it is the price of generality, and it is the only permitted
   removal.

The test of the contract: a reader who knew only the pack (not `yeet-code`)
can still perform every action the pipeline instructs, using standard tooling.

## 3. Target environment (what the template may assume)

- An agent harness (OMP) with a sub-agent runner primitive (the pack's own
  `agent-run`).
- A Git repository the agent works in.
- Standard CLI tools: `git`, `gh`, `cargo`/`rustc`, `python3`, `sh`.
- No other binaries. Any gate the pipeline requires is implemented by the
  pack's own vendored tooling (stdlib-only) or by documented repo-local
  convention.

## 4. Dependency inventory (what currently blocks self-containment)

Four classes of external dependency were surfaced by the completeness pass:

| Class | Members | Why it blocks |
|---|---|---|
| A. Excluded companion skills (13) | `assurance-db-qualification-ops`, `assurance-nix-ops`, `llm-gateway-ops`, `moon-task-graph`, `tooling-object-storage`, `development-environment`, `external-repo-bank`, `external-source-research`, `direct-repo-scout`, `playwright-cli`, `rust-afl-campaign`, `rust-nextest`, `tauri-linux-webdriver-host` | cited as `$skill`/`target:` but not shipped |
| B. Bespoke engine binaries | `assurance-runs-validator`, `assurance-skill-validator`, `groundwork` | skills comm~ the agent to run them (`cargo run -p …`, `groundwork check/build`) |
| C. Schema + registry files | `schemas/*.schema.yaml` (contract, run-event, campaign-frontmatter), `work-packet(.v2).schema.yaml`, `work-packet.md`, `closure.md`, `reckoning.md`, `architecture/protocols/AGENTS.md`, `architecture/risk/*/AGENTS.md`, `runs/AGENTS.md` | loaded by path; live in `yeet-code` root, not in the pack |
| D. Harness / layout conventions | `.codex/tools/manifest.yaml`, `runs/<N>/**` evidence trees, `spikes/`, `$ASSURANCE_GOAL_DOCKETS`, `$HOME/data/goal-dockets/` | `.codex/` is Codex-specific; rest are conventions the pack must own |
| E. Runtime contract | logical profiles (`agent-code`, `agent-manager`, `agent-validation`, `spike_worker`, `review_*` leads), models `gpt-5.6-sol/terra` | not repo pointers — runner config; retained |

## 5. Resolution matrix

For each class: **Absorb** (move context in), **Genericize** (rewire to a
pack-owned neutral mechanism), **Vendor** (ship the generic artifact inside the
pack), or **Drop** (codebase-relational only).

### A. Companion skills (13)
| Skill | Treatment | Rationale |
|---|---|---|
| `external-repo-bank`, `external-source-research`, `direct-repo-scout` | **Absorb** → fold their generalizable context into the pack's `search` skill (persistent deduped clone bank, pull-before-use, read-only reference). Three private names collapse to one neutral `repo-research` behavior. | Generalizable research hygiene, useful in any repo. |
| `tooling-object-storage` | **Absorb →** neutralize as `artifact-object-storage`: generic object-storage copy/verify discipline for release artifacts (no secret paths, no specific provider). | Generalizable; referenced as mandatory gate by 3000/4000. |
| `development-environment` | **Absorb** → neutralize as `environment-receipt`: repo-local dev-environment receipt fields for reproducible proof execution. | Referenced by 4000-series execution; its receipt concept is generic. |
| `moon-task-graph` | **Genericize** → drop the Moon-specific name; the pack defines a neutral `task-graph` primitive ("the repo's declared build/test graph"). | Moon is the source repo's build system; only the "run the declared graph target" idea is needed. |
| `rust-afl-campaign`, `rust-nextest` | **Drop** context, keep the *trigger* lines generalized: "when the target includes fuzz/test claims, apply the standard integrity gate." | Rust-specific tooling details are not pipeline semantics. |
| `playwright-cli`, `tauri-linux-webdriver-host` | **Drop**. | Harness-specific browser tooling; not pipeline semantics. |
| `assurance-db-qualification-ops`, `assurance-nix-ops`, `llm-gateway-ops` | **Drop** + generalize the citing sentences: "when the target includes DB/Nix/gateway-operational claims, the consumer's own registries govern." The specialized `$load` edge disappears; the conditional requirement stays as a standard-gate clause. | Strictly codebase-relational — the epitome of "only relational to the code we stripped it from." |

### B. Engine binaries (the crux)
Decision: **Vendor a pack-owned neutral engine; reword every invocation.**
- The pack ships `tools/validate-record` — a dependency-free (stdlib
  `python3`) implementation of the record grammar the pipeline gates on:
  `init` (scaffold charter/ledger/events), `check` (ledger, review, and event
  planes), `resolve-oracles` (addressability). It validates structure and
  emits deterministic pass/fail; it does **not** judge prose.
- Mechanical rewording (normativity preserved), applied to every invocation:
  - `cargo run -p assurance-runs-validator …` → `tools/validate-record …`
  - `…assurance-runs-validator check …` → `tools/validate-record check …`
  - `…assurance-runs-validator resolve-oracles …` → `tools/validate-record resolve-oracles …`
  - `cargo run -p assurance-skill-validator …` → `tools/validate-skill …` (pack-owned)
  - `groundwork check` / `groundwork build` → `tools/validate-record check …` / `tools/validate-record build …`
- The pack also ships `tools/validate-skill` (skill-source structural check)
  — the `skill-authoring`/`5020` gate.
- Fallback stance (documented in the pack for repos that adopt the
  conventions): if the consumer prefers its own implementation, the record
  grammar in `schemas/` is the contract; the pack's tool is the reference
  implementation. Fidelity does not depend on which executable runs the gate —
  only that the gate runs per the grammar.

### C. Schema + registry files
Decision: **Vendor the generic artifacts; scaffold the registries generically.**
- Bundle (verbatim, token-scrubbed): `schemas/contract-frontmatter.schema.yaml`,
  `schemas/run-event.schema.yaml`, `schemas/campaign-frontmatter.schema.yaml`,
  `schemas/work-packet.schema.yaml` (+ `work-packet-v2`), `schemas/closure.schema.yaml`,
  `schemas/closure-event.schema.yaml`, `schemas/review-frontmatter.schema.yaml`,
  `schemas/spike-frontmatter.schema.yaml`.
- Bundle the generic process docs: `development-lifecycle/` → pack-owned
  `runbook/lifecycle.md` (work-packet, closure, reckoning semantics) as the
  canonical generic statements. The yeet-specific registries
  (`architecture/risk/*` entries for llm-gateway, lane-epoch, fleet, flue) are
  **not** reproduced.
- Scaffold a neutral registry layout the pack owns and any repo can adopt:
  - `registry/protocols/AGENTS.md` (replace `architecture/protocols/AGENTS.md`)
  - `registry/risk/AGENTS.md` + one template entry (replace `architecture/risk/…`)
  - `runs/AGENTS.md` (the evidence-tree law the 7000 loader binds to)
- Reword path references mechanically: `architecture/protocols/AGENTS.md` →
  `registry/protocols/AGENTS.md`; `architecture/risk/…` → `registry/risk/…`;
  `architecture/development-lifecycle/…` → `runbook/…`; `schemas/…` stays.

### D. Harness / layout conventions
- `.codex/tools/manifest.yaml` (Codex-specific) → neutral `tool-manifest.yaml`
  at pack root; the pack ships one declaring the standard contract
  (validate-record, validate-skill, git, gh, cargo). Reword the four reference
  sites.
- `runs/<N>/<unit>/**`, `spikes/`, `run-state.json`, `promotion/package.md` →
  keep as the pack's record convention (no external dependency; any repo can
  create them). Document once in `runs/AGENTS.md`.
- Env: `$ASSURANCE_GOAL_DOCKETS` / `$HOME/data/goal-dockets/` → keep (generic
  location convention, pack-owned name).

### E. Runtime contract
- Retained unchanged. Logical profiles are defined by the pack's `agent-run`;
  `gpt-5.6-sol/terra` are public OpenAI model identifiers (not personal data,
  not repo pointers). The template documents them as *role requirements*
  (a reasoning-heavy lead, a terra/executor) with configurable model ids.

## 6. Target repository layout (proposal)

```
adversarial-assurance/
  PLAN.md, README.md, SANITIZATION.md
  skills/          # 81 skills, references rewired per matrix above
  schemas/         # 8 bundled record/schema files
  runbook/         # generic lifecycle docs (work-packet, closure, reckoning)
  registry/        # protocols + risk scaffolds + runs law
  tools/
    validate-record   # pack-owned stdlib-only record gate
    validate-skill    # pack-owned stdlib-only skill gate
  tool-manifest.yaml  # neutral command registry (replaces .codex/tools)
```

## 7. Mechanical transformation rules (applied without semantic rewrite)

Ordered renames, all token-level, applied across the whole tree:

1. Engine/executor swaps (Section 5B table).
2. Path swaps (Section 5C table) and `.codex/tools/manifest.yaml` →
   `tool-manifest.yaml`.
3. Companion-skill edge removal: delete `target:`/`$x` edges to the 5 dropped
   skills; generalize those citing sentences per the standard-gate clause.
4. Absorbed-context insertions: fold generic content of the 5 absorbed
   companions into their target skills.
5. Re-run the full zero-token sanitization scan and the byte-level fidelity
   check; re-run adversarial validation.

## 8. Non-goals (explicitly out of scope)

- Reproducing `yeet-code`'s risk register contents, service architecture, or
  product surfaces.
- Rewriting prose claims or normative dispositions ("PASS/FAIL/BLOCKED"
  semantics, gate ordering, evidence laws).
- Generalizing away the funnel itself (the 1000→7000 sequence is the product).
- Adding new functionality not already implied by the source.

## 9. Acceptance criteria

1. **Zero external pointers**: no path or `$skill`/`target:` reference in the
   pack resolves outside the pack (verified by scan; every reference maps to a
   shipped file or a documented standard tool).
2. **Zero residual identifiers**: full-tree token scan clean (owner tokens,
   hostnames, providers, SSH forms, secrets).
3. **Gates runnable**: `tools/validate-record` and `tools/validate-skill`
   execute on a bare environment (python3 stdlib) and pass/fail per grammar on
   fixtures.
4. **Fidelity audit**: every normative sentence survives; diff between pre- and
   post-generalization is exactly the mechanical renames + absorption +
   documented drops (auditable against this plan).
5. **Cross-reference integrity**: all `target:`/`$skill` edges resolve within
   the pack (no dangling set other than none).

## 10. Execution phases

1. Author pack-owned tools (`validate-record`, `validate-skill`) + schemas +
   registry scaffold (new files only — no source edits yet).
2. Apply mechanical renames (Section 7) to the 81 skills.
3. Absorb companion context (5 skills) and drop codebase-relational edges
   (Section 5A).
4. Re-sanitize + re-verify (byte-level + zero-token + full-tree scan).
5. Adversarial validation pass (OMP / gpt-5.6-sol, max).
6. Human review gate — no commit/push until review.

## 11. Open questions for adversarial review

- Q1: Is "vendored pack-owned tools + executor rewording" (5B) the right
  fidelity-preserving answer to the engine dependency, or should the template
  be spec-only (consumer implements the grammar)?
- Q2: Is the absorption-vs-drop boundary correct for the 13 companions —
  specifically, should `tooling-object-storage` and `development-environment`
  be absorbed (as proposed) or dropped?
- Q3: Is rewording `cargo run -p assurance-runs-validator …` →
  `tools/validate-record …` a violation of the no-semantic-rewrite constraint,
  or is it the correct executor-swap under Principle 2?
- Q4: Does the pack-owned `registry/` + `runbook/` + `runs/AGENTS.md` scaffold
  provide enough ground truth for the 7000 review loader, or must more of the
  lifecycle documentation be bundled?
- Q5: Are there additional intra-pack cross-references or commands the
  inventory missed (i.e., what did the plan fail to surface)?
