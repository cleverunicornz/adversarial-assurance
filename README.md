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
   its fourteen stage roles (charter guarantees, recon + scout, triage,
   test-integrity, integrity plan/execute, proof validation, gapfill,
   root-cause trace, feedback synthesis, retrospective, report, promotion).
   Each role is repo-agnostic, uses explicit `{{variable}}` bindings, and
   records procedure state under `situation/assurance/runs/<run-id>/`. Optional
   repository capabilities are evidence sources, never pack prerequisites.
2. **The record language** (`schema/`) — a closed YAML-LD vocabulary under
   the sovereign `urn:assurance:` base. Every act is a **Promise**, backed by
   digest-bound **Witnesses**, judged by an **Oracle** (`PASS` / `FAIL` /
   `BLOCKED`), inside a **Run**.
3. **The checker** (`assurance/`, Rust) — `assurance init`, `update`, `check`,
   and `build`: validates mount contract/bindings, evidence coverage, record
   law, run coherence, generated run graphs, and the stable graph manifest.
4. **The witness workflow** (`workflow/`) — one required job independently
   resolves bedrock from the substrate lock and assurance from its init pin,
   builds both tools outside the target checkout, then runs both checkers and
   generated-output gates.

## The model

| Layer | Location | Authority |
|---|---|---|
| Bedrock substrate | `situation/` base namespaces + registration vertex | formation law, mount boundary, substrate checker lock |
| Assurance mount | `situation/assurance/` | expansion bindings, schema, records, evidence, workflow template, graphs, manifest |
| Binding surface | `situation/assurance/assurance-init.yaml` | substrate contract requirement, closed variables, independent assurance pin |

**Skills point; the init file binds.** Proper model, harness, runner, and actor
names never appear in skill procedure text. Skills use the closed placeholders
`{{lead_model}}`, `{{executor_model}}`, `{{validator_model}}`, `{{harness}}`,
`{{witness_runner}}`, `{{reviewer_seat}}`, and
`{{final_validator_seat}}`. The adopting agent asks the human one grouped
question and writes those values once. Change a binding: edit one file and
commit it. `assurance check` rejects missing, placeholder, invented, or
undeclared record variables.

**Closed vocabulary and identity.** Nouns (`Promise`, `Witness`, `Oracle`,
`Run`) and verbs (`witnessed_by`, `judged_by`, `resolves_to`, `part_of`,
`succeeded_by`) are fixed in `schema/vocabulary.yaml` version 3. Assurance
owns `urn:assurance:`; pointers are
`urn:assurance:path/<repo-relative>`. It never mints or carries
bedrock-owned RDF IRIs.

**Depth behind Witnesses.** Record bodies are non-empty block-scalar summaries
bounded to 16 KiB. Full stage documents and replayable artifacts are committed
under same-run `evidence/**`; every evidence file must be inversely covered by
a Witness. External payloads use a committed URI/digest/size/provenance
manifest. Compilation omits body prose, emits source path/digest pointers, and
writes one byte-stable run graph plus a sorted mount graph manifest.

**CI is the two-checker witness seat.** The single required
`assurance-witness` job runs bedrock check → assurance check → assurance build
→ assurance graph/manifest unchanged gate → bedrock generated-output
unchanged gate on the substrate-approved runner. Manual commands are
authoring preflight only.

## Quickstart — mount into a formed repository

Prerequisite: the repository is already formed by bedrock 0.7.0 or newer with
`seed/substrate-lock.json` supporting `bedrock-expansion-mount/v1`.

```sh
cargo build --locked --manifest-path assurance/Cargo.toml
assurance/target/debug/assurance init /path/to/formed-repo
```

`init` refuses before mutation when `situation/` or mount support is absent. It
writes only `situation/assurance/`, ships an empty graph manifest, and prints a
complete `situation/architecture/mount-assurance.yamlld` proposal. It never
writes the bedrock registration.

The adopting agent follows [`bootstrap/README.md`](bootstrap/README.md):

1. populate the substrate block, independent assurance pin, and seven
   variables;
2. set `status: CONFIGURED`;
3. run `assurance update` to render the runner-bound workflow template;
4. copy that template to the consumer workflow location;
5. submit the printed registration proposal through the bedrock authoring
   loop.

```sh
assurance check /path/to/formed-repo
assurance build /path/to/formed-repo
```

```text
situation/assurance/
  assurance-init.yaml
  registry.yaml
  graph-manifest.yaml
  schema/
  workflow/assurance.yml
  runs/
    <run-id>/
      run.yamlld
      promises/*.yamlld
      witnesses/*.yamlld
      oracles/*.yamlld
      evidence/**
      graph.trig
```

Authoring order for graph-changing commits: assurance source preflight →
assurance build → bedrock build → both checks → commit both tools' generated
outputs. CI then proves the committed bytes were already current.

## The checker

The crate and binary are both named `assurance`. From this repository root:

```sh
cargo build --locked --manifest-path assurance/Cargo.toml
cargo test --locked --manifest-path assurance/Cargo.toml
```

- `assurance init [DIR]` — require a formed mount-capable substrate; install
  only `situation/assurance/`; print bootstrap and registration proposals.
- `assurance update [DIR]` — refresh only mount-owned schema copies, workflow
  template, and graph manifest; preserve bindings, records, evidence, and all
  bedrock files.
- `assurance check [DIR]` — fail-hard on substrate/binding, canonical-file,
  layout, evidence, shape, edge, digest, actor, lifecycle, graph, or manifest
  violations.
- `assurance build [DIR]` — validate source, then deterministically refresh
  run graphs and `graph-manifest.yaml`.

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

**Mount Contract v1 expansion side — implemented and locally verified:**

- vocabulary v3 and `urn:assurance:` identity sovereignty;
- full `situation/assurance/` re-root;
- substrate block and mount-capability refusal;
- recursive committed evidence law, external manifests, inverse coverage;
- deterministic run graphs plus stable graph manifest and stale-output gate;
- mount-only `assurance update`;
- normalized and corrected 15-skill campaign;
- independent-pin, two-checker workflow template.

Remaining cross-repository proof:

- run the external fresh-repository matrix against the separately released
  bedrock mount-support implementation;
- execute one full campaign through legal promotion-seam proposals.

## History

[`SANITIZATION.md`](SANITIZATION.md) is the extraction record for the skill
pack (including the scope correction to the 15-skill review campaign).
[`PLAN.md`](PLAN.md), [`PLAN-REVIEW.md`](PLAN-REVIEW.md), and
[`PROPOSAL.md`](PROPOSAL.md) document the design iterations and their
adversarial reviews — superseded where noted, kept as the decision record.
