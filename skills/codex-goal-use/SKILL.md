---
name: codex-goal-use
description: Goals are the default for all directed work — implementation, research, review, documentation, delegation. A goal is the drift eliminator that keeps agents in motion and re-anchors them to their objective, branch, and required policies every cycle. Use whenever work is directed; spitballing needs no goal. Defines the goal contract (base skill set, branch anchor, contract-or-docket reference), the docket pattern, and lifecycle use of `create_goal`, `get_goal`, and `update_goal`.
metadata:
  short-description: Goals as the default drift eliminator
---

# Codex Goal Use

A goal is the drift eliminator. Agents on long runs lose context as their
window migrates; the goal re-injects why the work started, where it lives,
and what completion means — every cycle. Goals keep the platform's law
true: nothing ever stops; work stays in motion.

## When A Goal Exists

- Goals are the default for all directed work — implementation, research,
  review, documentation, delegation; anything with an objective and a
  stopping condition. If a human or an orchestrator directed the work, a
  goal carries it.
- Spitballing needs no goal: conversation, ideas, questions with no position
  taken. The moment direction lands — "do this," "research that" — the goal
  is created.
- One goal per thread of work. Never create competing goals for the same
  work.

## Identity First

Identity has two axes:

- **Position** — where the actor stands in delegation and Git: orchestrator,
  sub-agent, or spitballer.
