# GitHub Enforcement Configuration

The mechanical half of the git policy. Applied by a human with repository
admin access; agents never modify rulesets. Once applied, every rule below
is deterministic — no agent judgment, no bypass.

## Ruleset 1 — Lane integrity (targets: `1000/**`, `2000/**`, `3000/**`, `4000/**`, `5000/**`, `6000/**`, `7000/**`, `8000/**`, `9000/**`, `merge/**`)

- Block force pushes.
- Restrict deletions.
- Bypass list: empty.

## Ruleset 2 — Default branch (target: `main`)

- Require a pull request before merging; direct pushes blocked.
- Required status check: `merge-gate` (the workflow in
  `.github/workflows/merge-gate.yml`).
- Block force pushes; restrict deletions.
- Bypass list: empty. An empty bypass list means no actor — admin included —
  can merge a PR while `merge-gate` is red. Changing that is editing the
  ruleset itself: legislation, not a door.

## Ruleset 3 — Ref namespace (creation rules)

- Restrict branch creation to: the nine lane prefixes (`1000/**` through
  `9000/**`), `merge/**`, and the harness scratch namespaces in use
  (currently `claude/**`).
- Scratch namespaces are never mergeable (merge-gate enforces this
  mechanically) and are never lanes.

## The merge-gate check

`.github/workflows/merge-gate.yml` — a deterministic head-ref assertion, no
model, no judgment. GitHub's native branch protection cannot filter PRs by
source-branch name; the required check is the standard mechanism, and with
Ruleset 2's empty bypass it is as absolute as GitHub allows: a PR from any
non-`merge/*` head is permanently unmergeable by every actor class.

## Squash-merge default

In repository settings, set squash merging as the default (and preferably
only) merge method for pull requests. This is what keeps trunk history out
of main's graph and makes every landing a single revertable commit.

## Application Order — Two Stages, Not One

- Stage one, safe immediately: Ruleset 1 (force-push and deletion blocks on
  the lane patterns and `merge/**`). Pure protection; restricts nothing and
  affects no ref outside those patterns. Enabling squash as an available
  default is also safe, with merge commits left enabled for now.
- Stage two, the cutover — applied together, only after full quiesce:
  Ruleset 3 (creation allowlist), the `merge-gate` required-check flip on
  main, and squash-only. Creation rules fire when new refs are born, and
  in-flight work creates branches mid-flight — a campaign that cannot cut
  its next branch is a stalled campaign. Quiesce means: every campaign
  terminal, every branch committed and pushed, zero open PRs from
  non-`merge/*` heads, nothing running anywhere that would ever need a
  pre-lane ref again. From the flip forward, the law is live and everything
  new is born inside it.
