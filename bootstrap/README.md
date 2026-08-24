# Actor-driven bootstrap

The adopting agent runs:

```sh
assurance init .
```

`init` installs `.assurance/`, the canonical schema copy, an explicitly empty
registry, and `.github/workflows/assurance.yml`. It does not guess bindings and
does not claim the adoption is ready.

The agent then asks the human one grouped question:

> What values should fill `lead_model`, `executor_model`, `validator_model`,
> `harness`, `witness_runner`, `reviewer_seat`, and
> `final_validator_seat`?

These seven names are the complete canonical variable set. The agent writes
the answers only under `variables:` in
`.assurance/assurance-init.yaml`. It also resolves this pack's repository
coordinate, pins the current full commit SHA, replaces every placeholder, and
sets `status: CONFIGURED`. The human does not need to supply values the pack
itself can resolve.

Skills use `{{variable}}` references and resolve them from this block before
acting. `assurance check` fails A001 when a canonical variable is absent,
empty, or still a placeholder; when the init file invents a variable; or when
a record contains an undeclared `{{variable}}`.

Commit `.assurance/` and `.github/workflows/assurance.yml` together. A
repository administrator must configure the workflow's `assurance-required`
job as a required merge check; adding a workflow file alone is not branch
protection. Workflow execution builds the pinned pack source, then runs
`assurance check` and `assurance build` on the bound runner. That CI log is the
witness.

Agents may run check/build manually while authoring. A local invocation is
preflight only: it is never evidence, never a merge gate, and never a fallback
when the required CI witness fails.
