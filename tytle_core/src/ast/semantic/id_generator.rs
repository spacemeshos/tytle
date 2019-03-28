use crate::ast::semantic::SymbolId;

pub struct IdGenerator {
    pub next_id: usize,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn get_next_id(&mut self) -> SymbolId {
        self.next_id += 1;

        SymbolId(self.next_id - 1)
    }
}
