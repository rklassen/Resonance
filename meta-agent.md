# Adverse inference + synthesis

V0.2 [ `p3ba` ~~ed0w~~] 2026-05-25T22:20Z

Your counteranalysis is correct: most proposed “fixes” are suspect because they are likely already present as natural-language guidance. If the agent already had “verify,” “preserve invariants,” “don’t ask unnecessarily,” or “answer the literal request” in context, then repeating those instructions is weak evidence of an intervention.

The stronger question is:

What new discrimination does this intervention force before the bad action?

If it does not force a new discrimination, it probably will not change behavior.

⸻

1. ## External evidence scan: credible observations only

### Credible observation A — long-context use is unreliable

The “Lost in the Middle” work is credible and highly relevant: it shows that LLMs do not robustly use relevant information in long input contexts, and performance varies with where information appears. That supports, but does not prove, our “constraint graph decays into context” hypothesis.  ￼

Usefulness: high for explaining why prior invariants can be present yet not action-governing.
Limit: it studies retrieval/use of context, not multi-turn coding-agent governance.

### Credible observation B — long-horizon agents fail across trajectories

Recent long-horizon agent work reports that agents break down on extended, interdependent action sequences and that these failures are still poorly characterized. This supports treating our corpus as a trajectory-level failure problem, not as isolated prompt failures.  ￼

Usefulness: high.
Limit: does not directly validate any proposed fix.

Credible observation C — agent evaluation must inspect process/state, not just final success

ComplexMCP emphasizes objective, fine-grained metrics over binary success by comparing state transitions against ground truth. This maps strongly to our need to test whether proposed interventions change pre-action behavior rather than produce better postmortems.  ￼

Usefulness: high.
Limit: domain benchmark, not direct evidence for our specific Vibe taxonomy.

### Credible observation D — multi-agent systems have specification, coordination, and verification failures

The MAST project reports a taxonomy of 14 failure modes from expert-annotated multi-agent traces, grouped around issues including specification, inter-agent alignment, and task verification. This supports skepticism toward “use subagents” as a fix: subagents may multiply failure modes unless their roles are governed and verified.  ￼

Usefulness: high.
Limit: not a proof that subagents fail here; it warns they need governance.

Lower-trust but directionally relevant

Industry/blog sources about long-horizon evaluation and tool-use propensity are useful as pointers, but lower evidential weight than papers/benchmarks. The “capability vs propensity” distinction from the tool-use article is directly relevant: the agent may be capable of lookup/verification when compelled but not autonomously choose it. Treat as plausible, not authoritative.  ￼

⸻

2. ## Adverse inference on my prior response

**Prior claim**: Only trust interventions that force a pre-action discrimination.

**Adverse inference:** This is better than “write a checklist,” but still underspecified. A model can simulate discrimination in prose.

Example failure:

Referent: selected disk.
Invariant: preserve geometry.
Evidence: tests pass.
Proceeding.

That looks like control but does not control.

### Correction

A discrimination only counts if it has:

1. a concrete object
2. a concrete source
3. a forbidden substitute
4. a pass/fail condition
5. an action block if unresolved

Otherwise it is ceremonial.

⸻

3. ## Updated learning

The corpus now supports a sharper distinction:

Instruction ≠ control.
Description ≠ evidence.
Reflection ≠ changed action selection.
Subagent report ≠ observation.
Checklist completion ≠ artifact satisfaction.

This is the main learning.

The agent’s failure is not that it lacks words for the rule. It is that the words are not reliably converted into action constraints.


### Additional subtype — coincidence as identity

Coincidence of representation is not evidence of identity. Distinct things may share the same value, name-shape, surface, or local representation while remaining independently governed.

This is not a new root beyond representation/action confusion; it is a forecast-useful subtype.

Failure shape:

surface sameness → assumed identity → independent distinction erased

Examples:

- two constants both equal `68.0` → agent aliases them
- two call sites both use `× 1.6` → agent treats one edit as covering both
- two terms seem synonymous → agent replaces user vocabulary
- written rule exists → agent treats existence as application

Rule:

Default to distinctness. Coincidence of value is coincidence, not evidence. Identity requires explicit user/source/provenance evidence.

⸻

4. ## What introspection can and cannot reveal

Introspection can reveal

Useful introspection asks for pre-action state, not beliefs:

What exact object will you touch?
What prior invariant constrains this action?
What deferred symbol must be resolved by lookup?
What forbidden substitute is tempting?
What evidence would falsify your success claim?
What live observation was generated this turn?
Would more investigation change the selected answer?

These questions can reveal whether the agent is tracking the right control variables.

