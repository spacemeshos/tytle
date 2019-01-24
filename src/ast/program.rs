use crate::ast::procedure::Procedure;
use crate::ast::scope::Scope;
use crate::ast::statement::Statement;
use crate::ast::variable::Variable;
use std::collections::HashMap;

pub struct Program {
    globals: Vec<Variable>,
    procs: HashMap<String, Procedure>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            globals: Default::default(),
            procs: Default::default(),
        }
    }
}

impl Program {
    pub fn add_global(&mut self, var: Variable) {
        self.globals.push(var);
    }

    pub fn add_procedure(&mut self, proc: Procedure) {
        self.procs.insert(proc.name.clone(), proc);
    }

    pub fn get_procedure_by_name(&self, name: &str) -> Option<&Procedure> {
        None
    }

    pub fn get_global_by_name(&self, name: &str) -> Option<&Variable> {
        None
    }
}
