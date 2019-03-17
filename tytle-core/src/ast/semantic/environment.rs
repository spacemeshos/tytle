use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{IdGenerator, SymbolTable, Variable};

pub struct Environment {
    pub symbol_table: SymbolTable,
    pub id_generator: IdGenerator,
    pub globals_index: u64,
    pub main_proc_id: Option<u64>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            main_proc_id: None,
            symbol_table: SymbolTable::new(),
            id_generator: IdGenerator::new(),
            globals_index: 0,
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
        };

        self.symbol_table.create_var_symbol(var);

        (id, var_name)
    }
}
