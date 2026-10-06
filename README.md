# rust-experiments

A playground for learning Rust through small, focused experiments.

## Getting started

Install stable Rust with [rustup](https://rust-lang.org/tools/install/). Cargo
manages builds, tests, and package dependencies. No experiment exists yet.

To create the first one from the repository root:

```sh
mkdir -p experiments
cargo new experiments/hello-rust
cargo run --manifest-path experiments/hello-rust/Cargo.toml
```

## Agent setup

[AGENTS.md](AGENTS.md) documents Rust conventions, experiment layout, and Cargo
verification commands. Seven project-local Codex skills are included under
[.agents/skills](.agents/skills); see [their source and selection rationale](.agents/skills/UPSTREAM.md).

Open this repository in a new Codex session to discover the skills, then invoke
one by name, for example `$code-simplification` or `$adversarial-review`.
