# Agent instructions

## Purpose and current state

This repository is a playground for learning and experimenting with Rust. Keep
examples small, runnable, and focused on one concept. At setup time it contains
only documentation and agent skills; there is no Cargo manifest or Rust code yet.
Inspect the current tree before choosing commands or assuming a project layout.

## Working approach

- Read the relevant files and any more specific AGENTS.md before changing code.
- Follow the user’s requested scope. Preserve unrelated work and avoid speculative
  frameworks, services, abstractions, and dependencies.
- Explain Rust concepts involved in a change, especially ownership, borrowing,
  lifetimes, traits, and error handling, when that helps the learner.
- For a new experiment, use a descriptively named Cargo package in
  `experiments/<name>/` unless the user requests another layout. Do not create a
  root workspace until experiments need shared management.
- Document each experiment’s purpose and exact run command in its README.
- Commit or push only when the user requests it. Never force-push by default.

## Rust conventions

- Use stable Rust and Cargo. Respect an existing rust-toolchain.toml, edition,
  MSRV, Cargo.toml, and Cargo.lock; do not change them incidentally.
- Prefer simple idiomatic Rust: enums for distinct states, pattern matching,
  Option for absence, and Result with `?` for recoverable failures.
- Prefer borrowing when practical. Introduce cloning, shared ownership, explicit
  lifetimes, async runtimes, or trait abstractions only when the example needs them.
- Keep unsafe out of ordinary experiments. An experiment explicitly about unsafe
  must state and justify each safety invariant at the relevant block or function.
- Avoid unchecked unwrap/expect in reusable code. Small teaching examples and
  tests may use them when the assumption is clear and failure is intentional.
- Start with the standard library. Add a crate only for a concrete need, explain
  why it is needed, and use Cargo to update the manifest and lockfile.
- Commit Cargo.lock for runnable experiments. Do not commit target directories,
  credentials, or local environment files.
- Preserve intentional println! output in CLI demonstrations; it is observable
  behavior, not automatically debug debris.

## Verification

Documentation and skill-only changes: run `git diff --check` and verify local
paths, skill references, and the final diff. Do not invent Cargo results when
there is no Cargo.toml.

For a changed Cargo package, run from that package directory:

```sh
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
```

Run a runnable experiment with `cargo run` or its documented example command
when relevant. Add focused tests for meaningful behavior, edge cases, and error
paths. Avoid tests that merely repeat implementation details.

If a root Cargo workspace is added later, run these from its root:

```sh
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Use feature combinations relevant to the change; enable all features only if
those features are compatible. Report what passed and any checks that could not
run, with the reason.

## Project-local skills

Skills live in `.agents/skills/`. Read the selected SKILL.md and its relevant
references before using it. These upstream skills are language-independent;
translate their examples into idiomatic Rust rather than copying other languages’
patterns. Use only the skill appropriate to the task, not every skill on every edit.

| Skill | Use |
| --- | --- |
| `codebase-design` | Place module responsibilities and keep interfaces small. |
| `design-an-interface` | Compare alternative Rust APIs when design exploration is requested. |
| `code-simplification` | Simplify the touched code without changing behavior, when requested. |
| `deslop` | Inspect or remove generated debris in the requested scope; preserve demo output. |
| `architecture-review` | Review module architecture when explicitly requested; read-only by default. |
| `adversarial-review` | Challenge a change from independent review lenses; read-only. |
| `summarize-diff` | Explain changes using the actual Git diff. |

Invoke a skill by name, such as `$code-simplification` or `$adversarial-review`.
Interface design and adversarial review use subagents; if delegation is unavailable,
state that limitation and do not claim independent reviewers ran.

See `.agents/skills/UPSTREAM.md` for source, revision, selection rationale, and license.
