# Rust Zero-to-Hero: Backend Track

A hands-on Rust course built for Python developers targeting backend/web services work.

## How This Repo Works

Each module has:
- **`README.md`** — concept explanations with Python comparisons, read this first
- **`src/main.rs`** — worked examples you can run and tinker with
- **`src/exercises.rs`** — stubs with `todo!()` for you to implement

### Running examples

```bash
cargo run -p module_01_basics
```

### Checking your exercises

Fill in the `todo!()` stubs in `src/exercises.rs`, then run the tests:

```bash
cargo test -p module_01_basics
```

All tests pass = you got it right. You can also run tests for all modules at once:

```bash
cargo test
```

### Asking me to check your work

If you want a code review or hint, just say "check my exercises for module X" and I'll look at what you wrote and give feedback.

---

## Modules

| # | Module | Core Concepts | Estimated Time |
|---|--------|--------------|----------------|
| 01 | [Basics](./module_01_basics/README.md) | Variables, types, functions, control flow | 1 week |
| 02 | [Ownership](./module_02_ownership/README.md) | Ownership, borrowing, references, slices | 1–1.5 weeks |
| 03 | [Structs & Enums](./module_03_structs_enums/README.md) | Structs, enums, pattern matching, Option | 1 week |
| 04 | [Error Handling](./module_04_error_handling/README.md) | Result, ?, thiserror, error propagation | 1 week |
| 05 | [Traits & Iterators](./module_05_traits_iterators/README.md) | Traits, generics, iterators, closures | 1 week |
| 06 | [Async & Axum](./module_06_async_axum/README.md) | async/await, tokio, Axum HTTP handlers, serde | 1–2 weeks |

---

## Prerequisites

Install Rust via rustup (if you haven't):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
rustc --version
cargo --version
```

Recommended editor: VS Code with the **rust-analyzer** extension.

---

## Tips

- **Don't skip module 02.** Ownership is the hardest concept and everything else builds on it.
- When the compiler gives you an error, read the full message — Rust's errors are unusually helpful.
- The [Rust Book](https://doc.rust-lang.org/book/) is free online and is the best reference. Use it alongside these exercises.
- `cargo check` is faster than `cargo build` — use it to get compiler feedback while writing.
