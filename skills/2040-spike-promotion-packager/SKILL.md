---
name: 2040-spike-promotion-packager
description: Promotion packager for a resolved spike campaign. Use when a 2000 campaign reaches its decision point with promote-candidate verdicts and the human gate needs the promotion package assembled from recorded verdicts. Builds from the answer-ledger only, maps findings to obligations, resolvable oracle refs, tolerance tiers, candidate collapser applications, and promise-grounding dispositions, then returns distillation content through `$agent-run`. It never writes architecture, selects a final gate, locks a contract, merges evidence branches, or moves quarantined code.
metadata:
  short-description: Spike promotion packager
---

# 2040 Spike Promotion Packager

You assemble the promotion package for one resolved campaign. `architecture/development-lifecycle/AGENTS.md` is binding — especially Promotion, Anti-Rot Law 4, the collapse model, and the oracle seam.

## Sub-Agent Protocol

Sub-agent execution follows `$agent-run`. This skill owns only the package's
inputs, assembly law, and output.

## Assembly — From Recorded Authority

Build the package by folding `runs/2000/<campaign-id>/ledger/claims.jsonl`, the decided spike records, and the orchestrator-owned `verdict` events. Never assemble from recollection or chat history: a finding absent from those records did not happen for the purposes of Promotion.

Before treating a candidate finding as surviving, fold its current matching
orchestrator-owned `verdict` event with the ledger and decided record. Require
that event to retain the confirmation-pass identity minted before the 2030
`Spawn`, the `$agent-run`/`$protocol-passback` reference, a stable
digest or exact retained identity for the typed 2030 `Passback` carrying
`CONFIRMED`, and the exact immutable charter commit, result commit, and
evidence paths that the package consumes. Verify the retained Passback
reference, digest or exact retained identity, and every binding against those
folded records. A worker
verdict, live child context, a confirmer-authored confirmation event, a
`Passback` carrying `RETURN`, or a missing, stale, or mismatched retained
receipt has no packaging edge; route it to the orchestrator without
re-judging or repairing it here.

The package (`runs/2000/<campaign-id>/promotion/package.md`, frontmatter core: identity and status only) carries:

- the decided question and the full campaign lineage (spike ids, parents, verdicts);
- each surviving finding with its evidence link and pre-declared criteria;
- for each finding, the **commissioned claim rows** the Build contract will adopt — statement, polarity (`elicit` and the paired `exclude` rows where forbidden behavior was characterized), envelope, and an `oracle_ref` that resolves to the spike's rig at a pinned SHA on its `2000/**` branch, feeding the oracle seam gate at `3000` lock;
- for each finding, the declared tolerance tier and the first-draft gauge specification (the rig is the genesis of the oracle), naming `$collapse-graph` node ids where the gauge maps onto commissioned instruments — promotion hands forward routes, not vibes; `$oracle` owns the gauge's proving, `$witness` its evidence shapes;
- for every candidate collapser application, the possibility class, exact
  instrument identity, declared subject mode and subject transformation
  account, applicability claim, fidelity envelope, known-bad and known-good
  control witnesses plus the decisive predicate each reached, charter commit,
  result commit, confirmation-pass identity, orchestrator-owned `verdict`
  event and its containing commit, `$agent-run`/`$protocol-passback`
  reference, typed 2030 `Passback` `CONFIRMED` disposition, stable digest or
  exact retained identity, and immutable charter/result/evidence bindings,
  oracle ref,
  producer-bound actual and raw path where the claim names a producer,
  declared producer-to-oracle transformation account, proposed witness shape,
  and residual. This is input to 3000 selection, not a final application or
  gate placement;
- the load-bearing unknowns that were resolved, and those explicitly accepted as risk — named residue, never silent;
- the branches or rigs pruned from the promoted record, with reasons.

Add a `Promise Grounding Disposition` for every candidate obligation:

- **grounded** — name the exact pinned `architecture/` source and show how the
  obligation's owner, seam, domain, and exclusivity mint from it; or
- **promise-authoring-required** — name the closed promoted knowledge and the
  separate 6000 promise-plane predecessor that must land before 3010 begins.

If the knowledge needed to write a closed promise is still missing, the
package may not call the obligation ready for authoring. Route a read-only
knowledge gap to 1000; that verdict may go directly to 6000 or, if it exposes
an empirical unknown, through 2000 and human promotion before 6000. When the
missing-promise gap is already known to need execution, record its closed
question and boundary in 1000 before the 2000 campaign; route a human-owned
design choice to the human. Every path returns through 6000 before 3010. Do
not invent an architecture ref, treat the package as architecture, or
commission a future promise to satisfy this disposition.

## The Gate And The Distillation

- Promotion is human-gated. You prepare and present; the human promotes. A
  `DEMONSTRATED` verdict becomes an adopted obligation only through that
  recorded human decision.
- Promotion adopts an obligation; it does not itself create promise-plane
  authority. Replace "committed promise" language in the package with
  "promoted obligation" unless an exact landed architecture source is named.
- Instrument qualification is not application selection or gate placement.
  The package may carry exact proven controls forward, but 3000 owns whether
  and where the instrument is commissioned.
- The human decision records which exact candidate application identities are
  included in the promoted handoff, deferred, or excluded. Inclusion is still
  not 3000 selection. A descendant commit is a different identity and inherits
  neither confirmation nor inclusion in that handoff.
- A candidate application is not promotion-ready when its claimed producer is
  unbound, its transformation account is missing, or either control failed
  before reaching the claimed predicate. Return that gap to the orchestrator;
  integrity-only evidence cannot be packaged as origin evidence.
- On promotion, prepare the distillation content under the disjoint id-keyed
  path `runs/2000/<campaign-id>/**` — the distilled package and findings memo
  only — and return it through `$agent-run`. Publication and integration
  follow `$git-policy`. Evidence branches are never wholesale-merged; raw
  lineage stays on `2000/**` branches forever.
- With a grounded disposition, the package is authoring *input* to `3010`.
  With `promise-authoring-required`, it is input to a separate 6000 unit and
  cannot enter 3010 until that promise lands. It never constitutes architecture
  authority or the locked contract, never bypasses `3020` validation or `3030`
  counterfactual review, and never carries quarantined code toward product
  paths — promoted behavior re-enters through Build under its gauge.

## Boundaries

No contract lock, no architecture mutation, no merges of evidence branches,
no product mutation, and no provider comment authority. A package, claim,
review, oracle ref, or promised follow-up
can never recursively ground its own missing promise. If the campaign has an
unspiked, unaccepted
load-bearing unknown, the package may not cut: return the gap to the
orchestrator instead of papering over it.
