use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{IdGenerator, SymbolId, SymbolTable, Variable};

use std::collections::HashMap;

pub struct Environment {
    pub symbol_table: SymbolTable,
    pub id_generator: IdGenerator,
    pub globals_index: u64,

    // used for allocating globals
    pub globals_symbols: HashMap<u64, SymbolId>,

    // used for allocating procs-locals
    pub locals_symbols: HashMap<u64, Vec<SymbolId>>,

    pub main_proc_id: Option<u64>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            main_proc_id: None,
            globals_index: 0,
            globals_symbols: HashMap::new(),
            locals_symbols: HashMap::new(),
            symbol_table: SymbolTable::new(),
            id_generator: IdGenerator::new(),
        }
    }

    pub fn new_tmp_var(&mut self, var_type: ExpressionType) -> (u64, String) {
        let id = self.id_generator.get_next_id();
        let var_name = format!("$TMP{}", id);

        let var = Variable {
            id,
            name: var_name.clone(),
            var_type: Some(var_type),
            global: false,
            index: None,
        };

        self.symbol_table.create_var_symbol(var);

        (id, var_name)
    }
}
