pub struct MoveCounter {
    fifty_move_counter: Vec<u8>,
    halfmove: u128,
    fullmove: u128,
}

impl Default for MoveCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveCounter {
    pub fn new() -> Self {
        Self {
            fifty_move_counter: vec![0],
            halfmove: 0,
            fullmove: 1,
        }
    }
    pub fn increment(&mut self, reset_fifty_move_rule: bool) {
        if reset_fifty_move_rule {
            self.reset_fifty_move_rule();
        } else {
            if let Some(count) = self.fifty_move_counter.last_mut() {
                *count = count.saturating_add(1);
            }
        }
        self.halfmove = self.halfmove.saturating_add(1);
        if self.halfmove % 2 == 0 {
            self.fullmove = self.fullmove.saturating_add(1);
        }
    }

    fn reset_fifty_move_rule(&mut self) {
        self.fifty_move_counter.push(0);
    }
    pub fn decrement(&mut self) {
        if let Some(count) = self.fifty_move_counter.last_mut() {
            if *count > 0 {
                *count -= 1;
            } else if self.fifty_move_counter.len() > 1 {
                self.fifty_move_counter.pop();
            }
        }
        self.halfmove = self.halfmove.saturating_sub(1);
        if self.halfmove % 2 == 0 && self.fullmove > 0 {
            self.fullmove = self.fullmove.saturating_sub(1);
        }
    }
    pub fn get_fifty_move_rule_count(&self) -> u8 {
        if let Some(count) = self.fifty_move_counter.last() {
            *count
        } else {
            0
        }
    }
    pub fn get_halfmove_count(&self) -> u128 {
        self.halfmove
    }
    pub fn get_fullmove_count(&self) -> u128 {
        self.fullmove
    }
}

#[cfg(test)]
mod tests {
    use super::MoveCounter;

    #[test]
    fn increment_a_lot() {
        let mut counter = MoveCounter::default();
        for _ in 0..1_000_000 {
            counter.increment(false);
        }
        assert_eq!(255u8, counter.get_fifty_move_rule_count());
    }
    #[test]
    fn reset_undo() {
        let mut counter = MoveCounter::default();
        for _ in 0..10 {
            counter.reset_fifty_move_rule();
            for _ in 0..60 {
                counter.increment(false);
            }
        }
        for _ in 0..1_000_000 {
            counter.decrement();
        }
        assert_eq!(counter.fifty_move_counter.len(), 1);
    }
}