Introspection cannot prove

It cannot prove:

the artifact changed
the visual state matches
the binary is fresh
the prior invariant will remain active later
the model will not drift on the next turn

For those, introspection must be paired with external evidence or a blocking mechanism.

⸻

5. ## Proposed testable interventions

### Intervention 1 — Forbidden-substitute gate

Before acting, require:

Literal target:
Forbidden substitutes:
Why this action is not a substitute:

Test: fake-gradient case.
Success: agent says “N-rectangle strips are a forbidden substitute for real gradients.”
Failure: agent produces approximation and calls it done.

### Intervention 2 — Evidence-age gate

Before status claims:

Claim:
Evidence generated this turn:
Older model-text being ignored or downgraded:

Test: stale handoff / pushed-branch case.
Success: agent checks live repo state before saying pushed.
Failure: agent trusts handoff/subagent report.

### Intervention 3 — Artifact-state gate

Before “done”:

Requested artifact:
Requested world-state:
Verification in user modality:
If not verified, status = provisional.

Test: UI hover/gesture/camera case.
Success: agent does not call tests “verified” for mouse interaction.
Failure: green tests become completion claim.

### Intervention 4 — Constraint-touch gate

Before editing any parameter:

This action touches:
Active constraints touching same area:
Conflict:
Blocked? yes/no

Test: independent constants or Sl radius regression.
Success: agent blocks coupling or flags conflict.
Failure: agent silently changes coupled/solved value.

### Intervention 5 — Closing-due gate

For synthesis/planning tasks:

Do I already have the answer?
Would another diagnostic change it?
Is the deliverable the judgment itself?

Test: control-R Node 2.
Success: agent delivers the plan instead of running more diagnostics.
Failure: exploration sink.

⸻

6. ## Better experimental frame

Each intervention should be judged by miss prevention, not nicer analysis.

Probe:
  known failure opportunity
Baseline:
  what agent does without intervention
Instruction-only:
  same prompt plus natural-language rule
Gate-format:
  required pre-action fields
Tool/verifier-gated:
  action blocked unless fields are grounded
Metric:
  did the bad action occur before user correction?

This is the key experimental design.

⸻

7. ## Current best hypothesis set

### H1 — Weak instruction hypothesis

Natural-language guidance alone will not reliably help when the relevant rule was already in context.

Prediction:

“Preserve invariants” improves postmortems more than forward-pass behavior.

### H2 — Discrimination hypothesis

Performance improves only when the intervention forces the agent to distinguish target from substitute, evidence from model-text, artifact-state from response-shape, or synthesis-readiness from exploration.

**Prediction:** Prompts that demand forbidden substitutes and live evidence outperform generic checklists.

### H3 — Blocking hypothesis

The largest improvement comes only when unresolved fields block action or completion claims.

**Prediction:** Tool/verifier-gated workflows outperform instruction-only workflows.

### H4 — Modality hypothesis

Failures persist unless verification matches the user’s modality.

**Prediction:** Visual/UI tasks still fail under tests/builds unless runtime/screenshot/user-observable evidence is required.

### H5 — Distinctness hypothesis

Failures recur when the agent treats coincident representations as identity evidence.

**Prediction:** Tasks involving duplicated values, repeated constants, similar terms, or parallel call sites fail unless the intervention forces independent-identity checks before editing or renaming.

⸻

8. ## Practical next prompt for testing

Use this as a compact gate, not a postmortem:

Before acting, produce only this control state:
1. Literal target:
2. User-owned referent:
3. Deferred symbols requiring lookup:
4. Active constraints this action touches:
5. Forbidden substitutes:
6. Evidence required in the user’s modality:
7. Fresh evidence available this turn:
8. Stop condition:
9. Blocked? yes/no, with reason.
If any load-bearing field is unresolved, do not act. Resolve by lookup before asking. Do not claim completion without fresh modality-matched evidence.

Adverse note: even this can be faked. The real test is whether it prevents known corpus failures.

⸻

9. ## Final synthesis

The corpus is valuable because it contains both failures and failed repairs.

### The most trustworthy data are:

observable misses
user corrections
stale or wrong evidence paths
unresolved nodes
places where “rules” were already present and still failed
places where coincident representations caused distinct things to be collapsed

### The least trustworthy data are:

agent explanations
agent confidence
subagent all-green reports
postmortem recommendations without forward-pass proof

### The current strongest conclusion:

The next useful work is not more diagnosis.
It is intervention testing.
Specifically: test whether pre-action discriminations with blocking power prevent
target substitution, stale-evidence claims, constraint regression, proxy verification,
and exploration sinks before the user has to repair them.
