use crate::ast::expression::ExpressionType;
use crate::ast::semantic::{Environment, SymbolId};
use crate::vm::{Address, MemoryValue, Pen, Turtle};

use std::collections::HashMap;

pub struct Memory {
    pub turtle: Turtle,
    pub pen: Pen,
    pub cells: HashMap<Address, MemoryValue>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            turtle: Turtle::new(),
            pen: Pen::new(),
        }
    }

    pub fn init_globals(&mut self, env: &Environment) {
        (0..env.globals_index).into_iter().for_each(|i| {
            let var_id = env.globals_symbols[&i];
            let var = env.symbol_table.get_var_by_id(var_id);

            let addr = Address(var.index.unwrap());
            let var_type = var.var_type.as_ref().unwrap();

            let value = match var_type {
                ExpressionType::Int => MemoryValue::Int(0),
                ExpressionType::Bool => MemoryValue::Bool(false),
                ExpressionType::Str => MemoryValue::Str("".to_string()),
                ExpressionType::Unit => panic!("variable can't be of type `Unit`"),
            };

            self.set_global(addr, value);
        });
    }

    pub fn get_global(&self, address: Address) -> Option<&MemoryValue> {
        self.cells.get(&address)
    }

    pub fn set_global(&mut self, address: Address, value: MemoryValue) {
        self.cells.insert(address, value);
    }
}
