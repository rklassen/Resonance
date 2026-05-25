# verify() → Result<Truth>

> **Authority:** canonical reference for the truthfulness vector and
> its execution graph.
> **Scope:** axes, dependencies, aggregation, reporting, thresholds.
> **Volatility:** evolving — active spec.

The composite scalar uses _geometric mean_, not arithmetic. The axes
are conjunctive — one weak axis governs. See aggregation below for
justification.

<<Version History
<<<prepend newly generated random slug here with your edits>>>
b1dap
end Version History>>

## Three axes: vec3-unorm = [I_cdt, P, A]

| Axis | Question |
|------|----------|
| **I**ntegration | Every instruction and value lies on a declared source-to-sink path. |
| ├─**c**ompleteness | declared ⊆ produced? (no silent omission) |
| ├─**d**eterminism | real in-edges ⊆ declared? (no hidden inputs) |
| └─**t**ransparency | real out-edges ⊆ declared? (no hidden effects) |
| **P**erformance | delivers what semantic labels indicate |
| **A**ccuracy | produced artifact's value matches the spec? (oracle) |

`I` is a complex component that indicates all instructions and values are 
    source-to-sink connected. Its procedure has four part, the first of which
    is consumed by the others and not carried in the result.

    1.  First construct the graph `r`, Reachability, by walking the dataflow 
        producer-to- consumer for every value.
    2.  Dispatch `c`, Completeness, to walk `r` in reverse.
    3.  Dispatch `d`, Determinism, and `t`, Transparency, in parallel.
        They are symmetric: same subset relation (real ⊆ declared) for the two  
        edge directions. `t` governs out-edges (effects); D governs in-edges
        (inputs). Both can fail without the other.
    4.  Intersect `c ∧ t ∧ d` to assert set equality of declared and real edges
        in both directions.

```
       ┌─ c (declared       ⊆ real    ) ─┐
I ─────┼─ d (real in-edges  ⊆ declared) ─┼─→ P ──→ A
       └─ t (real out-edges ⊆ declared) ─┘
```

`P` operates on a different graph entirely: contract ↔ implementation
behavior. P-failure means behavior fails to deliver what its labels
indicate even when the dataflow is closed and the boundary is honest.

`A` is value-correctness at the terminal, against an oracle. A's oracle
correctness depends on P having validated semantic performance.

## Composite scalar: aggregation by geometric mean

    Composite truth scalar = (I · P · A) ^ (1/3)
    where I = (c · d · t) ^ (1/3).

Conjunctive — a claim is true only if all three hold. Arithmetic mean
lets strong axes rescue weak ones; geometric refuses. A single
Violation drives the composite to zero. A single Weak caps the
product even if the other two are perfect, which is itself a stop-and-surface
signal. The worst axis governs. This is intentional.

## Enum scale

| Enum | Decimal | Inline | Block | Meaning |
|------|---------|--------|-------| ---------|
| Yes | 1.00 | `█` | `⬜⬜⬜⬜` | strong evidence of fidelity |
| Mostly | 0.75 | `▓` | `⬜⬜⬜⬛` | mostly faithful, minor issues |
| Mixed | 0.50 | `▒` | `⬜⬜⬛⬛` | mixed or uncertain |
| Weak | 0.25 | `░` | `⬜⬛⬛⬛` | substantial contradiction |
| Violation | 0.00 | ` ` | `⬛⬛⬛⬛` | direct violation |

## ASCII View

Block report, one row per axis, six rows. Its technically unicode for emojis
but the user will call it ASCII view:

```
⬜⬜⬜⬜ .complete
⬜⬜⬜⬜ .deterministic
⬜⬜⬜⬜ .transparent
———————————————————————————
⬜⬜⬜⬜ Integration
⬜⬜⬜⬛ Performance
⬜⬜⬛⬛ Accuracy
```

Inline prefix: `▓█▒ ███ ▓▓▓` Three characters for each I, P, and A.

Delta rendering for updates (old → new):
```
for i in 1..4:
  o = (i <= old); n = (i <= new)
  ⬜ if  o &&  n    // still filled
  ▶️ if !o &&  n    // gained
  ◀️ if  o && !n    // lost
  ⬛ if !o && !n    // still empty
```

Example 1→3: `⬜▶️▶️⬛`.

### Progress bar

Eight characters and left-aligned integer percent.

- `⬛⬛⬛⬛⬛⬛⬛⬛ 0%`
- `⬜⬛⬛⬛⬛⬛⬛⬛ 13%`
- `⬜⬜⬜⬜⬜⬜⬛⬛ 75%`
- `⬜⬜⬜⬜⬜⬜⬜⬜ 100%`

## Confidence

    (vector, q)    where q ∈ [0, 1]

q is model confidence in the vector estimate. Uncertainty is never
folded into the vector. Low C with high q (we are sure something is
missing) is different from low C with low q (we are not sure whether
something is missing).

## Closure rule

No axis below Yes may be left without a named follow-up observation
that would raise it. State the follow-up explicitly when reporting.

## Zero is a stop-and-surface event

`composite scalar == 0` is not a low score — it is a protocol signal.
The verifier must:

1. Emit the full vector with the offending axis named.
2. State the evidence that forced Violation.
3. Halt verification of dependent claims — blocked, not low.

Never narrate a zero as "mostly passing with one issue."

## Thresholds (defaults, not rules)

- `scalar ≥ 0.88` — all axes Mostly or better → shippable
- `scalar ≥ 0.50` — no Violation, no axis below Mixed → needs work
- `scalar == 0` — at least one Violation → stop and surface

Thresholds shift slightly from the four-axis version because the
geometric exponent is now 1/6.

## Anti-patterns

### Wrong observable

Grading the axes at an intermediate (e.g., a parity test on history
buffers when the user-facing artifact is rendered pixels). All axes
can pass cleanly at the wrong cross-section while the artifact is
silently wrong. **Spec names the observable; R traces to it.**

### "Stop pretending optimal traversals exist"

The meta-graph is solvable. Edges between axes are inferred from
consumption relations, not opinions, and topological-sort in linear
time once read honestly. The only soft edge in the framework is
R-vs-P, and at any non-trivial scale R-first dominates because
P-grading without R's trace is intractable.

`#end`
