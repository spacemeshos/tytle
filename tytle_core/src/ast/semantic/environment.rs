use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{IdGenerator, SymbolId, SymbolTable, Variable};
use std::collections::HashMap;

pub struct Environment {
    pub symbol_table: SymbolTable,
    pub id_generator: IdGenerator,
    pub globals_index: usize,

    // used for allocating globals
    pub globals_symbols: HashMap<usize, SymbolId>,

    // used for allocating procs-locals
    pub locals_symbols: HashMap<SymbolId, Vec<SymbolId>>,

    pub main_proc_id: Option<SymbolId>,
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

    pub fn new_tmp_var(
        &mut self,
        proc_id: SymbolId,
        var_type: ExpressionType,
    ) -> (SymbolId, String) {
        let id: SymbolId = self.id_generator.get_next_id();
        let var_name = format!("$TMP{}", id.0);

        let var_index = self.proc_locals_count(proc_id);

        let var = Variable {
            id,
            name: var_name.clone(),
            var_type: Some(var_type),
            global: false,
            param: false,
            index: Some(var_index),
        };

        self.symbol_table.create_var_symbol(var);

        let proc_locals = self.locals_symbols.entry(proc_id).or_insert(Vec::new());
        proc_locals.push(id);

        (id, var_name)
    }

    pub fn proc_locals_count(&self, proc_id: SymbolId) -> usize {
        let entry = self.locals_symbols.get(&proc_id);

        if entry.is_none() {
            return 0;
        } else {
            entry.unwrap().len()
        }
    }
}
