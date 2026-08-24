---
name: git-policy
description: The repository's single Git and GitHub operating policy. MUST be read before any git action or GitHub provider operation. Defines authenticated gh use, remote lane discovery, the lane index system (1000-9000), trunk-and-leaf procedure, branch naming, merge/* as the only path to main, the direction law, and recovery. Root AGENTS.md binds it as a hard invariant.
node: git-policy
class: skill
edges:
  - type: cites
    target: 1000-research
    provenance: declared
  - type: cites
    target: 5020-skill-validation
    provenance: declared
  - type: cites
    target: 6020-document-validation
    provenance: declared
  - type: cites
    target: agent-run
    provenance: declared
  - type: cites
    target: protocol-trunk-and-leaf
    provenance: declared
  - type: cites
    target: search
    provenance: declared
metadata:
  short-description: Mandatory git operating policy for all agents
---

# Git Policy

Root `AGENTS.md` binds this skill as a hard invariant: read it before any git
action. This file is the single home of git policy — nothing elsewhere
restates it; other skills and overlays point here.

## Absolutes

- Force push does not exist. Not to main, not to lanes, not to anything
  pushed. There is no scenario, no exception, and no actor for whom it
  exists.
- Nothing pushed is ever deleted. Lane branches, leaves, and `merge/*`
  branches are permanent records.
- Lane branches are never local-only: commit and push as you work. The
  remote is the restore point that makes every mistake recoverable.
- Only `merge/*` branches reach the default branch, by human-merged PR.
  No other index can merge to main by any path, for any actor. Agents never
  push to or merge the default branch — ever.
- Rebase exists only for local, never-pushed commits. A pushed ref is never
  rebased by anyone.

## The Work Protocol Vocabulary

The closed word set for work exchanges. A word enters only on a named
failure, observed twice independently, with its refusals defined at
entry; words deprecate, never delete; guards stay guards. This section
outgrowing one page is the drift alarm. Derivation:
`runs/1000/1785193617565-14df-verb-canon-sweep/` (evidence, cited for
reasoning, never authority).

Exchange verbs — complete acts with typed refusals:

