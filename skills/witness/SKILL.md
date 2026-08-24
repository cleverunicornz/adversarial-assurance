---
name: witness
description: The witness practice — what counts as the retained observation of a collapse event, what shapes witnesses take, where they live, and the rules that keep them honest (producer binding, dies-mute, re-manufacture, staleness, bounded absence). Use when naming witness paths in a 3000 contract, delivering evidence in 4000 execution, validating in 4030, or judging whether a claimed result was actually observed. A test manufactures the event; the witness is what the event leaves behind; the oracle judges it. A green run that retained nothing observed nothing.
metadata:
  short-description: Retained evidence — the observer's rules
---

# Witness

The witness is the observer: the retained, inspectable record that a
collapse event happened. Without it there is no observer, and an
unobserved event does not exist for the purposes of this repository —
"it ran and passed" in prose is a promissory note, and the lifecycle
does not bank promissory notes (seam law, dev-lifecycle overlay).

## What A Witness Proves

Existence, and only existence: the promise held in the one run the
witness records. No pile of witnesses proves exclusivity — that
"nothing else happens" is bought on the upper rungs, never testified
to (root law). Treating a green suite as completion is the standing
failure mode; a witness locates behavior, it never forbids it.

## Shapes And Homes

Witnesses are committed artifacts at contract-named paths:

- test output bound to a decisive assertion; snapshots; property-run
  regressions and their seeds
- rigs, fixtures, corpora — the apparatus that can re-manufacture the
  event on demand
- proof documents, command transcripts with exit codes, JSONL events
  on the unit branch
- for `exclude` claims: **bounded absence** — the forcing regime and
  budget ride the claim forever; "we tried" never hardens into
  "cannot happen"

Homes: unit evidence lives under `runs/<block>/<id>/` per the runs
grammar; pinned regression artifacts and rigs live beside the
implementation they guard. The contract names expected witness paths
before execution (3010 law); a deliverable with no witness path is not
yet a deliverable.

Worked (schematic, same chain as `$promise` and `$oracle`): the
oversize-refusal oracle runs. Its witnesses: the committed test
(re-runnable on demand), the oversize fixture that manufactures the
event, and the run's record in the unit's events — each at a
contract-named path. The fixture is what makes the event
re-manufacturable; without it the green run is a photograph of a place
nobody can revisit.

## The Rules

- **Dies-mute.** A witness that dies mute destroyed itself: any run
  built so failure erases the record — logs, marks, partial
  completions — is built wrong (root law). Retention is part of the
  apparatus, not a courtesy.
- **Re-manufactured, never recalled.** Validation re-derives witnesses
  against current source; cited output, transcripts, and memory of a
  prior run are not evidence (4030 law). If it cannot be re-made, it
  is history, not proof.
- **Staleness.** A diff that touches a claim's surface marks its
  witnesses stale; stale claims re-collapse before dependent work
  builds on them (dev-lifecycle law). A witness is a dated photograph,
  honest only inside its frame.
- **The seam rule.** No assertion crosses a stage seam unwitnessed;
  residue crosses only by name.
- **Application identity.** A witness names the exact collapser
  application that manufactured it. Output from another version,
  configuration, baseline, topology, or fidelity envelope is a different
  event and cannot be silently substituted.
- **Producer binding.** A witness names the actor whose behavior it claims to
  observe, and the claimed actual crosses a boundary owned by that actor. A
  harness may manufacture stimuli and retain or route the resulting actual;
  it may not manufacture, repair, enrich, relabel, or substitute that actual
  while attributing it to the producer. A recorder may add recorder-owned
  envelope metadata, but that metadata is not producer output. Grammar,
  hashes, and internal consistency establish representation or integrity,
  never origin. Every transformation between producer and oracle is part of
  the application identity and independently qualified; otherwise the
  artifact is a fixture, not a witness of the named producer.

## Relationships

- Observes the events oracles judge (`$oracle`); its shapes are chosen
  when the route is walked (`$collapse-graph`); `$collapser` binds those
  shapes to an exact application and fidelity envelope; its paths are
  named at authoring (3010), delivered in execution (4020),
  re-manufactured in validation (4030).

## This Is NOT

- Not the judge: a witness records; it renders no verdict.
- Not instrument qualification: retained output cannot prove that its
  producer was applicable, faithful, or capable of both verdicts.
- Not coverage: more witnesses is not more exclusivity.
- Not prose: an unretained observation, however sincerely reported,
  is not a witness.
