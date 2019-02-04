use crate::ast::semantic::SymbolTable;
use crate::ast::statement::ProcedureStmt;
use std::collections::{HashMap, HashSet};

pub struct Program {
    procedures: HashSet<String>,
    symbol_table: SymbolTable,
}

impl Program {
    pub fn new() -> Self {
        Self {
            procedures: Default::default(),
            symbol_table: SymbolTable::new(),
        }
    }
}
