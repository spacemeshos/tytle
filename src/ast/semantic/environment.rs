use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{IdGenerator, SymbolTable, Variable};

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

    pub fn new_tmp_var(&mut self, var_type: ExpressionType) -> u64 {
        let id = self.id_generator.get_next_id();
        let name = format!("$TMP{}", id);

        let var = Variable {
            id,
            name,
            var_type: Some(var_type),
            global: false,
        };

        self.symbol_table.create_var_symbol(var);

        id
    }
}
