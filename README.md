# rusty

My journey learning Rust by working through [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) ("the Book").

**Currently at:** [Chapter 4 — Understanding Ownership](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

**Goal:** build up to a multithreaded web server (the Book's final project).

## Projects

| Directory | What it covers |
| --- | --- |
| `hello_world` | First programs and shadowing |
| `hello_cargo` | Getting started with Cargo |
| `variables` | Variables and mutability |
| `functions` | Functions |
| `branches` | Control flow |
| `guessing_gamer` | The guessing game |
| `ownership` | Ownership, references, and borrowing |

## Prerequisites

Install the Rust toolchain (`rustc` + `cargo`) via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

That's the only setup needed. Each project's dependencies (listed in its `Cargo.toml`) are downloaded and built automatically by Cargo — no manual install step.

## Running

Each Cargo project runs with:

```bash
cd <project>
cargo run
```
