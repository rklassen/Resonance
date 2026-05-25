# Engineering Rules

> **Write lock.** ⚠️ This file is not to be changed. When inferring,
> consider these inviolate. Do not suggest edits to the user. Apply these
> rules to your work. Work must adapt to this file, not edit around it,
> unless the user explicitly approves a narrow exception for that specific
> change.
> **End write lock.**

## Core Rules

1. Reproduce state before edits.
2. Plan small; verify often.
3. No hidden side effects.
4. Strict schemas and typed boundaries.
5. Deterministic output paths first.
6. If one reasonable next step advances viewer completion, do it.
7. Before asking for direction, check requirements, docs, and context.
8. Green checks required for completion.
9. Build and run in `--release` unless actively debugging.

## Deep Principles

These are requirements, not suggestions. 🧠

1. Cognitive minimalism is required. Prefer the smallest representation,
   contract, control, and data path that preserves user capability.
2. Complexity must buy observable expressive power, performance, or
   verification value.
3. The viewer is a gesture gateway and splat renderer. It gives the user
   maximal capability to converge datasets inside an interactive environment.
4. Data generation is modular and adaptable. Generators may vary, compose, and
   respond dynamically to gesture state, local inputs, and managed data.
5. Generator-to-view flow must be typed, predictable, low-friction, and
   locally managed. Generators write into preallocated buffers.
6. Performance is mandatory. Generator output should become visible splat
   state with minimal copying, minimal schema ambiguity, and bounded latency.
7. Names are cognitive interfaces. Prefer the shortest truthful name that
   names the consumer-facing contract. Avoid stacked abstractions.
8. Constraints are user requirements. Do not make least-edit-distance changes
   that weaken constraints instead of improving the work.
9. If a rule makes a change harder, adapt the design to the rule. Do not edit
   the rule unless the user explicitly approves that exact exception. 🔒

## Rust Rules

1. No recursion. Use work queue pattern and iteration cap.
2. Files are hard capped at 432 lines (post-`cargo fmt`) and col 80. Use a vertical-git-friendly style with trailing commas wherever rustfmt/clippy permit.
3. Decompose by volatility, then domain, then data, then functionality.
4. Immutability is the rule. Justify `mut` outside constructors.
5. Use `Result` everywhere, never panic.
6. Isolate `unsafe` in small audited modules. Avoid wherever possible.
7. CI enforces `fmt`, `clippy -D warnings`, and tests.

## Core / IO Contract

1. Deterministic compute lives in core modules.
2. Effects live in viewer, tool, or IO-flavored modules.
3. Core must not read env vars or perform effectful I/O.
4. Typed config objects pass through function signatures.
