pub struct Counter {
    value: u8,
}

impl Counter {
    pub fn new() -> Counter {
        Self { value: 0 }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn count_down(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
