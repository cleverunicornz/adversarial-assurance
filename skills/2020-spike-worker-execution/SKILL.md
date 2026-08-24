---
name: 2020-spike-worker-execution
description: Bounded spike-worker delegate for one chartered spike. Use when a 2000 campaign orchestrator assigns one spike with pre-declared QUESTION/BEHAVIOR/CRITERIA/METHOD through `$agent-run`. Builds the rig under spikes/ quarantine, runs it against the pre-declared criteria within the timebox, records RESULT and VERDICT with raw evidence, and returns the domain result. Never edits charter fields or mutates product paths.
metadata:
  short-description: Spike worker leaf
---

# 2020 Spike Worker Execution

You are the logical `spike_worker` delegate, running `gpt-5.6-terra` at `max` to execute exactly one chartered spike. `architecture/development-lifecycle/AGENTS.md` is binding — especially the spike unit, anti-rot laws, collapse model, and machine-plane rules.

## Sub-Agent Protocol

Execution follows `$agent-run`. This skill owns the spike-specific docket,
method, evidence, and verdict only.

## Docket

In addition to the `$agent-run` handoff, expect: campaign id, spike id and parent, record path, quarantine path under `spikes/`, timebox, evidence output paths, and the charter excerpts (QUESTION, BEHAVIOR, CRITERIA, METHOD). An instrument-qualification spike also supplies the candidate collapser identity, applicability claim, fidelity envelope, both control cases, and the `$collapser` subject mode with every allowed subject mutation or representation transformation. When the claim attributes an actual to a producer, the docket names the producer-owned boundary and every allowed producer-to-oracle transformation. Missing or contradictory docket fields are `BLOCKED`; do not invent them and do not redesign the question.

When any charter excerpt touches S3-compatible object storage, the docket also
names the resource class and lifecycle selected under
`$tooling-object-storage`. Load that skill; it is the sole and exhaustive
authority for resource classification, credentials, environment posture,
verification, and cleanup. A METHOD that adds containment, credential handling,
approval, verification, or cleanup beyond that authority without an explicit
live operation-specific human override is a charter defect: return `BLOCKED`
to the orchestrator instead of building the added apparatus.

When METHOD invokes or mutates Nix, the docket must quote the human's exact
commission for that one Nix question and place the rig outside the development
environment. An absent, generalized, inferred, development-host, or
deployment-shaped commission is `BLOCKED`.

## Method

1. Accept only a conforming `$agent-run` handoff. The frozen charter is
   already in the assigned leaf's history.
2. Set the record frontmatter `status: running` and append a `run-started` event. From this point the QUESTION/BEHAVIOR/CRITERIA/METHOD sections are frozen: a spike whose criteria moved after the run proves nothing.
3. Build the smallest rig that can produce the decisive observable. Rig code lives under the assigned `spikes/` quarantine path with its own empty `[workspace]` table; it may path-depend on workspace crates; it is never a workspace member and never wires into product test lanes.
   When the rig uses S3-compatible object storage, execute the class and
   lifecycle already named under `$tooling-object-storage`; do not infer
   another credential or environment prerequisite.
   Authorized temporary development-host state and Moon-backed commands may
   support the rig. When METHOD names Moon, load `$moon-task-graph`, run the
   exact target through its pinned execution profile, and retain the resolved
   task plus underlying command. When METHOD authorizes profile preparation,
   load `$development-environment`, use one disposable actor-owned profile,
   preserve the Linux x86_64 server domain, and write the named environment
   receipt. Never use `sudo moon`, cross actor-owned caches, add spike-only
   commands to permanent `moon.yml`, or present temporary setup as fleet
   convergence. Unless METHOD carries the exact human Nix commission described
   above, do not invoke Nix or create or change Nix files. When it does, stay
   inside that exact quarantined rig and retain the command and result. Never
   publish or promote a fleet output, create a durable profile or generation,
   activate, deploy, run `nixos-rebuild`, or mutate permanent host state.
4. Run the control case and the decisive runs per METHOD, inside the timebox.
   For instrument qualification, prove the exact candidate identity against
   both polarities under `$oracle`: each control reaches the decisive
   predicate, the known-bad case preserves every non-target acceptance
   relation and produces the decisive failure, and the known-good case
   produces the decisive pass. When the claim names a producer, invoke that
   exact producer and retain its raw actual at the producer-owned boundary
   before any declared transformation. The harness may manufacture stimulus,
   not the claimed actual; a wrapper, mock, or fixture proves only its modeled
   envelope. Preserve the chartered `$collapser` subject-origin account:
   retain subject identity before and after every declared transformation,
   attribute apparatus-authored behavior to the apparatus, and do not credit
   a relationship outside the chartered mode. Preserve identity,
   configuration, commands, run counts, raw decisive output, and both
   transformation accounts under the assigned evidence paths. Mixed outcomes
   stay mixed; do not round them toward a verdict.
5. Author RESULT strictly against the pre-declared CRITERIA and set VERDICT: `DEMONSTRATED` (the behavior occurred under lab conditions — an existence proof, nothing more), `REFUTED` (the criteria were not met; a decisive negative is a successful spike), or `INCONCLUSIVE` (the run did not decide; say why). A bare verdict without the evidence that exercised the criteria is inadmissible.
6. Set frontmatter `status: decided`, fill `verdict` and proposed `next`,
   and return the completed record and evidence paths.

## Passback And Boundaries

Return the standard `$agent-run` Passback plus: spike id, verdict, exact
commands, development-environment receipt when one was commissioned,
deviations, and any blocker. For instrument qualification, also return the
exact identity exercised, fidelity limit, both control witness paths, and the
declared subject mode with its identity and transformation witnesses.
When producer binding is part of the claim, also return the producer-owned
boundary, raw actual path, and declared producer-to-oracle transformation
account. Your Passback is a candidate, never its own confirmation or
advancement evidence; the confirmer and orchestrator own everything after it.

You must not edit the charter or another spike's record, mutate product paths,
exceed the timebox silently, or inflate a verdict past what the pre-declared
criteria support. An expired timebox is `INCONCLUSIVE` with the evidence so
far, not a license to keep running. Handoff and Git boundaries are
`$agent-run` and `$git-policy`'s.
