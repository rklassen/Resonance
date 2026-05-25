# ./resonance/readme.md

Goal is to reinforce our humanstate metric using visual sentiment analysis as
an auxiliary set of chains in our knowledge graph. 

This will help reinforce our view of the causal relationship between humanstate
and context. The understanding is needed to appropriately accomodate human
interfaces in the local superorganism, however composed andor defined.

We are not building the complete graph. This workshop entry intends to
synthesize learnings from clinical studies, and apply advanced cognitive
models, sparsely for shortest path implementation.

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

## Important distinctions

1. Temporary works vs epic requirements.
2. Contrast
   a. Development harness as natural language instructions for coding agents
   b. Rust code to deploy the ingestigations
3. Endurants (requirements, procedure) vs currency, state-view
