mod controller;
mod model;
mod view;

use std::io::{self, BufRead, Write};

use controller::Outcome;
use model::Counter;

/// Runs the session using supplied input/output, so tests need no real terminal.
pub fn run(input: impl BufRead, mut output: impl Write) -> io::Result<()> {
    let mut counter = Counter::default();
    writeln!(output, "Commands: inc | dec | reset | show | quit")?;
    writeln!(output, "{}", view::render(&counter))?;
    output.flush()?;

    for line in input.lines() {
        let line = line?;
        match controller::handle(&line, &mut counter) {
            Ok(Outcome::Continue) => writeln!(output, "{}", view::render(&counter))?,
            Ok(Outcome::Quit) => break,
            Err(message) => writeln!(output, "Error: {message}")?,
        }
        output.flush()?;
    }
    Ok(())
}
