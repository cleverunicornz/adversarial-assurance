# Sol Adversarial Review of PLAN.md

Reviewer: independent model (OMP / GPT-5.6-Sol, maximum reasoning), read-only.
Input: `PLAN.md` §1–§11 against the live pack at `skills/`.
Verdict: **UNSOUND** — the plan is directionally right but does not yet produce
a self-contained, runnable-in-any-repo pipeline. Executing it as written would
break every stage of the funnel.

## Critical findings (12)

1. **Engine replacement collapses two authorities.** `validate-record` is
   defined with only `init`/`check`/`resolve-oracles`, then maps `groundwork
   build` to an unimplemented command. `3020-contract-validation` assigns
   Groundwork YAML admission + packet-schema + coherence + pinned-reference
   resolution; `4020` adds deterministic task-world generation, immutable
   skeletons, digests, Closure carriage, drift detection. You cannot fold a
   packet compiler into a record validator. Python stdlib cannot parse general
   YAML — vendor a parser or enforce a restricted grammar. Conformance
   fixtures required for every output/refusal/digest invariant.
2. **The "runner" is prose, not a primitive.** `agent-run` demands native
   `spawn_agent`/`fork_turns="none"`; `codex-goal-use` demands `create_goal/
   get_goal/update_goal` and hard-blocks when absent; `1000-research` and
   `7000` hard-refuse mismatched model identities. The plan simultaneously
   says `agent-run` defines the runtime and that fixed model IDs stay.
   Required: a shipped harness-adapter contract (spawn, no-history handoff,
   goal lifecycle, status/reclaim, model roles, reasoning effort), fixed model
   names → configured logical roles, fail-closed identity preserved.
3. **Pack-root resources will not resolve from a consuming repo.**
   `customDirectories` installs only `skills/`; `tools/validate-record` would
   run relative to the consuming repo, not the pack. `.codex/tools/manifest
   .yaml → tool-manifest.yaml` is an inert filename, not a resolver. Required:
   one installed entrypoint with pack-root discovery (e.g. `aa`) + a bootstrap
   that materializes schemas/registries/run-law into the consumer
   (`aa record …`, `aa packet …`, `aa skill …`, `aa post …`).
4. **Binding authority documents are omitted; a scaffold can't replace them.**
   Skills bind normativity to `architecture/development-lifecycle/AGENTS.md`
   (collapse model, oracle seam, machine/prose planes, anti-rot),
   root `AGENTS.md` (git invariant, completion gate, closed-artifact law),
   `architecture/AGENTS.md` (promise charter), Actor Constitution,
   Reckoning/Situating. These are semantic owners — they must be absorbed
   wholesale (generic clauses), not summarized.
5. **Generic registry still leaves mandatory missing entries.** `platform-
   transition`, `buf`, `assurance-db`, object-storage risk entries are
   mandatory gates. A `registry/risk/AGENTS.md` + one template can't answer
   them. Required: drop source-instance assertions, preserve the law
   ("declared risk disposition must be resolved"), add an explicit
   `none registered` result + repo-init gate for consumer-authored entries.
6. **The shipped collapse graph is a source-repo inventory, not generic law.**
   It hardcodes Rust/protobuf/Svelte/Tauri surfaces, commissioned instruments
   ("wired in pcc-native"), the outgoing DB, Nix, Moon. Retain schema +
   evidence-rung semantics + routing law + "every possibility exactly once";
   drop the instance; ship an empty consumer-commissioned registry.
7. **Companion classifications are semantically wrong.** Object storage
   governs proof resources/credentials/lifecycle/exposure response (not just
   artifacts); development environment owns capability prep, tool pins,
   mutable roots, venue (not just a receipt). `7035` forbids substituting a
   runner for `rust-nextest`/`rust-afl-campaign` — "standard gate" wording
   loses that invariant. Playwright is a public proof-method door with real
   tiers in `collapse-graph`, not harness trivia.
8. **Schema inventory incomplete + inconsistent.** Nine schemas listed where
   the layout says eight; omits `assurance-claim/v1` (2000:68, 3010:289),
   `assurance-review-run/v1` (7000:224–241), and
   `doctrine-graph/schemas/skill-node-header.schema.yaml`
   (skill-authoring:102–121). Path rule maps `architecture/development-
   lifecycle/**` → `runbook/**` while schemas sit under `schemas/`. Every
   typed record/$id/reference/enum/template needs one canonical home + pos/neg
   fixtures.
9. **GitHub/main hardcoding + missing enforcement artifacts.** `git-policy`
   requires authenticated `gh` + GitHub APIs, hardcodes `main`, GitHub PRs,
   provider checks, an unshipped `scripts/post`, and an unshipped
   `.github/workflows/merge-gate.yml` (itself hardcoding `claude/**`). Any-repo
   requires a forge/default-branch adapter + shipped `post` + GitHub reference
   workflow + a Git-only reference backend.
10. **The advertised funnel contradicts the pack's own routing.** It is not a
    linear 1000→2000→3000→4000→5000/6000→7000. protocol-1000 routes closed
    knowledge → 6000 and uncertainty → 2000; protocol-2000 requires 6000
    promise work before 3000; protocol-4000 commissions none of 5000/6000/7000;
    7000 is human-invoked, never a completion step; git-policy says every edge
    is admissible ordering, never automatic commission. Plan must document an
    **admission graph**, not a funnel.
