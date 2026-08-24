---
name: agent-run
description: Use whenever a repository agent runs a native sub-agent. Owns the
  bounded handoff between one primary and one child, covering the closed docket,
  assigned-leaf authority, native invocation, Passback, reclaim, and
  fail-closed recovery. Does not own workflow roles, goal law, or Git/GitHub
  procedure.
node: agent-run
class: skill
edges:
  - type: cites
    target: codex-goal-use
    provenance: declared
  - type: cites
    target: git-policy
    provenance: declared
metadata:
  short-description: Native sub-agent handoff
  class: policy
  edges:
    guarded-by: [git-policy, protocol-match, protocol-predeclare, protocol-resolve]
    follows: [protocol-cut]
    closed-by: [protocol-passback]
    cites: [protocol-spawn]
    indexed-in: [git-policy]
---

# Agent Run

This is the single home for repository sub-agent execution. A skill that needs
a child names the role and docket it owns, then follows this skill; it does
not restate this handoff.

## Relations

- The calling workflow skill owns the role, model, reasoning effort, bounded
  objective, allowed content, checks, evidence, and verdict vocabulary.
- `$codex-goal-use` owns the primary and child goals.
- `$git-policy` owns every branch, worktree, Git, GitHub, Post, Land, Seal,
  cap, cleanup, and human default-branch rule.
- This skill owns the transfer of one prepared worktree from the primary to
  one native child and back.

## Closed Docket

Before handoff, the primary resolves:

- role skill, model, reasoning effort, objective, and stop condition;
- assigned leaf, exact opening SHA, canonical worktree, and allowed paths;
- contract or docket reference, required authorities, pinned source refs,
  expected outputs, and required checks; and
- the required Passback fields and role-specific disposition vocabulary.

Missing, open, or contradictory fields refuse the handoff. The child receives
this docket, not the primary conversation history.

## Assigned-Leaf Boundary

The primary prepares and publishes the assigned leaf under `$git-policy`,
places the clean shared worktree on its exact opening SHA, and remains the only
Git and GitHub actor.

The child may edit only the allowed paths and run docketed checks. It may
inspect the assigned leaf and pinned repository objects read-only with commands
such as `git status`, `git rev-parse`, `git show`, `git grep`, and
`git diff`. It does not move `HEAD` to research another branch; a required
ref change returns `BLOCKED` so the primary can facilitate it between
handoffs.

The child never changes the index, `HEAD`, branches, worktrees, refs,
remotes, commits, or provider state. It never commits, pushes, Posts, Lands,
Seals, caps, cleans up, or delegates another child.

Every child prompt states the assigned leaf, opening SHA, allowed paths, and
this boundary explicitly, and requires the child to load this skill,
`$git-policy`, its role skill, its docket, and `$codex-goal-use`.

## Native Handoff

Invoke one child through native `spawn_agent` with `fork_turns="none"` and
the role's exact model and reasoning effort. If that identity cannot be
expressed, refuse rather than substitute.

Only one child may be live in the shared worktree. While it is live, the
primary may observe or relay explicit human steering, but makes no repository
mutation and changes no branch or worktree state.

## Passback And Reclaim

The child returns:

```text
Passback: COMPLETED | BLOCKED | FAILED | CANCELLED
Assigned leaf: <leaf>
Opening SHA: <sha>
Files changed: <paths | none>
Decisions: <bounded decisions>
Checks run: <exact commands | none>
Exact results: <results>
Direct conflicts outside scope: <items | none>
Residual risks: <items | none>
```

After the exact child is terminal, the primary reclaims the worktree and
verifies the assigned leaf and opening `HEAD` did not move, the index is
empty, no Git operation is in progress, and every change is within the docket.
Only then may it accept the Passback and resume Git handling through
`$git-policy`. Primary-owned transport never changes the child's semantic
verdict.

Invocation success is not a domain PASS. A missing, malformed, or
out-of-scope Passback does not advance the workflow.

## Recovery

Elapsed time and partial output are not terminal evidence. If child state is
missing, contradictory, or still live, preserve the worktree and make no Git
mutation, cleanup, replacement, or retry.

A terminally failed or cancelled handoff preserves its partial evidence for a
primary-owned disposition. Failure does not commission replacement, retry, a
new leaf, or another validation pass. When the active workflow commission
already admits a replacement, or the human expressly commissions one, it is a
new context-free child on a newly prepared leaf. An independent validation
pass admitted by the active workflow commission or expressly commissioned by
the human likewise uses a fresh child against its exact immutable candidate.
