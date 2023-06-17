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
            *count += 1;
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
