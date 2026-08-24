---
name: collapse-graph
description: The legend of the repository's commissioned collapse primitives — the base mechanism classes that force possibility spaces down, mapped per surface class with their relationships. Use when authoring a 3000 contract's Collapse Route, commissioning claims, planning validation expectations, running 3030 counterfactual review against declared residuals, or packaging a 2000 promotion's gauge spec. Pair it with $collapser to bind a graph edge to an executable application. Not a test-execution guide; never load this mid-mutation to choose methods because the coder walks the locked route, not the map.
metadata:
  short-description: The commissioned collapse-primitive legend
---

# Collapse Graph

This is a legend, not a map. It names the platform's commissioned collapse
primitives — the primary colors — and their relationships: which surface
class carries which possibility classes, which instrument collapses each,
at what evidence rung, at what cost. Everything built *on* these
primitives (concrete suites, harnesses, rigs, lanes) is derivative and is
discovered from the tree, never enumerated here. Relationships are the
point: an agent that has internalized this graph knows the whole
neighborhood — including which possibilities have no friend yet, because
an empty route is visible by construction.

## How To Read It

- **Three node kinds.** `surfaces` are classes of place (not enumerations
  of crates). `possibilities` are what can still happen there after
  existence is delivered. `collapsers` are the commissioned instruments
  that force a possibility class down.
- **`plane`** says what a collapser actually collapses: `product-behavior`
  (the code's own possibility space), `detector-strength` (whether the
  witnesses tell the truth), `performance-claim` (deltas, never
  engineering), `seam-structure` (contract/wire integrity).
- **`rung`** cites the evidence ladder ruled in
  `architecture/development-lifecycle/AGENTS.md` (1 unconstructible,
  2 proven, 3 forced-observable, 4 sampled, 5 monitored). This graph
  never restates what a rung means.
- **`cost`** is the walk behavior: `standing` runs for every unit and is
  already law; `routed` enters through a contract walk when its
  possibility class is live; `commissioned` is campaign-scale, bought
  deliberately, never ambient.
- **`door`** is the procedure skill that owns invocation or application.
  Runners are doors, never nodes. Where a specialist runner does not own
  the whole binding, `$collapser` is the application door.
- **`status`** may mark wiring honestly where an instrument is
  commissioned but not yet wired or proven in-repo. Volatile posture
  belongs in the risk register, never in a stale graph annotation.

## The Graph

