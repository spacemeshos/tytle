use crate::ast::semantic::SymbolTable;

pub struct Program {
    symbol_table: SymbolTable,
}

impl Program {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }
}
