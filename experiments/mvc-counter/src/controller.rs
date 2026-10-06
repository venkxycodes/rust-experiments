use crate::model::Counter;

pub(crate) enum Outcome {
    Continue,
    Quit,
}

/// Translates user input into model operations.
pub(crate) fn handle(input: &str, counter: &mut Counter) -> Result<Outcome, &'static str> {
    match input.trim() {
        "inc" => counter.increment()?,
        "dec" => counter.decrement()?,
        "reset" => counter.reset(),
        "show" => {}
        "quit" => return Ok(Outcome::Quit),
        _ => return Err("unknown command; use inc, dec, reset, show, or quit"),
    }
    Ok(Outcome::Continue)
}
