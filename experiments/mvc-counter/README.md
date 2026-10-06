# MVC counter: a small Rust experiment

An interactive counter using only the Rust standard library. Type a command and
press Enter. State lives in memory and resets when the program starts.

## Run

From the repository root:

```sh
cargo run --manifest-path experiments/mvc-counter/Cargo.toml
```

Try `inc`, `inc`, `dec`, `show`, `reset`, then `quit`. You will see:

```text
Commands: inc | dec | reset | show | quit
Count: 0
Count: 1
Count: 2
Count: 1
Count: 1
Count: 0
```

`dec` at zero reports an error and keeps the session running. Unknown commands
also report errors. `quit` or end-of-input ends the session. On macOS, Ctrl-D at
an empty line sends end-of-input.

## Read the code in this order

| File | Responsibility |
| --- | --- |
| `src/model.rs` | `Counter` owns state; increment/decrement enforce its limits. |
| `src/view.rs` | Formats the counter as text using a read-only borrow. |
| `src/controller.rs` | Matches a command and calls the appropriate model operation. |
| `src/lib.rs` | Wires the modules together and runs the input/output loop. |
| `src/main.rs` | Connects the library to the real terminal. |
| `tests/session.rs` | Exercises a complete session without a real terminal. |

```text
terminal input -> controller -> model
                                 |
                                 v
terminal output <- run loop <- view
```

This is a teaching adaptation of MVC, with the run loop coordinating updates and
rendering. Rust does not require a particular architecture. Modules organize
code; structs hold data; `impl` blocks define methods. There is no need for a
base controller class, framework, database, or async runtime for this example.

## Rust technicalities you can see here

- **Ownership:** the `run` function owns one `Counter`. Its private `value` field
  keeps callers from bypassing the model's rules.
- **Borrowing:** `&Counter` lets the view read state; `&mut Counter` lets the
  controller change it. The same value stays in the run loop; it is not cloned.
- **Methods:** `&self` reads a value and `&mut self` can change it. `impl Counter`
  groups these methods with the struct.
- **Enums and match:** `Outcome` has `Continue` and `Quit` variants; `match`
  handles those outcomes and command strings explicitly.
- **Option and Result:** `checked_add`/`checked_sub` return `Option` on numeric
  overflow/underflow. `ok_or` converts missing values into `Result` errors.
  The failed assignment leaves the original counter unchanged.
- **The `?` operator:** returns an error early. Model errors are caught by the
  run loop and displayed; terminal I/O errors propagate to `main`.
- **Strings:** `&str` borrows command text; `String` owns the view's formatted
  text. `&'static str` errors are fixed string literals stored for the program's
  lifetime. No custom lifetime annotations are needed for the borrowed counter.
- **Visibility:** `mod` declares modules. `pub(crate)` exposes items within this
  library crate; `pub fn run` exposes the entry point to the binary and tests.
- **Traits:** `impl BufRead` and `impl Write` accept any input/output types with
  those capabilities. Tests supply an in-memory cursor and byte vector; `main`
  supplies stdin and stdout. You do not need to implement your own traits yet.
- **Package vs crate:** Cargo.toml describes one package. `lib.rs` and `main.rs`
  are separate library and binary crates. The package name `mvc-counter` becomes
  `mvc_counter` when referred to in Rust code.

## Experiments to try

1. Change `view::render` to print `[counter = 3]`. The model should not change.
2. Add a `double` command: implement checked multiplication in the model, then
   dispatch it in the controller. Add a test for the new command and its limit.
3. Add a `help` command that displays usage without changing the counter.
4. Try mutating the counter inside the view through `&Counter`; read the
   compiler error, then undo it. This demonstrates a read-only borrow.
5. Later, add file persistence and compare how `Result` handles file failures.

## Verify

```sh
cd experiments/mvc-counter
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
```

Tests cover command flow, reset, quit, invalid input, zero/maximum limits,
end-of-input, and a failed output stream.

## GitHub references

Consulted on 2026-10-06; the implementation here is original.

- [rust-lang/book: separating concerns and error handling](https://github.com/rust-lang/book/blob/main/src/ch12-03-improving-error-handling-and-modularity.md)
  informed the small `main.rs` and testable library entry point.
- [rust-lang/rustlings](https://github.com/rust-lang/rustlings)
  provides focused Rust exercises. Use its ownership, structs, enums, modules,
  error-handling, and tests exercises alongside this experiment.

These are learning references; neither is an MVC framework or a dependency of
this package.
