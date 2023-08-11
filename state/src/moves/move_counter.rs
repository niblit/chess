pub struct MoveCounter {
    counter: Vec<u8>,
}

impl Default for MoveCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveCounter {
    pub fn new() -> Self {
        Self { counter: vec![0] }
    }
    pub fn reset(&mut self) {
        self.counter.push(0);
    }
    pub fn undo(&mut self) {
        if let Some(count) = self.counter.last_mut() {
            if *count > 0 {
                *count -= 1;
            } else if self.counter.len() > 1 {
                self.counter.pop();
            }
        }
    }
    pub fn increment(&mut self) {
        if let Some(count) = self.counter.last_mut() {
            *count = count.saturating_add(1);
        }
    }
    pub fn get_count(&self) -> u8 {
        if let Some(count) = self.counter.last() {
            *count
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MoveCounter;

    #[test]
    fn increment_a_lot() {
        let mut counter = MoveCounter::default();
        for _ in 0..1_000_000 {
            counter.increment();
        }
        assert_eq!(255u8, counter.get_count());
    }
    #[test]
    fn reset_undo() {
        let mut counter = MoveCounter::default();
        for _ in 0..10 {
            counter.reset();
            for _ in 0..60 {
                counter.increment();
            }
        }
        for _ in 0..1_000_000 {
            counter.undo();
        }
        assert_eq!(counter.counter.len(), 1);
    }
}
