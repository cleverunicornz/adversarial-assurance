---
name: state-gauge
description: The gauge practice for state protocols — how a design package's proof surface becomes executable gauges, and how a backend is qualified beneath a protocol. Use when building or qualifying proofs for replay determinism, typed refusal, fencing under contention, fault recovery, stale-reader coherence, projection generation-fencing, or checkpoint equivalence; when a 2000 campaign originates state-gauge apparatus; when a 3000 contract binds state claims; when 4000/7000 work re-manufactures or attacks them; or when deciding whether any backend may serve the substrate for a protocol. A specialization of $oracle, $collapser, and $witness for state; gauges ship with the protocol or the protocol is not complete.
metadata:
  short-description: State gauges — proofs up, conformance down
---

# State Gauge — The Proof Practice

Load `$use-derived-state` first; the canon's proof law governs: a
protocol's semantics are constituted by its outward proofs, and an
advertised state protocol without its gauges fails the mandatory
completion gate. This skill specializes the existing organs — `$oracle`
(comparators, known-answer controls, pinning), `$collapser` (bound
applications, instrument identity, fidelity), `$witness` (retained,
re-manufacturable evidence) — for state protocols. Their law applies
unchanged; nothing here relaxes it.

Genesis is 2000 work: the rig that proves a protocol behavior is the
gauge's first draft, and it rides the campaign's promotion package.
An unproven gauge measures nothing — both-polarity known-answer controls
before any verdict counts, on the exact instrument identity that will
run.

## The Gauge Classes

Every state protocol's proof surface draws from these classes; the design
package (`$state-modeling`, question 10) names which apply and what each
decisive observable is. Per protocol, the contents are the application's
own — classes are shared shape, never shared vocabulary.

- **Fold determinism.** Replaying the same history yields the same
  semantic state, byte-stable where the package declares it. The fold is
  the re-derivation law made testable.
- **Refusal admissibility.** Every inadmissible transition the grammar
  names receives its exact type, and refused operations are absent from
  history — both halves observed, not assumed.
- **Fencing under contention.** Two writers race one head: exactly one
  winner, one typed loser, and the loser's transitions absent from
  advanced history — provable by reading the head, never by trusting a
  clock or lease bookkeeping.
- **Recovery under fault.** A fault cut before the head advances leaves
  no partial truth; process loss followed by fresh-owner reconstruction
  from the substrate alone converges to the fold of durable history.
  Recovery is the ordinary read path and is gauged as such.
- **Stale-reader coherence.** A reader at generation N serves consistent,
  declared-stale truth; starved of every hint it is stale, never wrong,
  and converges on its next head read.
- **Projection generation-fencing.** A projection serves only against the
  generation it declares; its rebuild equals the fold's answer for that
  generation; eviction loses nothing canonical.
- **Checkpoint equivalence.** Recovery through a checkpoint equals replay
  from history to the same generation — materialization is an
  optimization, never a second truth.

## Anchoring Law

- Every gauged invariant is stated against logical time — the head
  sequence, a named generation — never a wall-clock window.
- Every gauged invariant is provable from the surface by an ordinary
  caller: an API boundary or a head read. If the gauge needs a drill
  rig — stopped worlds, cross-process clock correlation, private engine
  state — the clause is wrong; rewrite the clause against the head.

## Pointing Down — Backend Conformance

Gauges run in both directions. No backend serves the substrate for a
protocol until it passes the same suite under contention and fault —
conditional-write decisiveness, typed refusal behavior, durability of the
publish-then-swap topology. Backend admission runs under
`architecture/object-storage/`'s qualification and conformance ownership
(register: `architecture/protocols/AGENTS.md`). The platform already runs a proven
conditional-write gauge lineage at the object floor; extend that
apparatus, never reinvent it, and route any new or changed instrument
through 2000 qualification like every other collapser.

## Shipping And Pinning

- Gauges ship with the protocol: the contract names their witness paths
  before execution, 4000 delivers them as committed artifacts, 4030
  re-manufactures them against current source, and human-invoked 7000
  campaigns attack them.
- When a gap closes, the gauge pins at its contract-named gate placement
  and is never weakened, skipped, or un-advertised as a fix.
- Witness law applies whole: dies-mute apparatus is built wrong; a green
  run that retained nothing observed nothing; bounded absence rides every
  exclude claim.

## Relationships

- Governs: the canon's proof law; posture via `$use-derived-state`.
- Specializes: `$oracle` (origination, controls, pinning), `$collapser`
  (the eight application questions apply to every gauge binding),
  `$witness` (evidence shapes and homes).
- Consumes: the design package's proof surface (`$state-modeling`).
- Status: `architecture/risk/use-derived-state/` — no floor gauge suite
  exists; per-protocol gauges are the only state gauges there are.

## This Is NOT

- Not the oracle practice itself: origination, proof, and pinning law
  live in `$oracle`; this skill names the state classes that practice
  builds.
- Not a coverage mandate: one decisive comparator per named observable
  outweighs any pile of assertions observing nothing decisive.
- Not the floor gauge suite: none exists, and building one before two
  real protocols converge under a human gate is a violation, not
  initiative.
- Not a runner or harness catalog: execution rides the lanes and tools
  the contract names.
