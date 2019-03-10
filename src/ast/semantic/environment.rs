use crate::ast::semantic::{IdGenerator, SymbolTable};

pub struct Environment {
    pub symbol_table: SymbolTable,
    pub id_generator: IdGenerator,
    pub globals_index: u64,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            id_generator: IdGenerator::new(),
            globals_index: 0,
        }
    }
}
