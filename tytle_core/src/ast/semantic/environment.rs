use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{IdGenerator, Procedure, SymbolId, SymbolTable, Variable};
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

    pub fn create_proc(
        &mut self,
        name: &str,
        params_types: Vec<ExpressionType>,
        return_type: ExpressionType,
    ) -> SymbolId {
        let id = self.id_generator.get_next_id();

        let proc = Procedure {
            id,
            name: name.to_string(),
            params_types,
            return_type,
        };

        self.symbol_table.create_proc_symbol(proc);

        id
    }

    pub fn create_tmp_var(
        &mut self,
        proc_id: SymbolId,
        var_type: ExpressionType,
    ) -> (SymbolId, String) {
        let var_id = self.id_generator.get_next_id();
        let var_name = format!("$TMP{}", var_id.0);

        self.create_local_var(proc_id, var_id, var_name.as_str(), Some(var_type), false);

        (var_id, var_name)
    }

    pub fn create_global_var(
        &mut self,
        var_id: SymbolId,
        var_name: &str,
        var_type: Option<ExpressionType>,
    ) {
        let global_id = self.globals_index;

        let var = Variable {
            id: var_id,
            global: true,
            param: false,
            name: var_name.to_string(),
            var_type,
            index: Some(global_id),
        };

        self.symbol_table.create_var_symbol(var);

        self.globals_index += 1;
        self.globals_symbols.insert(global_id, var_id);
    }

    pub fn create_local_var(
        &mut self,
        proc_id: SymbolId,
        var_id: SymbolId,
        var_name: &str,
        var_type: Option<ExpressionType>,
        is_param: bool,
    ) {
        let var_index = self.proc_locals_count(proc_id);

        let var = Variable {
            id: var_id,
            name: var_name.to_string(),
            var_type,
            global: false,
            param: is_param,
            index: Some(var_index),
        };

        self.symbol_table.create_var_symbol(var);

        let proc_locals = self.locals_symbols.entry(proc_id).or_insert(Vec::new());
        proc_locals.push(var_id);
    }

    fn proc_locals_count(&self, proc_id: SymbolId) -> usize {
        let entry = self.locals_symbols.get(&proc_id);

        if entry.is_none() {
            return 0;
        } else {
            entry.unwrap().len()
        }
    }
}
