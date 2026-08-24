# Sanitization record

Records what was extracted and neutralized for this public pack. It is written
to be publish-safe: private owner identifiers are **not** reproduced here. The
exact original values, full-detail mapping, and the excluded directory names
live in a private validation record kept outside this repository.

## Extraction

- **Source:** a private internal `.agents/skills/` tree, read-only (the source
  was **not** modified — skills were copied, not moved).
- **Method:** per-skill directory copy preserving structure and every file
  (`SKILL.md`, `references/**`, `agents/**`).
- **Selected skill dirs:** 81 at extraction time; **scope-corrected to the
  15-skill 7000-series review campaign** (the original standalone intent)
  before first publication. The 66 non-review skills were pruned from the
  working tree and remain retrievable in git history at the initial commit;
  the list below is the extraction-time record.
- **Excluded skill dirs:** 25 (private-infrastructure / repo-bound, listed by
  category below).
- **Also excluded:** the source `.agents` metadata files (`manifest.yaml`,
  `capabilities/`, `specs/`) and the private hostname-schema reference folder.

## Token substitution

Applied to every text file in the copied tree. The original tokens are
withheld from this public record; each is shown as an opaque marker
(`[owner]`, `[actor]`, `[host]`) denoting a private identifier that was
mechanically replaced without any semantic change.

| Count | Original (opaque) | Replacement |
|---|---|---|
| 14 | `[owner]-runs-validator` | `assurance-runs-validator` |
|  3 | `[owner]-review-frontmatter` | `assurance-review-frontmatter` |
|  1 | `[owner]-review-run` | `assurance-review-run` |
|  2 | `[owner]-claim` | `assurance-claim` |
|  1 | `[owner]-closure` | `assurance-closure` |
|  1 | `[owner]-skill-validator` | `assurance-skill-validator` |
|  2 | `[owner]-db-qualification-ops` | `assurance-db-qualification-ops` |
|  1 | `[owner]-nix-ops` | `assurance-nix-ops` |
|  1 | `[owner]-goal-dockets env` | `ASSURANCE_GOAL_DOCKETS` |
|  1 | `[actor]-code` | `agent-code` |
|  1 | `[actor]-manager` | `agent-manager` |
|  1 | `[actor]-validation` | `agent-validation` |
|  1 | `[host]-llm-gateway-ops` | `llm-gateway-ops` |
|  1 | `[host]-brand prefix` | `LLM` |
|  1 | source repo name | `adversarial-assurance` |
|  1 | `[owner]-db` (risk-register path segment) | `assurance-db` |
|  0 | GitHub account handle | `n/a` (absent in selected scope) |

Plus generic case-variant fallbacks for the `[owner]`/`[host]` prefixes.
File count: 87 text files scanned; 14 edited. A deterministic byte-level
forward check confirms every edited file is an exact application of the table
above — 81/81 `SKILL.md` files, zero unexplained delta.

## Verified clean

- No private owner identifiers or actor handles anywhere in content or
  filenames.
- No real hostnames, provider references, SSH user forms, or private paths.
- No API keys, tokens, credentials, or private-key blocks.
- No real emails or non-loopback IP addresses.
- (Public OpenAI model identifiers such as `gpt-5.6-sol` are retained — they
  are not personal data.)

## Included skills (81)

1000-research; 2000-spike-campaign-orchestrator; 2020-spike-worker-execution;
2030-spike-confirmer; 2040-spike-promotion-packager; 3010-contract-authoring;
3020-contract-validation; 3030-counterfactual-contract;
4010-unit-dag-orchestration; 4020-unit-task-proof-execution;
4030-unit-task-validation; 5020-skill-validation; 6020-document-validation;
7000-code-review-orchestrator; 7005-review-charter-guarantees;
7010-review-recon; 7011-review-recon-scout; 7020-review-triage;
7025-review-test-integrity; 7030-review-integrity-plan;
7035-review-integrity-execute; 7036-review-proof-validation;
7040-review-gapfill; 7050-review-rootcause-trace;
7060-review-feedback-synthesis; 7065-review-assurance-retrospective;
7070-review-report; 7080-review-promotion; agent-run; git-policy;
collapse-graph; collapser; oracle; promise; witness; state-gauge; underwrite;
jump-gate; gravity; situate; reckon; riff; use-derived-state; state-modeling;
skill-authoring; adopted-protocols; search; codex-goal-use; protocol-1000;
protocol-2000; protocol-3000; protocol-4000; protocol-5000; protocol-6000;
protocol-7000; protocol-8000; protocol-9000; protocol-seal; protocol-spawn;
protocol-trunk-and-leaf; protocol-post; protocol-remediate; protocol-repair;
protocol-route; protocol-land; protocol-mint; protocol-passback;
protocol-completion-gate; protocol-cut; protocol-gate; protocol-commission;
protocol-promote; protocol-verify; protocol-predeclare; protocol-qualify;
protocol-resolve; protocol-place; protocol-match; protocol-corroborate;
protocol-declare; protocol-attack.

## Excluded skills (25, by category)

- Build/CI hosts (Linux build host, Tauri WebDriver host, Tauri native
  boundary, media-editing local dev harness)
- Network/ops (tailscale ops, LLM-gateway ops, host provisioning, Nix ops,
  object-storage tooling)
- Database/Rust infrastructure (DB qualification ops, sharding contract, Rust
  API-surface, Rust safety/perf)
- Provider tooling (Playwright CLI, Stripe best practices, frontend,
  podcast RSS contract guard)
- Repo-coupled research utilities (external repo bank, external-source
  research, direct repo scout, rust AFL campaign, rust nextest, moon task
  graph, development environment)

## Independent adversarial validation

Validated by an independent model (openai + agent harness, maximum reasoning):

- **Completeness:** PASS — 81 directories / 87 files, exact source parity, no
  missing or extra files.
- **Fidelity (semantic preservation):** PASS — all content differences are the
  table above; no lines added, removed, reordered, or truncated.
- **Sanitization completeness:** PASS — zero residual private identifiers in
  the published tree (this record and the repository root included).
- **Cross-reference integrity:** PASS — no unexpected dangling references;
  13 dangling targets are intentionally-excluded companion skills the consumer
  repo supplies (e.g. `$llm-gateway-ops`, `$assurance-nix-ops`,
  `$assurance-db-qualification-ops`).

## Status

Published under the pack repository. The extraction-time validation above
describes the 81-skill tree at extraction; the pack has since been
scope-corrected to the 15-skill 7000-series review campaign (see the
Extraction note). Human review remains the gate for any later-phase changes
to the skill set.
