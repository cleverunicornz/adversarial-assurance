---
name: adopted-protocols
description: The protocol-authority checker. Resolve before authoring, locking, or recording a verdict on any work that touches a plane governed by an adopted protocol (root Protocols section; register at architecture/protocols/AGENTS.md). Runs four closed questions — conformance, authority direction, pin integrity, reserved names — and returns PASS or a typed BLOCK quoting the violating sentences verbatim. A spike whose method violates the governing protocol is INVALID regardless of outcome; adoption-evaluation campaigns run the same check against the candidate's own pinned specification. Ships both-polarity known-answer controls that a session must demonstrate before its live verdict counts.
metadata:
  short-description: Check prose and contracts against adopted-protocol authority
---

# Adopted Protocols

Root's Protocols section is the law; the register at
`architecture/protocols/AGENTS.md` is the map; this skill is the
point-of-act check. It exists because prose is inferable and adopted
protocols are not: an adopted grammar is followed, never adjudicated, and
any artifact that points the arrow backwards must be stopped at the gate
it reaches first, not discovered later by luck.

## When It Runs

Resolved before authoring, locking, or verdict on any work touching a
governed plane. Binding points by lane:

- **1000** — before a research verdict is recorded.
- **2000** — twice: at charter lock (a hypothesis may not contradict the
  governing protocol) and at answer-ledger recording (see spike law).
- **3000** — at 3010 preflight; as a mandatory 3020 validation dimension;
  as a 3030 attack surface.
- **4000** — 4030 runs the authority-direction question over the diff:
  does any change assert against, reimplement, or rename a governed
  surface outside its binding architecture.
- **5000** — before Seal on any skill whose text touches a governed plane.
- **6000** — at authoring and at package evaluation.
- **7000** — before the lane rubric, beside the completion gate.

The lane and authoring skills carry these bindings at their own gates; this
section is the index. Where a surface has no family skill, this section plus
the root ruling bind directly.

## The Four Questions

Closed set. Asked of the artifact as written, against the register and the
owning architecture binding — never against memory or inference.

1. **Conformance.** Does every claim, hypothesis, method, and result
   conform to the protocol as pinned? An artifact may stage, consume, or
   scope the protocol only in the ways its owning architecture admits.
2. **Authority direction.** Does any sentence define, narrow, reinterpret,
   condition, or gate the protocol — or derive authority over its plane
   from anything but the protocol itself? Dependency edges, residuals,
   sequencing notes, delivery timelines, and consumer contracts carry no
   authority over an adopted plane, ever.
3. **Pin integrity.** Does the artifact cite the recorded pin with no
   silent version drift? Pin movement is a human ratification act recorded
   at the register, never an artifact side effect.
4. **Reserved names.** Does the artifact coin a name, spelling, or
   near-name that could confuse what the authority is? A deviation must
   take a distinct name and holds none of the protocol's authority under
   it.

## Spike Law

A spike whose method violated the governing protocol returns **INVALID
regardless of outcome** — a run that was not playing the adopted game
proves nothing about it. When the campaign's question is itself "do we
adopt X," the same four questions run against the candidate's own pinned
specification: a result obtained by violating the candidate is invalid as
evidence about the candidate.

## Verdict Grammar

**PASS** or **BLOCK**. Nothing between — no concerns, no notes, no
middle verdict.

A BLOCK is typed and carries, or it is itself invalid:

- the violating sentences, quoted verbatim with path and line;
- the exact root-law or register clause each violates;
- the question (1-4) that fired.

Every BLOCK routes to the human. The checker never edits the artifact,
never proposes the fix wording, never adjudicates protocol content, and
never decides adoption — it compares and reports.

## Instrument Law

The checker is an agent, so it is proven like any instrument: both
polarities, every session. Before a live verdict counts, the session
demonstrates BLOCK on `controls/known-bad.md` and PASS on
`controls/known-good.md`. The controls are permanent and incident-pinned;
they are never edited to make a verdict easier. A session that cannot
produce both control verdicts produces no live verdict.

## Boundaries

- Ground truth is the root Protocols section, the register, and the owning
  architecture binding — nothing else, supplemented by nothing.
- Ambient protocols are checked only at their register scope lines; their
  internals are their own specifications' business.
- This check runs beside lane rubrics, never instead of them.