```yaml
surfaces:
  rust-owner-surface:            # single-writer crates and service binaries
    possibilities: [logic-inversion, invariant-break, output-drift,
                    error-path-untested, schedule-fault-sensitivity,
                    perf-regression, detector-false-trust]
  rust-boundary-decoder:         # parsers, codecs, protocol envelopes, external input
    possibilities: [adversarial-input-crash, invariant-break,
                    logic-inversion, output-drift]
  proto-seam:                    # the typed contracts under proto/
    possibilities: [wire-compat-breakage, seam-contract-drift]
  web-surface:                   # Svelte 5 / SvelteKit application code
    possibilities: [type-drift, ts-logic-inversion, flow-breakage]
  native-shell:                  # Tauri
    possibilities: [native-boundary-breakage]
  public-flat-html:              # flat public pages via artifact delivery
    possibilities: [page-breakage, a11y-violation]

possibilities:
  logic-inversion:            {collapsed_by: [nextest]}
  invariant-break:            {collapsed_by: [proptest]}
  output-drift:               {collapsed_by: [insta]}
  adversarial-input-crash:    {collapsed_by: [afl]}
  error-path-untested:        {collapsed_by: [fail-rs]}
  schedule-fault-sensitivity: {collapsed_by: [dst, fail-rs]}
  perf-regression:            {collapsed_by: [criterion]}
  detector-false-trust:       {collapsed_by: [cargo-mutants]}
  wire-compat-breakage:       {collapsed_by: [buf]}
  seam-contract-drift:        {collapsed_by: [buf]}
  type-drift:                 {collapsed_by: [svelte-check]}
  ts-logic-inversion:         {collapsed_by: [vitest, fast-check]}
  flow-breakage:              {collapsed_by: [playwright]}
  native-boundary-breakage:   {collapsed_by: [tauri-webdriver]}
  page-breakage:              {collapsed_by: [playwright]}
  a11y-violation:             {collapsed_by: [axe-core]}

collapsers:
  rustc-types:
    plane: product-behavior; rung: 1; cost: standing
    note: collapse by construction — what the type forbids needs no test
  clippy:
    plane: product-behavior; rung: 2; cost: standing
    note: targeted per touched package, -D warnings; root law
  nextest:
    plane: product-behavior; rung: 4; cost: standing
    door: $rust-nextest
    note: example tests witness existence on the intended path
  proptest:
    plane: product-behavior; rung: 4; cost: routed
    note: input-space and invariant collapse — parsers, codecs, round
      trips, state machines; a generator that cannot reach the failing
      region proves nothing
  insta:
    plane: product-behavior; rung: 4; cost: routed
    note: drift collapse — did this work change something it was not
      commissioned to change; never blind-accept snapshots; stabilize
      volatile fields
  criterion:
    plane: performance-claim; rung: 4; cost: routed
    note: the performance-delta pointer — did this pass move performance,
      cheaply, without a soak; a pointer, never a verdict; performance
      engineering is a different discipline outside this graph
  afl:
    plane: product-behavior; rung: 4; cost: commissioned
    door: $rust-afl-campaign
    note: THE standardized fuzzing instrument — chosen over cargo-fuzz
      for depth; adversarial sampling of the input domain
    edges:
      structured-input: arbitrary   # the typed bridge; see door for use
    promotion: the first commissioned first-party typed-AFL target
      promotes `arbitrary` to its own node with an exact workspace pin
  cargo-mutants:
    plane: detector-strength; rung: meta; cost: commissioned
    note: the faraway friend — verifies that the tests actually kill;
      long and iterative; its edges activate when the neighborhood
      warrants the trip (critical shared logic, a suspiciously green
      suite, assurance being moved), never as habit; measures the suite,
      never the product
  buf:
    plane: seam-structure; rung: 2; cost: routed
    door: $collapser
    note: exact configured schema and wire compatibility against the
      named baseline. Qualification and gate placement are separate;
      architecture/risk/buf/ owns current pins, exclusions, lint posture,
      and whether any invocation is blocking
  fail-rs:
    plane: product-behavior; rung: 3; cost: routed
    status: commissioned-unwired
    note: named failpoints, armed in tests, absent from production
      builds — the error branch becomes an event you elicit and witness;
      the surgical half of forced-observable
  dst:
    plane: product-behavior; rung: 3; cost: commissioned
    door: $collapser
    note: deterministic simulation of real tonic-shaped service code over
      modeled network, time, randomness, and object_store seams. Credit
      ends at that fidelity envelope. The bounded history checker judges
      only single-writer-contiguous-commit-sequence and measures nothing
      after an identity change until both-polarity controls pass again
  svelte-check:
    plane: product-behavior; rung: 1; cost: standing
    note: tsc-backed type collapse for web surfaces; wired as `check`
      in both apps
  vitest:
    plane: product-behavior; rung: 4; cost: routed
    note: web unit tests; wired in pcc-native; the commissioned standard
      for web surfaces as they are built or refactored
  fast-check:
    plane: product-behavior; rung: 4; cost: routed
    status: commissioned-unwired
    note: proptest's color on the TS canvas — seeded, replayable property
      collapse; binds to vitest
  playwright:
    plane: product-behavior; rung: 4; cost: routed
    door: $playwright-cli
    tiers:
      low-level: flat HTML and simple surfaces — loads, buttons, forms,
        basic interactivity; the default tier
      agent-driven: complex orchestrated flows walked by an agent through
        the CLI in steps; pulled only when the interface demands it
    note: the tier split is the velocity control — most public pages need
      only the low tier; never default every interface to the heavy tier
  tauri-webdriver:
    plane: product-behavior; rung: 4; cost: routed
    door: $tauri-linux-webdriver-host
    note: native-shell journeys through tauri-driver/WebKitWebDriver;
      app startup and scenarios belong to the app's local dev harness
  axe-core:
    plane: product-behavior; rung: 4; cost: routed
    status: commissioned-unwired; scope: public-flat-html only
    note: rides inside the playwright low tier as an attached judge;
      collapses the machine-checkable subset of accessibility — the
      remainder is declared residual by nature

parked:                          # not rejected — each has a doorbell
  bolero:      {trigger: decommission of the outgoing storage engine (register: architecture/risk/assurance-db/) — then the real question
                is whether the platform wants a verification or
                engine-portable-harness primitive}
  miri:        {trigger: any first-party memory-level unsafe — unsafe
                impl Send/Sync, transmute, raw-pointer deref,
                MaybeUninit — outside vendored and outgoing trees}
  sanitizers:  {trigger: same family as miri — first real unsafe or FFI
                surface; TSan is the stronger candidate for this
                platform's shape}
  loom-shuttle: {trigger: a hand-rolled lock-free structure or custom
                Sync primitive enters first-party code}
  kani:        {trigger: a named floor-critical algorithmic kernel}
  strykerjs:   {trigger: the TS logic surface grows enough to need
                detector-truth on that side}

excluded-planes:                 # different truths, other homes
  supply-chain: cargo-deny/cargo-audit at infra with the root deny.toml —
    collapses what dependencies bring in, not what this code does
  runtime-validation: protovalidate-class instance checking — a boundary
    decision for its own day, not schema integrity
  coverage: rejected — measures where tests reached, not whether they
    would catch; cargo-mutants dominates it
  security-scanning: the security plane's own tooling
  operational-materialization: native development machine configuration, or
    staging and production Nix closures and activation, belong to 8000 after
    terminal Build and the human capability/environment disposition; they
    realize an artifact operationally but do not collapse its product-behavior
    possibilities or choose placement upstream
  runners-and-internal-tooling: runners are doors; transitional internal
    tooling is register-owned and never enters the legend; Moon may run native
    development build, proof, package, deploy, and verify tasks but is not
    itself a collapser; deployment for Build proof to an existing native role
    remains in the parent lane
```

