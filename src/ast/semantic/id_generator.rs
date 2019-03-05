pub struct IdGenerator {
    pub next_id: u64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn get_next_id(&mut self) -> u64 {
        self.next_id += 1;

        self.next_id - 1
    }
}
