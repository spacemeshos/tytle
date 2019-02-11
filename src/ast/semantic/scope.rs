use crate::ast::semantic::{Symbol, Variable};
use std::collections::HashMap;

pub type ScopeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new(id: ScopeId, parent_id: Option<ScopeId>) -> Self {
        Self {
            id,
            parent_id,
            symbols: Default::default(),
        }
    }

    pub fn store(&mut self, sym: Symbol) {
        self.symbols.insert(sym.name().clone(), sym);
    }

    pub fn lookup_symbol(&self, sym_name: &str) -> Option<&Symbol> {
        self.symbols.get(sym_name)
    }
}
