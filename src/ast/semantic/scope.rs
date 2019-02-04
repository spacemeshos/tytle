use crate::ast::semantic::Variable;
use std::collections::HashMap;

type ScopeId = u64;

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    variables: HashMap<String, Variable>,
}

impl Scope {
    pub fn new(id: ScopeId, parent_id: Option<ScopeId>) -> Self {
        Self {
            id,
            parent_id,
            variables: Default::default(),
        }
    }

    pub fn store(&mut self, var: Variable) {
        self.variables.insert(var.name.clone(), var);
    }

    pub fn lookup_var(&self, var_name: &str) -> Option<&Variable> {
        self.variables.get(var_name)
    }
}
