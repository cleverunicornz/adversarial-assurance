---
name: 1000-research
description: Directed research on its own 1000 lane — investigate repositories, crates, seams, external projects, or internal behavior and report findings, with no mutation intent. Use when research is directed and no behavior needs proving. A 2000 spike proves or refutes behavior with a rig; the moment answering requires building, realization, mutation, or an empirical run, stop and route to 2000. Read-only Nix inspection or evaluation is admitted only by an exact human commission and outside development. Every research effort gets its own 1000/ branch and ends in a recorded verdict, even when the verdict is no-action.
metadata:
  short-description: Directed research lane — investigate and report, never mutate
---

# 1000 — Research

Research is the read-only uncertainty stage: a directed investigation that
answers a question by reading and reasoning, never by mutating. It is used
when knowledge must be derived, not manufactured as a predecessor for settled
human direction. This lane exists to end a specific observed failure —
research requests escalating into spike ceremony because no lane existed for
"find out and report."

## Runtime Posture

The 1000 research actor and every delegated reader run `gpt-5.6-terra` at
`max`. A directly launched 1000 task under another posture stops and tells the
human to relaunch it with that exact model and reasoning effort; every reader
Spawn carries the same exact pair in its `$agent-run` invocation.

## Boundary

- Research reads, searches, and reports. It never mutates product paths,
  never builds rigs, never proves behavior by running it.
- Nix operational materialization is never a research deliverable or
  completion condition. Reading existing Nix source is ordinary research. A
  read-only Nix inspection or evaluation is admitted only when the human
  commissions that exact action and it runs outside development; retain the
  command and result. It never builds, realizes, publishes, activates,
  deploys, mutates a profile, or becomes permission for another Nix action.
- The moment the question can only be answered by building, running, or
  mutating something, it has crossed into spike territory: stop, record
  that verdict, and route to a 2000 campaign.
- Casual questions in conversation are spitballing, not research — no
  branch, no goal. Research begins when it is directed ("research X for
  me" is directed, whatever the subject).

## Flow

1. Read `$git-policy`; cut the research branch
   `1000/<epoch_ms>-<uid4>-<slug>/trunk` per its naming. Research is
   normally single-agent — the trunk alone, no leaves — unless it runs bounded
   delegate readers through `$agent-run`.
2. Create the goal per `$codex-goal-use`: the research question is the
   objective, the branch is the anchor, the docket carries the question
   decomposition and the done-when.
3. Investigate with the read-only toolkit — `$search`, `$direct-repo-scout`,
   `$external-repo-bank`, `$external-source-research`. Load
   `$moon-task-graph` when the question concerns how a surface builds, tests,
   validates, or deploys: read the registered projects, targets, and
   dependency relationships without executing them. Load
   `$development-environment` when the question concerns fleet tool posture,
   Codex convergence, server build domain, or machine bootstrap: inventory
   source and observed receipts read-only, but do not install, repair, or
   converge a host. Checked-out source is the evidence authority; docs and
   summaries reconcile against it.
4. When the question or verdict touches a plane governed by an adopted
   protocol (root Protocols; register: `architecture/protocols/AGENTS.md`),
   resolve `$adopted-protocols` before recording: the verdict records its
   PASS; a BLOCK stops the verdict and routes to the human.
5. Record the outcome on the branch under `runs/1000/<unit-id>/`:
   - Findings warranted → commit the research document: question, method,
     findings with source citations, verdict, proposed next.
   - Nothing warranted → one checkpoint commit whose message records the
     verdict ("researched X — no action needed"). The branch name itself is
     the index entry; the commit is the receipt.
6. Push. The branch is the record either way. Future agents discover prior
   research by listing `1000/` branches — the lane is the research index.

## Promotion

- Research branches are dead ends like every lane. Findings reach main only
  when a human wants them promoted, through a `merge/*` branch per
  `$git-policy`.
- A verdict that supplies enough closed knowledge for a decided architecture
  direction routes directly to a 6000 promise unit; 2000 is not mandatory.
- A verdict that exposes an empirical unknown, requires execution, or needs
  oracle genesis routes to a 2000 campaign. If its finding is human-promoted,
  the resulting direction then routes to 6000.
- Both lawful paths are therefore ordinary:
  `1000 -> 6000` and `1000 -> 2000 -> 6000`. Direct human design may begin at
  6000, and an empirical question may begin at 2000.
- Promised work routes to 3000 only after the owning `architecture/` source
  has landed and the contract's claims can be minted from it. A 1000 report,
  2000 package, or human promotion event is input to that promise source, not
  a substitute for it.