- **Role** — the one constitutional role the actor embodies, resolved by
  pointer to `architecture/development-lifecycle/AGENTS.md` ("Actor
  Constitution").

Directed work carries both axes and exactly one role. An undirected
spitballer carries no role or goal; when direction lands, its directed
position and role attach together.

Write the goal relative to your position:

- An **orchestrator** owns a trunk, handles branch operations per
  `$git-policy`, and commissions delegates; its goal points at its unit's
  contract or docket and anchors to its trunk.
- A **sub-agent** follows `$agent-run`; its goal names the assigned ref, role,
  and bounded objective, never the parent's goal.
- A **spitballer** remains undirected until both axes attach.

Order of operations: when the work touches git, read `$git-policy` before
creating the goal — the branch identity (assigned, or to-be-created under
the lane naming) is part of the goal and of the docket filename below.

## A Goal Is A Pointer And A Pulse, Not A Spec

The harness caps goals at 4,000 characters; treat that as design guidance.
Minimal beats more: the goal is pushed into the agent constantly, every
character costs on every cycle, and a fat goal becomes a second source of
truth — the exact drift it exists to prevent.

- A contract exists → the goal points at it: "the contract at `<path>` must
  be fulfilled in full before this goal is complete."
- No contract (the ad-hoc majority) → the invoked agent writes a docket
  (below) from its prompt instructions — unless the caller already supplied
  one — and the goal points at that.
- Goals name skills, never restate their content. Goals point at the
  reference, never carry evolving state: the docket holds the checklist and
  gets checked off; the goal holds the pointer and the pulse.
- Collapser selections, instrument identities, qualification controls,
  oracle refs, witness paths, fidelity envelopes, gate placements, and
  residual belong in the referenced contract or docket, never in the goal.
  The goal points at that state; it does not become a second assurance
  record.

## Goal Contract

Every goal states:

- one concrete outcome
- one concrete stopping condition
- **Branch** — the working ref (assigned, or to-be-created per
  `$git-policy` naming): the anchor that kills wrong-branch drift
- **Reference** — the contract or docket path this goal delivers on
- **Role** — one line naming the constitutional role by pointer, with no role
  prose
- **the base skill set, always**: all agent interaction uses `$agent-run`;
  all git interaction follows `$git-policy`; every actor carries the
  `$gravity` posture; work on a plane governed by an adopted protocol
  carries `$adopted-protocols`
- key constraints and non-goals

Recommended shape:

```text
Objective: <single durable outcome>
Done when: <verifiable stopping condition, re-derived against the Reference>
Branch: <working ref>
Reference: <contract path | docket path>
Role: <constitutional role> per the Actor Constitution
Skills: all agent interaction via $agent-run; all git per $git-policy; carry $gravity; carry $adopted-protocols on governed planes
Constraints: <must keep / must not change>
```

Completion is re-derived, never recalled: the goal closes only by checking
the Reference against current state — files, commits, evidence — not from
the memory of having done the work.

### Delegated Work

Delegated work follows `$agent-run`. The primary goal keeps its own branch
anchor; each child goal names only its assigned leaf, caller-supplied role,
and bounded docket. The goals point to that relationship and do not restate
the handoff.

A primary goal cannot complete while its child is live or the child's
Passback remains undispositioned.

## Dockets — The Reference When No Contract Exists

- Location: `$ASSURANCE_GOAL_DOCKETS` when set, otherwise
  `$HOME/data/goal-dockets/`. Create the directory on first use. Never
  inside any repository — dockets are operational material, not repo
  content.
- Name: `<epoch_ms>-<branch-with-dashes>.md` — the working branch
  (assigned, or the branch this work will create) with `/` replaced by `-`.
  The filename is the lineage breadcrumb: docket ↔ branch, back-parseable
  by any agent or human.
- Content — a closed artifact (no open questions; unclear means ask before
  writing): Role; Objective; Deliverables checklist; Done-when; Constraints;
  Risk — the register entries touched (`architecture/risk/…`) or "none
  registered"; Evidence expectations.
- Every docket names the constitutional role its executor embodies as
  `Role: <constitutional role> per the Actor Constitution`. A caller supplies
  this field for a delegate; a delegate never casts itself, and its goal
  carries the caller-supplied role.
- The docket is the working memory: check deliverables off as they land.
  The goal stays small and points here.
- Dockets are never deleted by agents. The epoch prefix makes pruning a
  human act and trivial, and the folder is a free audit trail of every
  ad-hoc commission.
- Delegates are goal-required. A native child creates a bounded goal that uses
  the caller-supplied docket as its Reference and names only its assigned ref
  and caller-supplied role.

### Authority-Bearing Dockets

A docket states commission, boundaries, provenance, and exclusions; it never
scripts the executor's reasoning procedure.

A docket that transcribes rulings a human has already made carries a
`Provenance` block naming three parts:

- **Decision** - the human's, already made.
- **Preparation** - the executor as scribe.
- **Ratification act** - the named human act, normally the merge.

Without that block, a docket claims no decided-ruling status.

A docket never asks its executor to validate the docket itself and never
conditions a stop, refusal, or other terminal act on the executor's own
interpretation of law. Validation is a separate actor examining a produced
artifact; an executor's reading of its own commission is not validation. A
conflict the executor reads between the docket and landed law returns as a
question quoting both sentences, with the unit held open.

At authority seams, docket sentences prefer the platform's own minted
vocabulary. A mechanical verb never carries an authority meaning within reach
of halt language. Every sentence able to halt work carries its defusal in that
same sentence, never elsewhere in the docket.

### Venue

`architecture/development-lifecycle/AGENTS.md` ("Venue") owns the category and
actor rule; this skill owns only the docket shape. A docket for work touching
more than one host or any non-default access carries a `Venue` section with
exactly these fields:

| Field | Required content |
| --- | --- |
| **Suns used** | The governing field, always first: for each presupposed capability, cite the adopted authority that owns it from `architecture/protocols/AGENTS.md`. For source identity and transport, use Git at pinned SHAs with no other mechanism admitted on that plane; for durable data, use the S3 plane. |
| **Hosts touched** | Each host with its admitted profile per `architecture/risk/development-environment/`. |
| **Access and credentials assumed** | The access paths and credentials the work presupposes. |
| **Cross-host seams** | Each seam's named mechanism, direction, and owner. |
| **Readiness checks** | The checks the orchestrator runs before Spawn. |
| **Teardown** | What is removed or released at Seal. |

Worker rule: "An actor that finds its venue unready, or its commission venue-silent where the preceding rule requires specification, stops and reports a venue defect; the actor performing that commissioned act owns no venue or topology decision."

## Tool Rules

- Use `get_goal` first if you need to recover or confirm the current thread goal state.
- Call `create_goal` only when no active goal already covers the required work.
- Do not set a token budget unless the user or parent workflow explicitly requires one.
- If a token budget is explicitly requested, treat it as a limit, not as proof of completion.
- Use `update_goal(status="complete")` only when the objective is actually achieved and no required work remains.
- Use `update_goal(status="blocked")` only after the same blocker has persisted for three consecutive goal turns and meaningful progress is not possible without user input or an external change.
- Do not mark a goal complete because progress was made, tests partially passed, or budget was exhausted.
- Delegates own bounded child goals; a child-goal owner never creates or completes a parent goal.

## Failure Posture

- If the work needs a goal but the objective is vague, shape the contract or
  docket first; do not create a loose goal.
- If goal tools are unavailable, block and report the environment posture
  instead of pretending the thread is goal-owned.
