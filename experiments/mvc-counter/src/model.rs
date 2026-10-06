/// Owns the state and enforces the counter's rules.
#[derive(Default)]
pub(crate) struct Counter {
    value: u32,
}

impl Counter {
    pub(crate) fn value(&self) -> u32 {
        self.value
    }

    pub(crate) fn increment(&mut self) -> Result<(), &'static str> {
        self.value = self
            .value
            .checked_add(1)
            .ok_or("counter is at its maximum")?;
        Ok(())
    }

    pub(crate) fn decrement(&mut self) -> Result<(), &'static str> {
        self.value = self
            .value
            .checked_sub(1)
            .ok_or("counter cannot go below zero")?;
        Ok(())
    }

    pub(crate) fn reset(&mut self) {
        self.value = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn overflow_preserves_state() {
        let mut counter = Counter { value: u32::MAX };
        assert!(counter.increment().is_err());
        assert_eq!(counter.value(), u32::MAX);
    }

    #[test]
    fn decrement_at_zero_preserves_state() {
        let mut counter = Counter::default();
        assert!(counter.decrement().is_err());
        assert_eq!(counter.value(), 0);
    }
}
