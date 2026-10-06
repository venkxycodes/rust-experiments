use crate::model::Counter;

/// Reads the model and returns text; it cannot mutate the counter.
pub(crate) fn render(counter: &Counter) -> String {
    format!("Count: {}", counter.value())
}
