# rust-experiments

A playground for learning Rust through small, focused experiments.

## Experiments

- [MVC counter](experiments/mvc-counter/README.md): a tiny model/view/controller
  example with terminal input, borrowed state, error handling, and tests.

```sh
cargo run --manifest-path experiments/mvc-counter/Cargo.toml
```

Install stable Rust with [rustup](https://rust-lang.org/tools/install/). Cargo
manages builds, tests, and package dependencies. Each experiment is its own Cargo
package; run commands with its manifest path or from its directory.

## Agent setup

[AGENTS.md](AGENTS.md) documents Rust conventions, experiment layout, and Cargo
verification commands. Seven project-local Codex skills are included under
[.agents/skills](.agents/skills); see [their source and selection rationale](.agents/skills/UPSTREAM.md).

Open this repository in a new Codex session to discover the skills, then invoke
one by name, for example `$code-simplification` or `$adversarial-review`.
