# 4010 Separate Primary Task Runbook

Use this reference when `$4010-unit-dag-orchestration` is supervising live
unit execution and the happy path is no longer obvious. The main skill owns the
contract. This file records the operating projection for route packets,
observation, recovery, and final audit.

## Operating Model

The 4010 primary is the group control plane. It owns the DAG, group run table,
and its assigned group branch. It does not enter a unit branch, re-anchor its
goal to a unit contract, or invoke a 4020 manager as a child.

Each unit runs in a separately commissioned primary task under
`$4020-unit-task-proof-execution`. That task has its own one-unit goal and
matching 4000 branch. Its worker and validator roles follow `$agent-run`; its
Git and provider work follows `$git-policy`.

Pushed unit branches and their branch-native evidence are workflow truth.
Provider state is discovery and integration evidence; hidden task or child
state is never authority by itself.

## Preflight

Before selecting a ready unit:

1. Verify the 4010 goal still points at the group docket and assigned group
   branch.
2. Use authenticated `gh` under `$git-policy` to discover the exact 3000 and
   4000 refs and pin their SHAs.
3. Inspect pinned objects without switching this worktree into a unit branch.
4. Re-fold dependencies and the highest valid generation for every node.
5. Refuse a new route while the previously routed unit lacks a terminal or
   human-blocked branch disposition.

The 4010 primary may update only group evidence on its assigned group branch.
It never repairs a unit worktree, branch, index, provider PR, or implementation.

## Run Table

Keep one row per unit. At minimum, track:

- unit id and dependencies;
- current 3000/4000 generation, exact refs, and pinned SHAs;
- state: `pending`, `amending`, `ready`, `running`, `validating`, `terminal`,
  or `blocked`;
- `blocked_by` and the contract rule that makes the dependency decisive;
- separately commissioned 4020 task reference when known;
- route-packet identity and commission time;
- latest proof and current matching 4030 review paths;
- terminal event ref or exact human blocker; and
- next action.

Refresh the table after every routed task reaches a durable disposition and
before selecting a newly unblocked unit.

## Route Packet

For one ready unit, return a closed packet to the human/operator containing:

- receiver: a new primary task using `$4020-unit-task-proof-execution`;
- unit id and current generation;
- exact locked 3000 ref and lock SHA;
- matching 4000 ref, or the instruction for that new primary to Cut it;
- prior-generation frozen head when applicable;
- collapser application refs and branch-native evidence requirements;
- one-unit goal reference, branch, and terminal condition; and
- the refusal that this 4010 task cannot serve as the 4020 primary or Spawn it
  as a child.

The human/operator commissions the separate primary task. Do not route another
unit until the first task has posted a terminal or human-blocked disposition.

## Observation Loop

Observe a routed unit by re-discovering its provider ref, pinning the current
SHA, and folding branch-native events and evidence. Do not scrape task output,
infer progress from silence, or treat a live conversation as workflow truth.

When proof exists without a current identity-matching 4030 PASS, the unit is
not terminal and remains in its 4020 fix loop. When a higher 3000 generation
appears, freeze recovery on every older 4000 generation and re-derive the
current node state.

## Recovery

### The 4020 Task Is Quiet

Do not replace or duplicate it. Inspect only the pushed unit branch. If no
terminal evidence exists, keep the node `running` or `validating` according to
the branch record.

### The 4020 Task Is Unavailable

Without terminal branch evidence, mark the node human-blocked. A replacement
primary task is commissioned only after the prior task is proven non-live and
the human authorizes recovery of the same current generation.

### Validation Failed

Keep the node in the 4020 fix loop. The group primary does not patch source,
rewrite the verdict, or perform the unit's Git/GitHub closure.

### Branch Or Generation Drifted

Re-discover and pin current provider refs. Never normalize, switch, reset, or
clean a worktree owned by another task. A newer generation suspends the older
one; an ancestry or identity mismatch is visible blocked state.

## Final Group Audit

Before completing the group goal, prove all of these from current pushed
evidence:

- every node is terminal or has an exact human-required blocker;
- every dependency condition was evaluated against its locked contract;
- every terminal unit uses its highest valid generation;
- every task proof has a current identity-matching 4030 PASS;
- no routed task remains in an unresolved `running` or `validating` state; and
- the group branch and final node table preserve every route, disposition, and
  named residual.

Report the final node table, separately commissioned task references, terminal
event and review paths, and exact human blockers. Never merge or promote a unit
or integration PR from the group role.
