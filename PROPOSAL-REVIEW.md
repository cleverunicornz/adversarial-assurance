# Sol Adversarial Review of PROPOSAL.md v2

Reviewer: independent model (OMP / GPT-5.6-Sol, maximum reasoning), read-only.
Inputs: `PROPOSAL.md` (v2 + §15), `PLAN.md`, `PLAN-REVIEW.md`, `README.md`,
`SANITIZATION.md`, and representative skills.
Verdict: **UNSOUND** — direction sound; operating detail is contradictory and
under-specified. A competent engineer cannot build this today from the
document alone.

> **Historical path note:** Mount Contract v1 supersedes the reviewed
> `.assurance/` layout with `situation/assurance/`. Findings below retain their
> original path wording as review evidence.

## What Sol endorses
- The two-lane authority split (orchestration/harness vs enforcement/CI) is
  coherent **as an authority split** (local advisory, CI authoritative) —
  but not as a *prohibition* on running the checker locally.
- Rust as the implementation language for a static validator: defensible.
- One executable: fine, if its responsibilities are explicit subcommands.
- The general direction (template + bootstrap + standardized enforcement) is
  right; the failure is operational clarity and internal contradiction.

## The central contradictions (all real)

1. **Bootstrap has two mutually exclusive definitions.** §8 requires six
   ordered operations (schema materialization, registry init, binding
   resolution, green CI self-test). §15 reduces the entire bootstrap surface
   to "does the workflow file exist?" A file's presence proves nothing:
   stale / locally modified / malicious / wrong-pinned / unrelated. Requires
   a version-aware init state machine (fresh / valid / upgrade / drift /
   partial), a written manifest (pack commit, workflow digest, binary digest,
   schema/context digests, binding profile), and explicit registry states.
   Presence-only init must be deleted.
2. **Which schema copy governs is unspecified.** Bundled schemas are
   authoritative *and* copied into the adopter *and* read by a binary built
   from a separate pinned checkout containing its own schema copy. Never
   defined who wins.
3. **Pin vs push contradiction.** "An ordinary push ships a change" while
   "adopters build from a pinned ref." An immutable pin does not move on
   push; a moving ref is not a pin. One adoption lock manifest is required
   binding: pack repo + immutable commit, binary version + digest, workflow
   template digest, schema/context set + digests, law-set version, and each
   run's subject/base/target SHAs. Old immutable records must validate under
   their original schema versions, never "latest".
4. **A workflow file is not a merge gate.** Branch protection / rulesets /
   required checks are not installed by adding a YAML file. A PR can modify
   its copied workflow unless that surface is protected. Needs a CI-adapter
   contract + GitHub implementation: prefer a minimal adopter wrapper that
   calls a centrally hosted reusable workflow at an immutable commit; init
   verifies ruleset/required-check config or returns BLOCKED; workflow
   changes are owner-review-gated (reusable required-workflow). Repository
   administrators are the legislative trust boundary.
5. **The gate-order problem (most important).** The pipeline runs machine
   gates *before* Post/Lock/proof progression (3010 gates, then reruns
   before Lock). CI can only see committed/pushed state, so moving the check
   to CI *reorders a normative gate*: agents advance blind and discover
   rejection only after pushing. The fix that preserves NORM: the exact
   pinned `assurance` binary must be **locally callable** for advisory
   authoring/preflight/compilation, with CI remaining the only
   merge-authoritative seat. "CI-only" violates the immutable core.
6. **Groundwork is a compiler, not a validator.** Packet admission,
   dry-run expansion, deterministic task-world generation, immutable
   skeleton generation, digests, Closure carriage, drift detection cannot be
   folded into a stateless report-only validator. The executable surface must
   be explicit: `assurance validate`, `assurance packet check`,
   `assurance packet build`, `assurance record init`, `assurance skill
   validate`, guarded `post`. Either preserve the compiler contract or
   explicitly delete the packet/task-world law and stop claiming NORM
   untouched.
7. **Every record is not "information and a graph query."** A JSON-LD
   document is graph data; framing is an operation over it. And "closed
   record set" (schemas authoritative) vs "new record types additive /
   contexts version independently" are incompatible without extension and
   compatibility rules.
8. **`init-done.json` vs all-YAML-LD** (one record is JSON), and **dockets
   live outside the repo** (`$HOME/data/goal-dockets/`) so CI cannot read the
   scope selector. Gate-relevant scope must move into committed run records.
