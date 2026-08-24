# Actor-driven bootstrap

The adopting agent runs:

```sh
assurance init .
```

`init` installs `.assurance/`, the canonical schema copy, an explicitly empty
registry, and `.github/workflows/assurance.yml`. It does not guess bindings and
does not claim the adoption is ready.

The agent then asks the human one grouped question:

> Which model should fill lead, worker, validator, and reviewer; which harness
> or harnesses should launch them; which runner label should host the CI
> witness; and who occupies the final-validator and reviewer actor seats?

The agent writes those answers only to
`.assurance/assurance-init.yaml`. It also resolves this pack's repository
coordinate, pins the current full commit SHA, replaces every placeholder, and
sets `status: CONFIGURED`. The human does not need to supply values the pack
itself can resolve.

Commit `.assurance/` and `.github/workflows/assurance.yml` together. A
repository administrator must configure the workflow's `assurance-required`
job as a required merge check; adding a workflow file alone is not branch
protection. Workflow execution builds the pinned pack source, then runs
`assurance check` and `assurance build` on the bound runner. That CI log is the
witness.

Agents may run check/build manually while authoring. A local invocation is
preflight only: it is never evidence, never a merge gate, and never a fallback
when the required CI witness fails.