11. **Source-product content survives outside the inventory.**
    `adopted-protocols/controls/{known-good,known-bad}.md` carry Matrix/Flue/
    product architecture; `skill-authoring` reserves `llm_gateway.*` and
    `.agents`/`.claude`/`$CODEX_HOME`/`.codex` layouts; `2020` imposes Cargo
    `[workspace]`, `moon.yml`, Linux x86_64, Nix, fleet rules on every spike;
    `use-derived-state` depends on an unshipped "Nine" canon;
    `state-gauge` asserts current platform apparatus.
12. **Proposed validation can't see the real dangling set; docs go false.**
    Scans only paths + `$skill`/`target:`. Misses fixed historical
    `runs/1000/<id>` pointers (collapse-graph:258–263, git-policy:55–57),
    frontmatter edges, commands, env vars, model/profile refs, supporting
    files. Byte-parity cannot prove semantic fidelity after insert/delete/
    rewrite. README:51–54 and SANITIZATION's "consumers supply excluded
    skills" claims become false. Requires a sentence-level migration ledger
    (norm preserved / executor swapped / codebase drop).

## Missed dependency classes (beyond the 13 skills)

`spawn_agent`/`fork_turns`; `create_goal/get_goal/update_goal`; `scripts/post`;
`.github/workflows/merge-gate.yml`; `rg/fd/ast-grep` pins in `search`;
`skill-node-header.schema.yaml`/`doctrine-graph.md`/`compiler.md`;
lifecycle `AGENTS.md`/`reckoning.md`/`situating.md`; `architecture/AGENTS.md`
promise charter; `fleet-substrate/AGENTS.md`; Nine/state canon;
`runs/1000/<id>` historical pointers; `assurance-claim/v1` +
`assurance-review-run/v1` contracts; source-product calibration controls;
pack-root installation; hard model identities; Cargo/Nix/Moon spike rules;
GitHub-only forge API + default-branch hardcode; stale README/SANITIZATION
claims; `tool-manifest.yaml` as non-resolver.

## Direct answers

- **Q1 (vendor vs spec-only):** Vendor tools AND spec. Ship faithful reference
  implementations + conformance fixtures, responsibilities kept distinct.
- **Q2 (companion boundary):** No. Absorb object-storage and dev-env as
  complete standalone skills; absorb research trio as one `repo-research`;
  genericize Moon as a real task-graph adapter; vendor nextest/AFL/Playwright
  exact procedures or delete their graph nodes; drop Tauri + DB/Nix/gateway
  source authority, keeping a generic scoped-authority rule with explicit
  `none configured`.
- **Q3 (executor swap vs fidelity):** Executor substitution is allowed — but
  `groundwork → validate-record` is invalid because it changes the act
  (packet compile ≠ record check). Neutral executor must preserve Groundwork
  semantics and prove equivalence with fixtures.
- **Q4 (scaffold enough?):** No. Bundle the full generic authority: laws
  (root completion/closed-artifact/protocol/state, lifecycle Actor
  Constitution/venue/evidence ladder, Closure, work packet, Reckoning,
  Situating, 7000 run law, promise plane), every record schema + projection,
  neutral adopted-protocol controls, registry empty-state behavior.
- **Q5 (what §4 missed):** the full missed-dependency table above; the
  uncounted non-skill dependency surface is the real failure.

## Adjusted treatment matrix (summary)

Absorb: research trio → `repo-research`; object storage → standalone generic
skill; dev-env → standalone execution-environment skill; root/lifecycle/runs/
promise laws wholesale. Genericize: Moon → task-graph adapter; harness/goals/
profiles/models → explicit configuration; git/forge/default-branch → adapter;
state canon → general law only; collapse graph → schema + consumer-owned
instance; search/proof tools → declared capabilities. Vendor: record engine,
packet compiler (separate), scoped skill validator, `post`, complete schemas +
fixtures, YAML support. Drop: Tauri/native-shell instance, DB/Nix/gateway
source authority, source risk entries, historical run citations, source-current
state assertions, Matrix/Flue controls (replace with synthetic neutral pair).
Rewrite: README + SANITIZATION after the transformation (their current claims
become false) and add a sentence-level migration ledger.

## Top 5 actions

1. Replace §4–§5 with an exhaustive dependency-and-authority graph
   (commands, harness APIs, model roles, paths, schema $ids, frontmatter
   edges, supporting files, historical pointers, source-instance assertions)
   with a sentence-level preserve/swap/drop rationale.
2. Define the bootstrap + runtime contract before writing validators:
   one installed CLI (`aa`), deterministic pack-root discovery, repo
   initialization, harness/goal adapter, model-role mapping, forge/default-
   branch adapter, explicit capability preflight.
3. Extract and vendor the full normative substrate + faithful engines
   (laws, schemas, record validator, packet compiler, skill validator,
   `post`, YAML support, pos/neg conformance fixtures).
4. Delete source-repository state, then initialize neutral consumer-owned
   registries (collapse instance, risk entries, Nix/Moon/fleet topology,
   product controls, state-status assertions, historical runs) preserving
   their general gates and evidence laws.
5. Prove a fresh-repository run end to end: bootstrap an empty non-Rust repo,
   execute representative 1000→2000→6000→3000→4000 gates + a separately
   commissioned 7000, validate records and refusals, scan every reference
   class, then update README/SANITIZATION from observed behavior.

## Implication for scope

Genuine self-containment is a re-hosting of the pipeline: pack-owned engines,
the full normative substrate, harness + forge adapters, and consumer-owned
registries — not a scrub. The "no semantic rewrite" constraint is preserved by
Sol's framing: norms are retained verbatim; executors and source instances
are swapped/dropped, each entry recorded in a migration ledger. This is a
multi-session body of work, and the plan must be rewritten to this shape
before any skill file is touched.
