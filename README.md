# adversarial-assurance

**Phase 1 WIP.** This working tree implements Proposal v3's schema, checker,
witness-workflow template, and actor-driven bootstrap. It is not committed,
published, or approved for adoption. The 81-skill fidelity sweep is a later
phase; `skills/` remains the sanitized extraction and is unchanged here.

## Three layers

| Layer | Location | Authority |
|---|---|---|
| Pack | this repository | Generic vocabulary, schema, Rust checker, workflow template, bootstrap procedure, and skills |
| Adoption | `.assurance/` in a consuming repository | Materialized schema, run records, registry state, and bindings |
| Binding surface | `.assurance/assurance-init.yaml` | Models by role, harnesses, witness runner, actor seats, and immutable pack pin |

Skills point to the init file; the init file binds. Generic skill text must not
hardcode a model, harness, runner, or actor.

## Promise / Witness / Oracle

- **Promise** — a judgeable commitment with polarity and envelope.
- **Witness** — digest-bound retained evidence that one event existed; never a
  claim of exclusivity.
- **Oracle** — the bound authority that judges a Promise against its Witnesses
  and renders `PASS`, `FAIL`, or `BLOCKED`.
- **Run** — one expressly commissioned container for the triad.

The closed vocabulary is [`schema/vocabulary.yaml`](schema/vocabulary.yaml).
Agents may not mint nouns, verbs, or fields. Additions require a versioned
schema and checker change.

YAML records map 1:1 to JSON-LD 1.1 through one offline context. Prose is
authored as a literal or folded `body` block scalar. Compilation never inlines
that prose into the graph: it emits a `resolves_to` edge to the source record
and a SHA-256 `content_digest`. A graph consumer sees every relationship, then
loads only the pointed-at record when it needs depth.

## Pack layout

```text
schema/
  context.yamlld
  vocabulary.yaml
  records.schema.json
assurance/
  Cargo.toml
  Cargo.lock
  src/
  tests/
workflow/
  assurance.yml
bootstrap/
  README.md
skills/                         # untouched in Phase 1
```

An initialized adoption has:

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

## Checker

The crate and binary are both named `assurance`. From this repository root:

```sh
cargo build --locked --manifest-path assurance/Cargo.toml
cargo test --locked --manifest-path assurance/Cargo.toml
```

Commands:

- `assurance init [DIR]` — install `.assurance/` and the witness workflow,
  leave bindings explicitly `UNCONFIGURED`, and print the adopting agent's
  one-question bootstrap instructions.
- `assurance check [DIR]` — validate bindings, canonical schema copies, closed
  vocabulary, YAML/JSON-LD shape and formatting, run layout, all vertex/file
  edges, Witness digests, actor seats, and successor legality.
- `assurance build [DIR]` — refuse while check fails; otherwise compile one
  deterministic TriG graph per run and print its SHA-256 digest.

Failures are intentionally exact and fail-hard:

```text
RULE path:line fix instruction
```

One violation is printed per line. The failure output is the repair
instruction; the checker does not guess, coerce, or judge whether a Promise is
true.

## Bootstrap and CI witness

Follow [`bootstrap/README.md`](bootstrap/README.md). The adopting agent asks the
human once for models, harnesses, the witness runner, and reviewer/final
validator seats, then writes only `assurance-init.yaml`. The workflow reads
that committed file, checks out this pack at its full commit pin, builds with
the pinned Rust toolchain and lockfile, and runs check plus build on the bound
runner.

CI is the sole witnessed checker seat: the required workflow log is the
attestation that the checker ran on those inputs. Manual check/build is allowed
as authoring preflight, but is never evidence or a gating fallback. Repository
rules must require the workflow's `assurance-required` job; a workflow file by
itself cannot prevent merges.

## Dependency boundary

Direct dependencies are exact-pinned and limited to the same narrow pipeline
classes used by bedrock:

- `serde_norway` and its tokenizer — YAML 1.2 deserialization plus
  syntax-level rejection of anchors, aliases, tags, and merge keys.
- `serde_json` and `jsonschema` — the shared JSON-LD value and Draft 2020-12
  record-shape gate.
- `oxjsonld`, `oxrdf`, and `oxttl` — offline JSON-LD 1.1 expansion, RDF quads,
  deterministic TriG serialization, and parse-back verification.
- `sha2` — Witness, prose-source, and graph digests.

There is no CLI framework, directory walker, async runtime, network client, or
test-only dependency.

## Phase status

Phase 1 supplies the comprehension contract and deterministic witness
mechanism. Remaining work:

- adversarial review and human approval of this WIP;
- schema evolution/replay rules and any record types proven necessary beyond
  the four-noun core;
- the sentence-level NORM/BIND/RECORD/INSTANCE fidelity ledger and 81-skill
  sweep;
- a fresh-repository proof through the 1000/2000/6000/3000/4000 admission
  graph plus human-invoked 7000 review;
- production workflow hardening, durable evidence retention, forge adapters,
  and repository-rules automation.

[`PLAN.md`](PLAN.md) and [`PROPOSAL-REVIEW.md`](PROPOSAL-REVIEW.md) remain
superseded history. [`SANITIZATION.md`](SANITIZATION.md) remains the extraction
record and is not changed by Phase 1.
