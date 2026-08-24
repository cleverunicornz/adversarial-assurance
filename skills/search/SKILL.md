---
name: search
description: Repository search, file discovery, source reading, evidence gathering, symbol lookup, and structural code search with normal Codex shell tools. Use whenever Codex needs to inspect repository files, find symbols, map callers or references, prepare implementation or review work, answer broad codebase questions, search external checked-out source, or choose a precise read strategy. Prefer `rg`, `git grep`, `fd`/`find`, `git ls-files`, `sed`, `nl`, `cat`, and required `ast-grep` for structural searches.
metadata:
  short-description: Normal repo search and structural reads
---

# Search

Use normal Codex shell tools for repository discovery, source reads, evidence
gathering, and structural code search. `ast-grep` is a fleet development
baseline tool on `PATH`; use it whenever syntax-aware matching is valuable. If
it is missing, report development-environment drift and restore the exact
`.codex/tools/manifest.yaml` identity through the authorized machine profile.
Do not silently recast a commissioned structural question as an `rg`-only
lexical search. A question that was lexical from the outset may proceed with
`rg`. Always invoke the structural CLI as `ast-grep`, never as `sg`: on Linux
`/usr/bin/sg` is the unrelated shadow-utils tool and the name collision fails
confusingly.

## Classify Before Searching

State the question class in one sentence before running commands; the class picks the tool and keeps independent agents on the same evidence path:

- Navigational (where does X live): `git ls-files`/`fd` plus the repo layout — `crates/`, `services/`, `applications/<domain>/`, `tools/`, `proto/`, `infra/`, `architecture/`.
- Lexical (a string, identifier, path, config key, log or error text): `rg`.
- Structural (a code shape — call sites, signatures, impls, attributes, nesting): `ast-grep`.
- Relational (who calls/implements/reads X; cross-surface impact): compose the two — `rg` shortlist, `ast-grep` filter, bounded reads.

## Declare Search Scale

Before the first search command, state a compact search contract:

- the exact question controlling the next answer or action;
- whether the claim concerns existence, absence, cause, extent, or impact;
- the initial evidence neighborhood warranted by that claim;
- the decisive marker that would answer it; and
- the condition that would warrant widening.

Scale follows the question's proof burden. It is neither "search as little as
possible" nor "search until nothing else can be found." The initial
neighborhood must be capable of answering the question, and a search widens
only while a named unresolved question could still change the answer or next
action. Increasing confidence by accumulation is not by itself a widening
condition.

## Decisive Anchors

A directly inspected source fact or retained event becomes an anchor after its
provenance, envelope, and relevance have been checked. It establishes exactly
what it demonstrates:

- one witnessed event proves that the behavior can occur within its envelope;
- it does not alone prove frequency, universality, cause, extent, or impact;
- a declaration that the behavior should not occur cannot negate the event;
  and
- an absence claim requires a closed inventory or authoritative ownership
  boundary; an empty search result is never enough.

The predeclared question and decisive marker are the comparator. When an
anchor satisfies that marker, stop searching for additional confirmation.
Continue only when cause, extent, impact, or remediation remains part of the
commissioned question. Before widening, name one plausible result and how it
would change the answer or next action. If no result could do so, the search is
decision-invariant and must stop.

## Default Search Ladder

1. Establish scope with `pwd`, `git status --short --branch`, `git rev-parse --show-toplevel`, and the nearest applicable `AGENTS.md` overlays. Prefer the narrowest owning surface; go repo-wide only when the question is genuinely repo-wide.
2. Discover files with `git ls-files`, `fd`, or `find`, respecting ignored/deprecated paths unless the task explicitly asks for them.
3. Sample before dumping: get counts and file lists first (`rg -l`, `rg -c`). Above roughly 30 files or 100 matches, narrow the scope or refine the pattern instead of paging through output.
4. Fetch bounded matches with `rg -n` and at most `-C 2` context (`--max-count` caps runaway files). Use `git grep` when searching tracked content or historical stages is the better fit.
5. Search structure with `ast-grep` when the query depends on syntax, language constructs, nesting, call shapes, imports, attributes, JSX/TSX, or refactor-safe matches.
6. Read narrowly with `sed -n`, `nl -ba`, or `cat` for small files. Rust implementations concentrate in `lib.rs`/`main.rs` by repo policy — start there. Keep reads bounded around the evidence you need.
7. Reconcile search hits with direct file reads, tests, and command output before making claims.