9. **No canonical distribution coordinate.** The workflow says "clone THIS
   repository" but nothing is pushed/committed yet and the published install
   exposes only `skills/`.

## Missing executable contracts (10)

1. Constrained YAML→JSON-LD profile (version, scalar schema, dup keys,
   anchors/aliases/tags/merge keys, timestamps, binary, non-string keys).
2. Shape language and validation semantics (JSON Schema / SHACL / ShEx /
   bespoke — pick one).
3. Record-type/version registry.
4. Pack/schema/context/binary/workflow lock manifest.
5. CLI, exit-code, diagnostic, and canonical-report contract (sorted output,
   content-derived report IDs, no absolute host paths, input limits, invalid
   vs internal-failure separation).
6. Reference grammar (repo identity, ref types, relative paths, fragments,
   external artifacts, cross-repo refs, reachability). "The pinned SHA" is
   ambiguous: pack version vs target repo vs candidate/lock/closure/report
   SHAs.
7. Workflow contract: triggers, events, permissions, inputs, scope
   derivation, required-check identity, merge-queue, fork behavior,
   skipped/cancelled semantics, report retention.
8. Harness adapters: spawn, no-history handoff, goal lifecycle (currently
   omitted entirely), status/reclaim, model+effort selection, forge
   operations, guarded Post — with request/response schemas, invariants,
   failure mappings, conformance fixtures.
9. Bootstrap/upgrade/rollback/partial-install recovery.
10. Replacement for Groundwork packet compilation / task-world generation.

## Critical findings (10) — condensed

1. Bootstrap: two definitions, delete presence-only (§ above).
2. CI-only validation reorders gates; require locally callable pinned binary
   (advisory) with CI authoritative.
3. Deletes the required compiler and calls the remainder complete.
4. "YAML-LD" is not an implementable spec at this granularity; prefer
   constrained typed YAML unless RDF interop is a demonstrated requirement;
   if YAML-LD stays, publish a strict offline profile (narrow 1.2 subset, no
   dup keys, protected local contexts, absolute @ids, explicit @list, one
   shape language + processor version, canonicalization, byte-stable output).
5. Evidence-law pass unimplementable from stated inputs (segment immutability,
   sole-writer, append-vs-replace, ordering, projections, witness-at-time);
   needs ref grammar + full-history inputs + declared semantic for "sole
   writer".
6. Versioning destroys pin or upgradeability; add a lock manifest; remove
   presence-based stage 2.
7. Workflow file ≠ merge gate (see above).
8. Placeholder names ≠ harness bindings; missing goal capabilities; fixed
   model → capability-qualified role with the no-substitution refusal law
   preserved.
9. NORM/BIND/INSTANCE/RECORD is not a valid ledger: RECORD is a
   transformation, categories overlap, mixed sentences can't be one-labeled.
   Use two orthogonal axes (semantic content × transformation) with
   source-span/hash/retained-invariant/reason per entry.
10. Report persistence undefined; workflow summary is not durable evidence;
   a bot pushing to the PR branch violates sole-writer law. Use a canonical
   report artifact (content-addressed, signed/attested by the enforcement
   identity) on a protected evidence ref; summaries are projections only.

## Major findings (9) — condensed

1. Authority boundary: machine (syntax, shape, refs, hashes, history,
   projections, formal transitions, illegal successors) vs agent/human under
   versioned laws (relevance, sufficiency, oracle fidelity, justification).
   Green validator = `CONFORMANT`, never the domain `PASS`.
2. `.assurance/` has four competing owners; split zones
   (pack-versioned read-only / adopter-owned / actor-segmented / CI-owned)
   with overwrite+CODEOWNERS per zone.
3. Empty registries: distinguish `UNCONFIGURED` vs `CONFIGURED_EMPTY`;
   `UNCONFIGURED` blocks dependent gates.
4. Specific instruction rows needing split/replace/parameterize (pack
   model-identity, agent-run spawn contract, goal dockets/CI visibility,
   git/forge/main/merge-gate/post mechanics, spike Cargo/Moon/Nix/fleet rules,
   Groundwork carrier + task compiler, runs/AGENTS.md + JSONL→YAML-LD,
   adopted-protocol controls as fixtures, skill-authoring `.agents/.codex/
   llm_gateway.*`, collapse-graph instance, "Nine" canon, search/ast-grep
   pins) — none of which the proposed single-label ledger covers.