## Walking It

The walk happens at contract-authoring time, and its output is the
contract's `## Collapse Route` section (owned by
`$3010-contract-authoring`):

- Name the unit's surface class(es); read the possibility classes listed
  there. The row bounds the walk — a small surface walks a short row.
- Every listed possibility class appears in the route exactly once:
  bound through `$collapser` as collapse by construction, a proven
  standing gate, or a commissioned application; otherwise it is
  **declared residual** with a reason. A bare graph id or tool name is
  ornamental routing and remains uncollapsed.
- The claims' oracle refs are already routes; the section's value is the
  classes *beyond* the oracles — what else this surface can do, and what
  forbids or absorbs it.
- The author walks the map; the coder walks the route. Delegates receive
  the walked route inside the locked contract and never need this graph.
- `$3030-counterfactual-contract` walks the same row adversarially: a
  declined edge — a routed possibility declared residual — is a standing
  counterfactual candidate, and its reason is the attack surface.
- `$2040-spike-promotion-packager` names graph ids in first-draft gauge
  specs and candidate application facts, so promotion hands forward
  routes, not vibes. Contract authoring still owns the final binding.

## This Skill Is NOT

- **Not the witness plane:** it defines no evidence format, retention, or
  seam handling — a routed collapser's output is governed where witnesses
  are already owned.
- **Not the oracle plane:** it defines no comparators and mints no oracle
  refs — the oracle seam is owned law, and oracle genesis is 2000 work.
- **Not the application practice:** `$collapser` binds a node to an exact
  boundary, identity, controls, oracle, witness, gate, and residual.
- **Not a runner guide:** procedure lives behind each node's named door;
  this skill grants no mutation authority and no execution procedure.
- **Not risk status:** in-flux instruments carry register entries; nodes
  point, never restate.

## Change Law

- A collapser enters this graph only by human commission with a recorded
  assessment or witness behind it; the initial legend's provenance is the
  1000 research round `runs/1000/1784631450900-6f43-collapse-graph-inventory`
  (census, five instrument assessments, synthesis, and the human ruling
  in its end cap). New nodes cite their evidence the same way — a
  reference, never restated content.
- Nodes exit by subtraction when their instrument leaves or their ruling
  is reversed; parked entries convert when their trigger rings, through a
  recorded human decision.
- `status` marks (unproven, unwired, occupant-pending) are cleared in the
  same change that lands the wiring or the spike verdict — the graph
  never advertises wiring that does not exist.
- Volatility never lives here: an instrument in flux carries an
  `architecture/risk/` entry, and its node points at it.