Working evidence is a map, not a dump: carry `path:line` plus a one-line summary forward, never raw match floods.

## Command Map

- Files tracked by Git: `git ls-files '<pathspec>'`
- Filename discovery: `fd '<name-or-regex>' <path>`
- Sample first: `rg -c '<pattern>' <path>` (counts) and `rg -l '<pattern>' <path>` (files)
- Bounded text matches: `rg -n -C 2 --max-count 20 '<pattern>' <path>`
- Regex with type filters: `rg -n --glob '*.rs' '<pattern>' crates services applications`
- Tracked text search: `git grep -n '<pattern>' -- <pathspec>`
- Bounded read: `sed -n '<start>,<end>p' <path>`
- Numbered bounded read: `nl -ba <path> | sed -n '<start>,<end>p'`
- Full small-file read: `cat <path>`
- Structural search: `ast-grep run --lang rust --pattern 'fn $NAME($$$ARGS) { $$$BODY }' <path>`
- Structural capture (see what is passed where): `ast-grep run --lang rust --pattern '$CLIENT.run_workflow($$$ARGS)' --json=compact <path>`
- Relational match without a rule file: `ast-grep scan --inline-rules '{id: x, language: rust, rule: {pattern: "$F($$$)", not: {inside: {kind: macro_invocation, stopBy: end}}}}' <path>`
- Shortlist then filter: `rg -l '<name>' crates services | xargs ast-grep run --lang rust --pattern '<shape>'`
- Structural YAML/JSON-adjacent work: prefer parsers or purpose-built CLIs when available; otherwise combine `rg` wayfinding with bounded direct reads.

## Structural Search

Use `ast-grep` for questions such as:

- Find functions, impls, traits, handlers, commands, routes, invocations, imports, attributes, derives, enum variants, or test declarations.
- Find a construct with required descendants, ancestors, or sibling context (rule mode: `inside`, `has`, `not`).
- Distinguish comments/strings from real code.
- Check whether a refactor pattern still exists after mutation.

Language support is verified against the pinned `ast-grep` 0.43.0, not assumed: `rust`, `typescript`, `tsx`, `javascript`, `html`, `css`, `json`, `yaml`, `bash`, and `nix` are accepted `--lang` values. `svelte`, `toml`, and `proto` are not supported — use `rg` for `.svelte`, `.toml` (including `Cargo.toml`), and `.proto` files.

Start with a broad structural pattern, then narrow with path scope, `--lang`, and follow-up `rg` or bounded reads. Treat `ast-grep` results as wayfinding until direct source reads confirm the claim.

## Structural Footguns

- A pattern must parse as a valid standalone snippet of the target language or it silently matches nothing. Example: `--lang nix` with pattern `$K = $V` returns zero because a bare binding is not a nix expression, while `builtins.map $F $L` matches. When a pattern unexpectedly returns zero, run `--debug-query=ast`: an `ERROR` node means the pattern, not the code, is wrong.
- Structural search cannot see through macros. Code inside macro invocation bodies (`println!(...)`, `info!(...)`, domain macros) parses as raw tokens, and macro-, derive-, or build-script-generated code (tonic/prost services, `OUT_DIR` output) is not in the source tree at all. Use `rg` in macro-heavy and codegen zones.
- An empty structural result is therefore never evidence of absence. Confirm every negative claim ("X is never called", "no impl exists") with `rg` before stating it.

## Search Discipline

- Use the smallest path scope that still answers the question.
- Prefer parallel independent reads/searches when the results do not depend on each other.
- Do not search generated, vendored, build, cache, target, or deprecated trees unless the task explicitly includes them.
- When search results disagree, trust direct source reads and rerun narrower queries.
- For external repositories, use `$external-repo-bank` (the shared persistent clone bank) for source inspection instead of scattered temporary clones.
