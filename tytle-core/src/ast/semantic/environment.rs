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

    pub fn new_tmp_var(&mut self, proc_id: SymbolId, var_type: ExpressionType) -> (u64, String) {
        let id = self.id_generator.get_next_id();
        let var_name = format!("$TMP{}", id);

        let proc_locals = self.locals_symbols.entry(proc_id).or_insert(Vec::new());

        let proc_locals_count = proc_locals.len();

        let var = Variable {
            id,
            name: var_name.clone(),
            var_type: Some(var_type),
            global: false,
            index: Some(proc_locals_count),
        };

        self.symbol_table.create_var_symbol(var);

        let proc_locals = self.locals_symbols.entry(proc_id).or_insert(Vec::new());
        proc_locals.push(id);

        (id, var_name)
    }
}
