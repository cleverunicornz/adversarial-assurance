# adversarial-assurance

A repo-agnostic adversarial assurance pipeline for agent work. Adopt it into
any repository: agents run the assurance funnel, every act is recorded as a
YAML-LD document, and CI — through the `assurance` checker — is the
deterministic witness that validates the records and compiles each run into
one ingestible graph. The output is a machine-verified **lineage** of what
was done, by whom, judged how, against what evidence.

Four parts, one product:

1. **The skill pack** (`skills/`, 15 skills) — the 7000-series adversarial
   review campaign: the orchestrator (`7000-code-review-orchestrator`) and
   its thirteen stage roles (charter guarantees, recon + scout, triage,
   test-integrity, integrity plan/execute, proof validation, gapfill,
   root-cause trace, feedback synthesis, retrospective, report, promotion).
   Companion skills the stages reference (the agent runner, git policy, the
   funnel stages under review) are consumer-provided and bound through the
   init file — not shipped here.
2. **The record language** (`schema/`) — a closed YAML-LD vocabulary. Every
   act is a **Promise** (the judgeable commitment), backed by **Witnesses**
   (digest-bound evidence), judged by an **Oracle** (`PASS` / `FAIL` /
   `BLOCKED`), inside a **Run**.
3. **The checker** (`assurance/`, Rust) — `assurance init`, `check`, and
   `build`: validates the closed vocabulary, record law, and run coherence;
   compiles each run into one byte-stable TriG graph.
4. **The witness workflow** (`workflow/`) — CI builds the checker at a
   pinned commit of this repository and runs check + build on your runner.
   The workflow log is the attestation. Validation runs before anything
   merges; agents never self-attest.

## The model

| Layer | Location | Authority |
|---|---|---|
| Pack | this repository | vocabulary, schema, checker, workflow template, bootstrap procedure, skills |
| Adoption | `.assurance/` in a consuming repository | materialized schema, run records, registry state, bindings |
| Binding surface | `.assurance/assurance-init.yaml` | models by role, harnesses, witness runner, actor seats, immutable pack pin |

**Skills point; the init file binds.** Skill text is generic — it never
hardcodes a model, harness, runner, or actor. Where a skill needs to know
*who* to launch, it points at the init file, which the adopting agent writes
once by asking the human. Change the reviewer or the models: edit one file,
commit, done.

**Closed vocabulary.** Nouns (`Promise`, `Witness`, `Oracle`, `Run`) and
verbs (`witnessed_by`, `judged_by`, `resolves_to`, `part_of`,
`succeeded_by`) are fixed in `schema/vocabulary.yaml`. Agents may not mint
nouns, verbs, or fields; additions require a versioned schema and checker
change. The checker enforces this — that is its primary job: structure stays
put while judgment stays free.

**Prose lives in the records, not the graph.** Records are YAML-LD mapped 1:1
to JSON-LD 1.1 through one offline context, and may carry free-text bodies
(`body: |` / `body: >`) — declarative statements inside a typed graph node.
Compilation never inlines prose: it emits a `resolves_to` edge to the source
record plus a SHA-256 `content_digest`. Anyone — agent or human — can ingest
a run's TriG graph, see every relationship, and load only the pointed-at
record when they want depth.

**CI is the sole witnessed checker seat.** Manual `check`/`build` is allowed
as authoring preflight, but only the workflow log is evidence. Repository
rules must require the workflow's `assurance-required` job; a workflow file
by itself cannot prevent merges.

## Quickstart — adopt into any repository

```sh
# from a checkout of this pack, at the commit you want to pin:
cargo build --locked --manifest-path assurance/Cargo.toml

# install the adoption skeleton into your repo (creates .assurance/ and
# .github/workflows/assurance.yml, leaves bindings UNCONFIGURED):
assurance/target/debug/assurance init /path/to/your-repo
```

Then the adopting agent follows [`bootstrap/README.md`](bootstrap/README.md):
ask the human once — which model fills each role (lead, worker, validator,
reviewer), which harness launches them, which runner label hosts the CI
witness, who holds the final-validator and reviewer seats — and write the
answers into `.assurance/assurance-init.yaml` (`status: CONFIGURED`).

From then on:

```sh
assurance check /path/to/your-repo     # fail-hard validation (see below)
assurance build /path/to/your-repo     # compile runs/ into graph.trig
```

Agents author records under `.assurance/runs/<run>/`:

```text
.assurance/
  assurance-init.yaml
  registry.yaml
  schema/
  runs/
    <run>/
      run.yamlld
      promises/*.yamlld
      witnesses/*.yamlld
      oracles/*.yamlld
      graph.trig                # generated, byte-stable
```

Commit the records; CI builds the checker at the pinned pack commit, runs
check + build on the bound runner, and the workflow log is the attestation.

## The checker

The crate and binary are both named `assurance`. From this repository root:

```sh
cargo build --locked --manifest-path assurance/Cargo.toml
cargo test --locked --manifest-path assurance/Cargo.toml
```

- `assurance init [DIR]` — install `.assurance/` and the witness workflow;
  leave bindings explicitly `UNCONFIGURED`; print the adopting agent's
  bootstrap instructions.
- `assurance check [DIR]` — validate bindings, canonical schema copies,
  closed vocabulary, YAML/JSON-LD shape and formatting, run layout, vertex
  and file edges, Witness digests, actor seats, and successor legality.
- `assurance build [DIR]` — refuse while check fails; otherwise compile one
  deterministic TriG graph per run and print its SHA-256 digest.

Failures are intentionally exact and fail-hard — one violation per line:

```text
RULE path:line fix instruction
```

The failure output is the repair instruction. The checker does not guess,
coerce, or judge whether a Promise is true; judgment belongs to Oracles.

## Dependency boundary

Direct dependencies are exact-pinned and narrow:

- `serde_norway` (+ tokenizer) — YAML 1.2, with syntax-level rejection of
  anchors, aliases, tags, and merge keys;
- `serde_json`, `jsonschema` — the JSON value model and Draft 2020-12
  record shapes;
- `oxjsonld`, `oxrdf`, `oxttl` — offline JSON-LD 1.1 expansion, RDF quads,
  deterministic TriG serialization, parse-back verification;
- `sha2` — Witness, prose-source, and graph digests.

No CLI framework, directory walker, async runtime, network client, or
test-only dependency.

## Status

**Phase 1 — shipped and verified** (7/7 tests: both-polarity rule fixtures,
byte-stable TriG, prose-as-pointer-only, refusal-after-failed-check; fmt and
clippy clean under the pinned toolchain):

- closed vocabulary + record shapes + offline context;
- `assurance init / check / build`;
- witness workflow template + actor-driven bootstrap;
- skill pack scoped to the 7000-series review campaign (15 skills).

Remaining phases:

- the 15-skill sweep: bind skill text to the init file and convert embedded
  record semantics to the YAML-LD model (NORM/BIND/RECORD/INSTANCE ledger);
- a fresh-repository proof: human-invoked 7000 campaign end to end;
- schema evolution/replay rules, workflow hardening, durable evidence
  retention, forge adapters.

## History

[`SANITIZATION.md`](SANITIZATION.md) is the extraction record for the skill
pack (including the scope correction to the 15-skill review campaign).
[`PLAN.md`](PLAN.md), [`PLAN-REVIEW.md`](PLAN-REVIEW.md), and
[`PROPOSAL.md`](PROPOSAL.md) document the design iterations and their
adversarial reviews — superseded where noted, kept as the decision record.