5. Rust fine; build-every-run not (CI queue, cold deps, toolchain
   provisioning, supply chain). Publish signed/checksummed versioned artifacts
   from the first adopter-facing release; hermetic `cargo build --locked`
   pinned as a reproducibility path; isolate caches by digest.
6. Self-hosted runner security absent: ephemeral dedicated runners, minimal
   permissions, no secrets for untrusted PRs, content-addressed caches, DoS
   limits in the parser.
7. README + SANITIZATION become false; keep them as a dated extraction record
   and add a separate migration/conformance record for the generalized pack.
8. CLI/deterministic-output contract absent.
9. External/binary evidence and repo growth unaddressed; large evidence →
   content-addressed storage + digests; YAML-LD records link to digests
   rather than absorb bytes.

## Direct answers to the proposal's open items

1. GitHub-only vs "any repository": the design claims agnostic, the document
   designs GitHub. Choose one: forge-neutral `assurance validate` contract +
   GitHub reference adapter (+ local/other CI adapters), or state the hard
   GitHub prerequisite precisely (GitHub repo, configured required
   workflow/ruleset, supported self-hosted runner, compatible harness).
2. Pinned release vs build from source: signed/checksummed **release
   artifact** for adopter CI; source build = documented reproducibility
   path (`Cargo.lock` committed, pinned toolchain, `--locked`, isolated
   cache). "Bespoke" = pack-owned source; third-party crates remain
   dependencies; any Bedrock-derived code must be documented at module level
   (or Bedrock is not a "base").
3. One binary vs model-owned (c)/(d): one executable fine; do not move all
   of (c)/(d) to the model and do not let the binary claim semantic
   assurance. Mechanizable parts (hash chains, segment history, declared
   writer, folds, formal transitions, forbidden successors) stay in the
   binary; semantic judgment stays under versioned laws with independent
   actors. Restore packet compilation locally (the four passes don't replace
   it).
4. YAML-LD burden: yes, real. Either use constrained typed YAML, or (if
   YAML-LD is fixed) ship a strict profile + protected offline contexts +
   shapes + canonical examples/counterexamples + a record generator + local
   validation + migration rules, and a demonstrable query need.
5. Init without provisioned runners: an unmet prerequisite, no offline
   equivalent. Either fail closed with the stated requirement, or a
   local/container adapter that emits a signed receipt which the online gate
   then verifies. Unsigned local reports are diagnostic only.

## Extra open items Sol adds

- Who validates records mid-run → the same pinned binary, locally callable,
  advisory; CI authoritative. Without this, current gate order is unbuildable.
- Who owns/may mutate the seeded workflow → adopter admins; pack owns the
  canonical template. Copying a file does not preserve pack ownership.
- How old immutable runs replay after schema upgrades → each run binds its
  exact pack/law/context/shape versions; revalidation against "latest"
  prohibited.

## First breakages, in order

1. Workflow resolution fails ("clone THIS repo" has no canonical remote/ref;
   install exposes only `skills/`).
2. Init produces a non-adoption (presence ≠ schemas/contexts/bindings/runner/
   rulesets/required check/registry).
3. First CI attempt enforces nothing (no runner → queued; no required check →
   no block; workflow not yet on default branch).
4. First orchestrated run fail-closes (model identity, spawn/goal APIs,
   lifecycle authority, companion authorities unresolved).
5. First 3000/4000 packet stops permanently (Groundwork check/build + task
   world not implemented by a stateless validator).

## Top 5 actions

1. Rewrite the operating contract: one manifest, one authority hierarchy, one
   init state machine, one version/upgrade model, one support matrix. Delete
   the §8/§15 contradiction.
2. Specify the record protocol before coding: prefer constrained typed YAML;
   otherwise the complete offline YAML-LD profile, shapes, IDs, ordering,
   canonicalization, compatibility, reference grammar.
3. Restore point-of-act mechanics: locally callable pinned CLI; packet
   check/build, record scaffolding, skill validation, guarded Post; CI stays
   authoritative.
4. Design enforcement as adapter + real gate: triggers, scope, reports,
   runner security, merge queue, branch rules, required workflow, workflow
   immutability, durable attestation storage.
5. Redo the migration as an evidence ledger (two orthogonal axes), then prove
   a full fresh-repo run (1000/2000/6000/3000/4000 + independently
   commissioned 7000) before rewriting README/SANITIZATION.
