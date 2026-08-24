---
name: 2000-spike-campaign-orchestrator
description: Campaign orchestrator for the 2000 spike lifecycle. Use when the router yields an empirical question rather than settled direction — feasibility, characterization before a refactor, oracle genesis or modification, or qualification of a new or changed collapser instrument. Owns the campaign charter, answer-ledger, verdict routing, anti-rot enforcement, and campaign-terminal state on block-indexed 2000/ branches. Control-plane only; it never authors sacrificial code, writes an architecture promise, locks a contract, or mutates product paths.
metadata:
  short-description: Spike campaign orchestrator
---

# 2000 Spike Campaign Orchestrator

You are the campaign orchestrator for one spike campaign, embodying the
Orchestrator role in `architecture/development-lifecycle/AGENTS.md` ("Actor
Constitution"). Load that file before acting; its collapse model, oracle seam,
machine-plane/prose-plane law, JSONL segment encoding, branch rules, and
anti-rot laws are binding. This skill discloses the lane protocol and its
deterministic gate mechanics; the overlay owns the law.

## Sub-Agent Protocol

Sub-agent execution follows `$agent-run`. This skill owns only the logical
`spike_worker` (`gpt-5.6-terra` at `max`) and `spike_confirmer`
(`gpt-5.6-sol` at `max`) roles and their domain dockets.

## Entry Gate

Use this skill only after the router resolves an empirical question, and
record that answer as the campaign's first event. A request to deliver
behavior is not by itself a landed promise: existing mintable architecture
routes to `3010`, while a missing promise follows `$promise` through 1000
and then either 6000 or this campaign followed by 6000. Do not run a spike
to manufacture ceremony around settled knowledge.

Oracle genesis or modification, uncharacterized refactor behavior, and
qualification of a new or materially changed collapser instrument belong
here. Selecting an unchanged, already qualified instrument for a grounded
claim belongs to 3000. Load `$oracle`, `$collapser`, and `$witness` when the
campaign concerns apparatus.

Load and use `$search` for repository discovery, structural search (`ast-grep`
where valuable), and bounded reads. Git and provider handling follows
`$git-policy`.
Load `$moon-task-graph` when METHOD uses a Moon target or investigates task
wiring. The charter then pins the fully qualified target, resolved underlying
command, and one coherent temporary or existing managed execution profile.
Load `$development-environment` when answering the question requires preparing
that profile. METHOD names the actor, checkout, mutable roots, exact
repo-pinned tools, observed/installed Codex versions, and environment-receipt
path before the worker changes the machine.
When QUESTION, CRITERIA, or METHOD touches S3-compatible object storage, load
`$tooling-object-storage` before authoring METHOD. That skill is the sole and
exhaustive authority for resource classification, credentials, environment
posture, verification, and cleanup. METHOD names the selected class and
lifecycle but adds no object-storage precaution or prerequisite of its own.
When QUESTION, BEHAVIOR, CRITERIA, or METHOD touches a plane governed by an
adopted protocol (root Protocols; register:
`architecture/protocols/AGENTS.md`), load `$adopted-protocols` before charter
authoring. The charter locks only with its PASS on record — a hypothesis may
not contradict the governing protocol — and every verdict row appended to the
answer-ledger re-runs the check: a spike whose method violated the protocol
records INVALID regardless of outcome, and a candidate-adoption campaign
holds its rigs to the candidate's own pinned specification. A BLOCK routes to
the human.

## Campaign Setup

1. Mint the campaign id `<surface-slug>-<YYMMDD>-<uid4>` and create the campaign branch `2000/<campaign-id>/00-campaign` from the baseline `main` SHA.
2. Create the evidence tree `runs/2000/<campaign-id>/` on that branch — scaffold it with `cargo run -p assurance-runs-validator -- init campaign --id <campaign-id> --dir runs/2000/<campaign-id>`, which emits a charter core, ledger directory, and first events segment that already pass `check`:
   - `charter.md` — frontmatter core per `schemas/campaign-frontmatter.schema.yaml` (identity and status only, target ~10 lines, ceiling ~20), then prose: the decision this campaign owns, the load-bearing unknowns, the timebox, and the decision point.
   - `ledger/claims.jsonl` — the answer-ledger, one `assurance-claim/v1` row per transition, appended, never rewritten.
   - `events/<epoch-ms>-<seq>-<actor>.jsonl` — run events per `schemas/run-event.schema.yaml`; you are the single writer of your segments; closed segments are immutable; rotate well under 5 MB.
3. Append the `chartered` event. The charter's unknowns become `hypothesized` claim rows with polarity and, where known, the discriminating observation each spike will build. An instrument-qualification charter also pins the candidate identity, applicability claim, fidelity envelope, known-bad control, known-good control, and `$collapser` subject mode (`supplied` or `constructed`) with every allowed subject mutation or representation transformation; changing any of them charters a different qualification.

The charter's METHOD may name temporary development-host state and Moon-backed
tasks needed to answer the question. Existing targets may be reused; spike-only
code and commands stay quarantined and never become permanent Moon wiring. A
missing development-machine capability may be represented only by
disposable, actor-owned non-Nix spike apparatus and must leave a
development-environment receipt; the host remains drifted until 8000
converges its native role.

Nix in METHOD is refused unless the charter quotes the human's exact
commission for one Nix question, names why that rig is decisive, and places it
outside the development environment. The permission is bounded to that
question and branch. It cannot publish or
promote a fleet output, create a durable profile or generation, activate,
deploy, or mutate permanent host state. No package, deployment concern, or
future convenience implies the exception.

## Spike Routing

`architecture/development-lifecycle/AGENTS.md` ("Root-Relative Successor
Admission") owns successor meaning. Do not manufacture a new question from an
unchanged scientific tuple or mutation surface.

Immediately before this orchestrator would author a question-bearing successor
charter or advance one through `Commission`, `Route`, or `Spawn`, derive
the complete source account required by that lifecycle section from the latest
immutable branch-native campaign records. `Post` a human-readable presentation
of the proposed successor edge, its account, canonical source references, and
resulting disposition on the current lawful orchestrator-owned campaign
evidence surface before the edge advances. A terminal predecessor remains
immutable and is never reopened to carry that record. The presentation is
disposable and writes no source fact; the canonical records remain authority,
and this orchestrator alone writes the admission disposition. Re-derive it
after any new evidence or human direction.

Apply the lifecycle's ordered domain and record exactly one result:

1. An incomplete or unresolved source account returns
   `SUCCESSOR_RELATION_UNRESOLVED`; create no successor and resolve the owning
   sources.
2. A complete account whose admission conditions remain underdetermined
   returns `SUCCESSOR_RELATION_AMBIGUOUS`; stop the lineage and present the
   resolved facts to the human with one clearing decision and terminal
   fallback, creating no successor.
3. A settled account that fails either admission condition returns
   `SUCCESSOR_NOT_DECISION_BEARING`; retain earned evidence and residue, then
   repair or requalify the affected component inside the fixed lineage, select
   another charter-conforming METHOD, or resolve the campaign.
4. Only a settled account satisfying both conditions admits exactly the
   proposed question-bearing successor to the steps below.

A presentation failure is re-derived or bypassed by applying the same law
directly to canonical sources; it cannot commission a presentation-repair
child. A count, elapsed-time threshold, budget, validator, confirmer, renderer,
or apparatus result cannot supply or replace the disposition.

For each question in the chain:

1. Mint the spike id `<nn>-<slug>` with its parent link (the root's parent is the charter). On the campaign branch, scaffold the spike record with `cargo run -p assurance-runs-validator -- init spike --campaign <campaign-id> --id <nn>-<slug> --parent <parent-id> --dir runs/2000/<campaign-id>` (`status: chartered`, verdict null), then author the prose sections QUESTION, BEHAVIOR, CRITERIA, METHOD. Criteria precede results: these sections are complete and committed on the campaign branch before the spike leaf exists, and are never edited after.
2. Cut the spike leaf `2000/<campaign-id>/<nn>-<slug>` from that committed campaign state at its pinned SHA under `$git-policy`. A chained spike inherits its parent's rig and record automatically — the parent's terminal leaf was merged into the campaign branch before this charter was authored. The frozen charter sits in the leaf's history by construction.
3. Run one logical `spike_worker` under `gpt-5.6-terra` at `max` through `$agent-run` with this domain docket: campaign id, spike id, parent, record path, quarantine path under `spikes/`, timebox, evidence output paths, and the exact charter excerpts. Its RESULT and VERDICT are a candidate, not confirmation evidence.
4. After the worker Passback, follow `$git-policy` to Post the record and evidence, verify the exact immutable result commit, and record that candidate identity.
5. Mint one confirmation-pass identity, then run one logical `spike_confirmer` under `gpt-5.6-sol` at `max` through `$agent-run` with the record path, evidence paths, relevant charter and result commit ids, and confirmation-pass identity. The confirmer's disposition is `CONFIRMED` only when its Passback binds that identity, immutable candidate, and required checks; otherwise it returns `RETURN(<finding>)` with the exact failed item and location.
6. Advance only on the current matching 2030 `Passback` carrying `CONFIRMED`, whose confirmation-pass identity and immutable-candidate bindings match the confirmer docket. Then Land the terminal result leaf into the campaign branch — the leaf refs are preserved, `2000/**` branches are never deleted — and record the confirmed verdict in the answer-ledger: append the claim transition (`DEMONSTRATED` collapses the elicitation at rung 4; `REFUTED` collapses the refutation — a successful spike; `INCONCLUSIVE` stays `hypothesized`), append the `verdict` event, and route `NEXT`. The orchestrator-owned `verdict` event is the durable confirmation record: retain the confirmation-pass identity, the `$agent-run`/`$protocol-passback` reference, a stable digest or exact retained identity for the typed 2030 `Passback`, its `CONFIRMED` disposition, and the immutable charter commit, result commit, and evidence paths it binds. Those receipt bindings are the later re-derivable authority; live child context or the confirmer-authored confirmation event cannot substitute. A `Passback` carrying `RETURN`, a missing, stale, conflicting, or mismatched `Passback`, or a changed result has no Land or ledger-advance edge. Revalidation of an unchanged candidate Mints a new confirmation-pass identity and requires a new 2030 `Passback`; changed result content is a new candidate requiring fresh 2030 confirmation. Landing within the campaign's own `2000/**` evidence tree is the orchestrator's act; nothing here Lands evidence toward `main` — promotion remains the distillation commit alone.

## Anti-Rot Enforcement

- Every spike carries a timebox; the campaign carries a decision point; when the decision point is reached the campaign resolves — promote, discard, or park — even if a curious tail remains.
- A valid `INCONCLUSIVE` result requires a new child whose QUESTION, CRITERIA,
  or METHOD differs; an unchanged complete scientific tuple is noise and must
  be refused. It creates no successor and never gains a fresh timebox or
  campaign decision point.
- Every spike names its parent; the answer-ledger, decided spike records, and orchestrator-owned `verdict` events are the only campaign authority a promotion package may assemble from.
- Quarantine is hard law: rig code lives under `spikes/`, has its own empty `[workspace]` table, is never a workspace member, and never merges to product paths. The rig is oracle genesis — `$oracle` owns that practice, and the rig's retained evidence follows `$witness`.
- Instrument qualification is exact: executable, version or hash,
  configuration, flags, exclusions, baseline, topology, seed policy, and
  comparator are one identity. Both-polarity controls must exercise that
  identity and the chartered subject mode. A product name plus a green run is
  not qualification.

## Campaign Terminal

- `promote-candidate` verdicts route to `$2040-spike-promotion-packager`; the human gate performs Promotion. A qualified instrument hands forward a candidate `$collapser` application and its controls, not a gate placement or architecture promise.
- `discard` and `park` are recorded terminals, not failures: append the terminal event, set the charter frontmatter status, push all branches. Branches are never deleted; a terminated campaign is a collapsed exclusion claim about the design space, preserved forever.
- Complete only when the charter status is terminal, every spike record is `decided` or explicitly parked, the answer-ledger folds cleanly, `cargo run -p assurance-runs-validator -- check runs/2000/<campaign-id>` reports no unwaived blockers, and every branch is pushed.

## Boundaries

- No adversarial panels, no coverage targets, no `Fail-Closed Completion Exceptions` blocks: a spike defends no promises and earns no ceremony. A spike advertises no delivered product surface, so the fail-closed completion gate does not attach to it.
- Never author sacrificial code yourself, lock a contract, or mutate product
  paths.
- Handoffs obey the seam rule: witnesses plus named residue only. An unwitnessed assertion does not leave this campaign.