| Verb | Act | Refused when |
| --- | --- | --- |
| Mint | Record an identity required by the active commission, carrying its because-edge | No provenance edge or no commissioning context |
| Cut | Create an already-commissioned branch from a pinned parent SHA | Branch not commissioned; parent unpinned or drifted |
| Spawn | Delegate an already-commissioned bounded docket at a pinned branch; complete only when the child's lineage is durable before any blocking wait | Docket not commissioned; no branch or durable lineage |
| Post | Commit and push to the intended target | Checked-out state differs from intended target |
| Land | Integrate finished work into the surface that continues: leaf → trunk, merge/* → main | Landing ungated or red work |
| Passback | Close a Spawn with a typed result: witness refs, exact commands, named residue — never narrative | Bare verdict; prose handoff |
| Qualify | Admit one exact instrument identity into service via both-polarity known-answer controls | Single polarity; product name for identity; crediting a changed identity |
| Corroborate | A disjoint actor in a disjoint workspace re-manufactures a claimed witness | Same actor, clone, or cache; author transcript as evidence |
| Declare | Record an un-collapsed possibility: scope, owner, reason | Silent residue; mixed outcome rounded to a verdict |
| Commission | Bind explicit human-requested work or an active-Closure act to an owner with judge and finite envelope | No resolving judge or envelope; inferred or self-grounding commission |
| Route | Classify what this unit cannot lawfully continue and name a possible receiver without creating work there | Unnamed receiver or condition; self-supplied predecessor; treating the route as a commission |
| Promote | Human-only: convert non-binding material into binding obligation through the human gate | Any agent performing it; assembly from recollection |
| Place | Install a qualified comparator as a standing gate, by contract or human decision | Unqualified instrument; placement by side effect; weakening a placed gate as remediation |
| Seal | Terminally close a run, log, or unit — dispositions: promote, discard, park | Obligations neither collapsed nor Declared |

Guards — preconditions bound to verbs, never standalone acts:

| Guard | Discipline | Binds to |
| --- | --- | --- |
| Match | Actual state equals intended state before any mutation | Post (mandatory), Spawn, Land |
| Verify | Confirm by regenerating from source; a read of the claim is never Verify | Land, Seal |
| Gate | A named check inside an existing commission; acceptance enables only an already-admitted act, and refusal creates no recovery | Land, merge, Lock, Seal |
| Predeclare | Criteria, decisive observables, and the finite sets they range over are fixed before the run that answers them | Spawn, Gate, Qualify |
| Attack | Before crediting a claim, construct its counterexample and fail to make it fire | Land, Seal |
| Resolve | A reference binds to a real referent from current source, or refuses | Mint, Lock, Spawn, Land |

Repair pair: Repair (contract meaning changes) / Remediate (evidence or
hygiene, no meaning change). Canonical elsewhere: Poll (agent-run,
pull-driven status); Lock (bind a name to an immutable SHA — the
contract lock is its named case; "pin" is its deprecated alias). Fold
belongs to the state plane exclusively — derive current state by
replaying immutable history — and is not a git act. Deprecated
synonyms converge on re-author, never by sweep: delegate, dispatch,
assign, launch, fan out → Spawn; terminal-close spellings → Seal.
The mechanical form of Post is `scripts/post`, which refuses unless
the checked-out branch matches the intended target. Each word's
point-of-act form is its matching protocol node skill — nodes own
guard bindings, mechanical forms, and graph edges; this section stays
the index and the lock-point for amendments.

Only Commission creates work. A route, gate result, refusal, finding, edit,
Post, merge, or available budget never creates a branch, unit, pass, review,
repair, retry, delegate, or receiving act.

## GitHub Provider Surface

Authenticated local `gh` is the only normal GitHub provider surface. Do not
route repository, branch, PR, issue, check, release, or API operations through
provider wrappers, fixed actor catalogs, or sub-agent delegation. The
authenticated `gh` account is the actor; never infer identity from the
repository owner, remote URL, branch name, commit author, or local username.

Before the first GitHub operation in a run:

```bash
command -v gh
gh auth status --hostname github.com
gh api user --jq .login
gh repo view --json nameWithOwner --jq .nameWithOwner
```

Record the returned login and repository name, never a token. Missing or
ambiguous authentication stops the provider operation with the failed command
and remediation. Machine bootstrap may run `gh auth setup-git` to configure
Git's credential helper for the authenticated account; ordinary work does not
rewrite credential configuration speculatively.

Use `gh pr`, `gh issue`, `gh run`, `gh release`, and `gh api` directly. Before
a provider mutation, re-check `gh api user --jq .login` if the run is
long-lived or authentication may have changed. Agents may create and update an
explicitly commissioned integration PR, but the human alone merges it.

## Remote Lane Discovery

Pushed lane branches are durable workflow state even when they have no PR and
no local tracking ref. Discover them from the forge, not from local branch
memory:

```bash
repo="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"
lane="1000"
gh api --paginate "repos/${repo}/git/matching-refs/heads/${lane}" \
  --jq '.[] | [.ref, .object.sha] | @tsv'
```

Use the applicable lane prefix (`1000` through `9000`, or `merge`) to bound the
listing. For one selected branch, require an exact ref match, pin its SHA,
fetch the named ref, and verify that the fetched object is the discovered
object:

```bash
branch="1000/<unit>/trunk"
repo="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"
sha="$(
  gh api "repos/${repo}/git/ref/heads/${branch}" --jq .object.sha
)"
test -n "${sha}"
git fetch origin "refs/heads/${branch}"
test "$(git rev-parse FETCH_HEAD)" = "${sha}"
```

A mismatch means the remote ref moved between discovery and fetch: rerun
discovery and decide against the new SHA; do not silently inspect the old
assumption. Once pinned, use normal Git and `$search` against the SHA
(`git log`, `git show`, `git grep`) without creating a local branch merely to
read it. The rule is: `gh` discovers the remote ref and pins its SHA; Git reads
it.

## Lanes — Git As A Database

Every unit of work lives in exactly one lane. The lane is the index; the
same trunk-and-leaf procedure governs all of them. Each lane has a matching
protocol-code node (procedure node: `$protocol-trunk-and-leaf`); nodes
own the routing refusals at each door and the lifecycle edges.

| Lane | Work |
|---|---|
| `1000/` | Research — directed investigation with no mutation intent (family skill: `$1000-research`) |
| `2000/` | Exploratory — spikes and campaigns (family skill defines its flow) |
| `3000/` | Contract work (family skill defines its flow) |
| `4000/` | Behavioral / implementation work (family skill defines its flow) |
| `5000/` | Skill changes — the index of all agent-skill mutations (validation delegate: `$5020-skill-validation`) |
| `6000/` | Documentation changes — overlays, register, architecture prose (validation delegate: `$6020-document-validation`) |
| `7000/` | Review work (family skill defines its flow) |
| `8000/` | Operational materialization - after runnable behavior is terminal, or directly for machine substrate work, the human selects or admits one capability and environment. Development uses native Ubuntu configuration; staging proves a Nix closure; production promotes that same closure. Test apparatus and Moon development proof deployment ride their parent lane. |
| `9000/` | Miscellaneous — the named escape hatch for work that fits no other lane (hot fixes, small one-off changes) |
| `merge/` | The only mergeable index. Nothing else lands on main. |

The 1000/2000/3000/4000/7000 lanes have workflow family skills that define
their internal choreography; the block families keep their established
branch conventions (e.g. `2000/<campaign-id>/00-campaign`). The
5000/6000/8000/9000 lanes have no family skill: the orchestrating agent
runs the universal procedure below directly; a 5000 or 6000 unit reaches its
merge branch only through a current-candidate `PASS` from its lane's validation
delegate, run through `$agent-run`. Each lane protocol owns the same default
envelope: an initial independent Disposition followed, when needed, by at most
two complete Disposition-to-Disposition corrective loops. `PASS` stops early;
an exhausted non-accepting Disposition stops at the human decision boundary.
Harness scratch namespaces (currently `claude/*`) exist for
session bootstrap only; they are never mergeable and carry no work product
that isn't also flowing through a lane.

## Branch Naming

For lanes without an established family convention:

- Trunk: `<lane>/<epoch_ms>-<uid4>-<slug>/trunk`
- Leaf: `<lane>/<epoch_ms>-<uid4>-<slug>/leaf/<epoch_ms>-<slug>`
- Merge: `merge/<lane>-<epoch_ms>-<uid4>`

`<epoch_ms>` is the creation time in milliseconds — fixed width, so lexical
sort is chronological sort and every branch carries its own timestamp for
free. `<uid4>` covers the one collision window (two orchestrators minting
trunks in the same millisecond). Leaves need no uid: the trunk's single
writer serializes their creation, so their epoch alone is unique within the
unit. The trunk is a typed child (`/trunk`) because a git ref can never be
the path-prefix of another ref — leaves could not nest under a bare unit
path. Everything under `<lane>/<unit>/` is one unit.

### 3000/4000 Active-Lineage Addresses

The contract and Build branches for a material unit use `3000/<unit-id>` and
`4000/<unit-id>`. Situated Closure introduces no successor-generation grammar.
After contract Lock, an agent changes the contract or either Closure only on
one exact human instruction; the active 4000 lineage's sole writer commits and
pushes those bytes directly before work resumes. The change creates no 3000
re-entry, new branch, review, or successor.

Existing `-g<N>` refs remain immutable historical evidence under the procedure
that created them. They are never renamed, deleted, backfilled, or interpreted
as active Situated Closure mechanics. Moving an in-flight lineage to the new
model requires exact human direction; the move still occurs on its active
branch without minting another generation.

## The Trunk

- The orchestrator creates one trunk per unit of work and is its only
  writer. One writer per ref, everywhere, always — a non-fast-forward
  rejection on your own trunk means a second writer exists: stop and report;
  never force.
- The trunk is the unit's last-known-good state and the orchestrator's
  viewport: builds, Moon, clippy, and comparisons run against it. It stays
  known-good because of the landing gate below.
- The trunk is the only absorber: newer main arrives by merging main into
  the trunk. This is safe precisely because the trunk is a dead end — its
  history never enters main, so back-merges can never pollute it.
- The trunk never merges to main. At completion, cut the unit's `merge/*`
  branch from the trunk and append the trunk's final commit recording that
  branch name — the unit's exit record. The trunk then stands forever as
  the complete, walkable account of the work.

## Leaves

- Every delegated mutation follows `$agent-run`. This skill owns the leaf and
  every Git/GitHub act; `$agent-run` owns the handoff to the child.
- Landing gate: a leaf merges into the trunk only when accepted by the lane's
  already-commissioned bar (build, targeted clippy, or its bounded validation
  disposition). The
  orchestrator is the ticket-taker; ungated landings are how a trunk decays
  from known-good to merely-latest.
- Leaves never absorb anything. Not main, not the trunk. A leaf that needs
  newer material is refreshed by re-cut-and-land-forward: cut a NEW leaf
  from the current trunk, merge the old leaf INTO the new one (old work
  flows forward; trunk-side content is truth in conflicts), abandon the old
  leaf in place as evidence, continue on the new leaf. No rebase exists in
  this procedure.
- Remove a leaf-specific disposable working copy after landing when one exists;
  the shared campaign worktree follows the handoff lifecycle below. Never
  delete the refs.

### Native Shared Worktree Handoff

Sub-agent execution follows `$agent-run`. Its shared-worktree handoff changes
no rule in this skill: the primary remains the only Git/GitHub actor, ref
movement occurs only between completed handoffs, and every leaf remains a
permanent lane record.

## Merge Branches — The Only Door

- `merge/<lane>-<unit-id>` is cut from the trunk at completion. Final
  adjustments may happen here; it is integration class.
- Create the late integration PR with authenticated `gh`, base `main`, and the
  exact `merge/*` head. Verify its reported head SHA, base, draft state, merge
  state, and required checks through `gh pr view` / `gh pr checks`; never treat
  a local branch name or intended push as proof of provider state.
- If main moves before the PR lands: merge main into the `merge/*` branch,
  or cut a fresh `merge/*` from the refreshed trunk. Cross-unit conflicts
  surface here, in the one place built for them.
- `merge/*` PRs squash-merge by default: the trunk's history never enters
  main's graph, main stays clean, and the revert point is one commit.
- `merge/*` branches are never deleted after merging. Squash means their
  commits exist nowhere else; deleting one would dangle the trunk's exit
  record.
- Stop at the human merge gate. Agents do not run `gh pr merge`, enable
  auto-merge, or otherwise convert a ready PR into a default-branch mutation.
- An independently commissioned follow-on unit cuts its trunk from `main`
  after any required predecessor merge lands. Common admissible orderings
  include `4000 -> 5000`, `4000 -> 8000`, and `8000 -> 6000`; no predecessor
  result or merge creates the follow-on.

## The Direction Law

Merges flow in exactly two directions; everything else is forbidden.

- Landings — finished work into the surface that continues: leaf → trunk,
  old-leaf → new-leaf, `merge/*` → main.
- Refreshes — the world into a designated absorber: main → trunk, and
  main → `merge/*` for late conflicts. Nothing else absorbs: never
  main → leaf, never trunk → leaf, never anything → a finished lane.

Post-Lock human-directed governing edits add no merge direction, Cut, or
absorber. They are committed and pushed by the active 4000 lineage's sole
writer. Git history retains the superseded bytes; no review or other workflow
is inferred from the edit.

## Sequencing — Promise, Behavior, Materialization, Pronouncement

- Research (1000) owns directed read-only uncertainty when it exists, recorded
  on its own branch even when the outcome is "no action needed." Settled human
  direction need not manufacture a research predecessor. The moment answering
  requires building or mutating, classify it as possible 2000 work and stop;
  create no spike campaign.
  Read-only Nix inspection or evaluation remains 1000 only under an exact
  human commission; it creates no build, realization, profile, release, or
  fleet state.
- Promise-plane documentation (6000) precedes 3000 whenever architecture is
  missing. Closed knowledge may be classified `1000 -> 6000`; empirical
  uncertainty may be classified `1000 -> 2000 -> 6000`. These are admissible
  orderings, not commissions.
- Behavioral change lands first (4000). A skill for behavior that has not
  landed on main describes nothing that exists.
- Independently commissioned skill changes may follow (5000), written against
  landed behavior so the skill's names and procedures are real on arrival.
- Operational materialization (8000) is a separate, optional follow-on for
  terminal runnable product behavior and the direct lane for commissioned
  machine substrate work. Before any mutation, the human selects or admits one
  capability and environment; an agent never decides placement upstream.
  Development materializes natively, staging proves a Nix closure, and
  production promotes that same closure. A 1000 or 2000 Nix exception runs
  outside development and only under its exact human commission. A 3000 or
  4000 unit contains no Nix file, invocation, output, package, profile,
  closure, cache action, or Nix proof. A wired 4000 Moon task may deploy an
  exact development artifact to an already commissioned native development
  destination for proof, but it creates no durable machine authority and
  never crosses into staging or production.
- Reconciliation documentation (6000) pronounces landed reality: it removes
  transitional state, updates the risk register, and asserts that the written
  world matches the actual one. It follows whichever behavior, skill, or
  operational fact it describes; documentation is how risk changes.
- Spikes (2000) move no documentation - a spike proves or refutes and may
  never touch `main`. Its answer may classify possible 6000 documentation
  before 3000, but creates neither unit.
- Skills and documentation may also change standalone, in their own lanes,
  under the same procedure.

Every relationship in this section states admissible order only. A result,
finding, route, Post, or merge never commissions the next lane.

## Recovery Playbook

Every failure below is local-only, because the remote can never be
rewritten and everything is always pushed.

- Rebased a pushed ref locally: `git reset --hard origin/<ref>` restores
  remote truth instantly. Salvage any never-pushed commits from
  `git reflog` with `git cherry-pick`. Do not "revert" a rebase — reset to
  the remote.
- Non-fast-forward rejection pushing your trunk: a second writer exists.
  Stop, fetch, report to the human. Never force.
- Leaf went sideways: abandon the ref where it stands. Re-cut or carry work
  forward only when the active commission already admits another leaf or the
  human expressly requests it; failure itself creates no retry.
- A human-directed governing edit was Posted on the wrong active lineage:
  preserve the pushed commit, stop, and report the exact mismatch. Never
  reinterpret it as authority, create a successor to hide it, or resume until
  the human names the governing active state.
- Local working copy mess of any kind: the remote ref is the restore
  point — reset to it.
- Accidental commit of the wrong content, already pushed: it is permanent.
  Land a corrective commit; never rewrite. If a secret was pushed, report
  to the human immediately — rotation, not history surgery, is the
  remediation.

## Enforcement

Mechanically enforced on GitHub (rulesets plus the `merge-gate` required
check; exact configuration: `references/github-enforcement.md`): force
pushes blocked everywhere, deletions blocked everywhere, branch creation
restricted to the lane prefixes plus `merge/*` and harness scratch, main
accepts PRs only, and the merge-gate check fails every PR whose head is not
`merge/*` — with an empty bypass list, so no actor can merge past it.
Everything directional (this file's laws) is procedural: enforced by this
skill being read, by goals naming it, and by orchestrators gating landings.
