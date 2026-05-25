# ./resonance/readme.md

Goal is to reinforce our humanstate metric using visual sentiment analysis as
an auxiliary set of chains in our knowledge graph. 

This will help reinforce our view of the causal relationship between humanstate
and context. The understanding is needed to appropriately accomodate human
interfaces in the local superorganism, however composed andor defined.

We are not building the complete graph. This workshop entry intends to
synthesize learnings from clinical studies, and apply advanced cognitive
models, sparsely for shortest path implementation.

## Wayfinding

```text
resonance/
|- readme.md                 - repo entry point, engineering rules, and navigation.
|- Cargo.toml                - Rust package manifest for the runtime crate.
|- Cargo.lock                - locked Rust dependency graph.
|- rustfmt.toml              - formatter policy for Rust source.
|- artifacts/
|  |- readme.md              - artifact-level notes.
|  |- alpha/                 - phase-alpha outputs and support artifacts.
|  |- beta/                  - phase-beta fixtures and generated artifacts.
|  `- gamma/                 - phase-gamma artifacts and outputs.
|- contracts/
|  |- readme.md              - contract entry point and category map.
|  `- *.md                   - role contracts for artifact, probe, payload,
|                             operator, state, requirement, gate, trace,
|                             claim, and output.
|- plan/
|  |- developer_document.md  - working developer handoff and execution notes.
|  |- implementation-plan.rs - natural-language implementation plan.
|  `- implementation-plan.snap
|                           - Snap-encoded plan DAG and phase graph.
|- procedure/
|  `- verify-lm.md           - verification procedure and scoring discipline.
|- specification/
|  |- engineering-rules.md   - locked engineering constraints.
|  |- requirements*.md       - active requirements and split section-11
|  |                          supplements.
|  `- snap-spec-0.8.md       - stable Snap DSL reference.
|- src/
|  |- lib.rs                 - crate export surface.
|  |- alpha/                 - alpha-phase runtime implementation.
|  |- beta/                  - beta-phase privileged-path implementation.
|  |- gamma/                 - gamma-phase shell and extension seams.
|  `- *.rs                   - shared contracts and top-level data records.
|- tests/
|  `- *_phase.rs             - sparse phase-level integration tests.
`- schmae/                   - auxiliary schema area; currently repo-local and
                               separate from the main specification tree.
```

## Markdown Index

Currency is omitted from this flat list.
`plan/developer_document.md` is a current working handoff and state-view, so it
is intentionally excluded from the enduring repo index.

- [readme.md](readme.md)
- [artifacts/readme.md](artifacts/readme.md)
- [contracts/readme.md](contracts/readme.md)
- [contracts/artifact.md](contracts/artifact.md)
- [contracts/claim.md](contracts/claim.md)
- [contracts/gate.md](contracts/gate.md)
- [contracts/operator.md](contracts/operator.md)
- [contracts/output.md](contracts/output.md)
- [contracts/payload.md](contracts/payload.md)
- [contracts/probe.md](contracts/probe.md)
- [contracts/requirement.md](contracts/requirement.md)
- [contracts/state.md](contracts/state.md)
- [contracts/trace.md](contracts/trace.md)
- [procedure/verify-lm.md](procedure/verify-lm.md)
- [specification/engineering-rules.md](specification/engineering-rules.md)
- [specification/requirements.md](specification/requirements.md)
- [specification/requirements-11-role-set.md](specification/requirements-11-role-set.md)
- [specification/requirements-12-phase-claim-trace.md](specification/requirements-12-phase-claim-trace.md)
- [specification/requirements-13-contract-alignment.md](specification/requirements-13-contract-alignment.md)
- [specification/snap-spec-0.8.md](specification/snap-spec-0.8.md)

## development process

The development is planned in three principal phases with intermediate
observation points as defined in the implementation plan and directed graph
encoded as a snap file. 

Consider the development DAG. If you do not see any fatal flaws in it, treat it
as locked. Rectify the other degrees of freedom in machine inference to fit
this, not the other way around.

🔒 Do not edit without explicit user instructions.

## requirements

The epic goals are defined in `specification/requirements.md`

the process to develop and test is defined in `./plan/`. these are temporary
works that describe how a developer may arrive at the epic goal and are not
relevant once the development is complete.

Procedures needed to effectively perform the worker finding procedure the
principle of these is verify which describes a graph walk and of critical
importance and evaluation formula that differs from the evaluator that is
integrated in inferential machines' default runtime code.

In all these documents, we are performing the work in a more intentional and
structured manner. Our principles relate to talkative simplicity, determinism,
stateless execution, and cognitively minimal integration into more complicated
systems, the top position and functional requirements of which we do not know
at this time. This requires humility and gnosticism about the strict runtime
environment, and or data sets that we will be processing.

Pay special attention to things that seem surprising or unusual within the
context. If we are stating them as hard requirements, then it is because those
constraints are carrying real engineering weight and should not be rounded away
by priors.

Engineering rules:

1. Keep all files under a 400-line hard cap.
   Exception(s): `specification/snap-spec-0.8.md`.
   The Snap DSL reference remains whole even when it exceeds the general
   document cap.
2. Evaluate file length only after the language auto formatter has produced the
   final form. For Rust, judge the file after the normal vertical formatter
   style has been applied.
3. Prefer vertical layout with as few elements per line as practical so fields,
   match arms, arguments, and variants remain readable.
4. Do not try to satisfy line caps by condensing whitespace, packing multiple
   elements onto one line, or otherwise fighting the formatter. That is an
   anti-pattern.
5. Run Rust validation and executable checks in `--release` by default.
   Formatting is the exception because `cargo fmt` does not have a release
   mode.
6. Keep functions attached to stateless structs at minimum. If an object is
   already defined then prefer object methods, but determinism and immutability
   remain principal goals.
7. Do not introduce `Kind` buckets in type names or fields. Name the actual
   role instead, such as `mode`, `status`, `phase`, `contract`, or another
   concrete term. `Kind` is treated here as a cognitive-complexity smell and a
   design failure mode because it hides meaning behind a generic bucket.

## Important distinctions

1. Temporary works vs epic requirements.
2. Contrast
   a. Development harness as natural language instructions for coding agents
   b. Rust code to deploy the ingestigations
3. Endurants (requirements, procedure) vs currency, state-view
