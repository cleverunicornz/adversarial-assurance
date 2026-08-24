# Actor-driven expansion-mount bootstrap

Prerequisite: bedrock has already formed the repository and
`seed/substrate-lock.json` advertises Mount Contract version 1. Otherwise:

```sh
assurance init .
```

refuses before writing anything and tells the adopter to update/form bedrock.

On a supported substrate, `assurance init .` writes only
`situation/assurance/`: init template, registry, canonical schema, workflow
template, empty runs directory, and empty-but-present graph manifest. It then
prints a complete `situation/architecture/mount-assurance.yamlld` proposal.
It never writes that bedrock vertex.

The adopting agent asks the human one grouped question:

> What values should fill `lead_model`, `executor_model`, `validator_model`,
> `harness`, `witness_runner`, `reviewer_seat`, and
> `final_validator_seat`?

The agent writes exactly those values under `variables:` in
`situation/assurance/assurance-init.yaml`, resolves and pins this pack's own
repository/ref, retains:

```yaml
substrate:
  contract: "bedrock-expansion-mount/v1"
  minimum_contract_version: 1
```

and sets `status: CONFIGURED`. The bedrock checker pin is never copied here;
the substrate lock owns it independently.

Then:

1. Run `assurance update`. It refreshes only mount-owned canonical files and
   renders `situation/assurance/workflow/assurance.yml` for the configured,
   substrate-approved runner.
2. Copy the rendered template to the consumer workflow location. Require its
   single `assurance-witness` job in repository rules. Retire/migrate any
   separate stale bedrock required workflow so both checks use the same
   substrate lock.
3. Submit the printed ExpansionMount proposal through the normal bedrock
   authoring loop, run `bedrock build`, and commit the registration plus
   bedrock generated output. Assurance never writes `situation/architecture/`.
4. For the first campaign, commit one kebab-case run directory containing
   `run.yamlld`, `evidence/`, and one complete Promise/Witness/Oracle triad,
   then run `assurance build` and commit the run graph/manifest.

`assurance check` fails A001 for a missing/wrong substrate block, unsupported
lock, unapproved runner, missing/placeholder canonical variable, invented
variable, or undeclared record variable.

The required workflow builds independently pinned tools outside the target
checkout and runs bedrock check → assurance check → assurance build →
assurance generated-output gate → bedrock generated-output gate. Its completed
URL is the Witness. Local commands are preflight only and never replace that
two-checker seat.
