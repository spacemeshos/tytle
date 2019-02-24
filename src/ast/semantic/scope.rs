use crate::ast::semantic::{Symbol, SymbolKind, Variable};
use std::collections::HashMap;

pub type ScopeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    symbols: HashMap<SymbolKind, HashMap<String, Symbol>>,
}

impl Scope {
    pub fn new(id: ScopeId, parent_id: Option<ScopeId>) -> Self {
        let mut symbols = HashMap::new();

        symbols.insert(SymbolKind::Var, HashMap::new());
        symbols.insert(SymbolKind::Proc, HashMap::new());

        Self {
            id,
            parent_id,
            symbols: symbols,
        }
    }

    pub fn store(&mut self, symbol: Symbol, kind: &SymbolKind) {
        let table = self.get_kind_table_mut(kind);

        table.insert(symbol.name().clone(), symbol);
    }

    pub fn lookup(&self, sym_name: &str, kind: &SymbolKind) -> Option<&Symbol> {
        let table = self.get_kind_table(kind);

        table.get(sym_name)
    }

    pub fn lookup_mut(&mut self, sym_name: &str, kind: &SymbolKind) -> Option<&mut Symbol> {
        let table = self.get_kind_table_mut(kind);

        table.get_mut(sym_name)
    }

    pub fn is_root_scope(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn is_inner_scope(&self) -> bool {
        !(self.is_root_scope())
    }

    fn get_kind_table(&self, kind: &SymbolKind) -> &HashMap<String, Symbol> {
        self.symbols.get(kind).unwrap()
    }

    fn get_kind_table_mut(&mut self, kind: &SymbolKind) -> &mut HashMap<String, Symbol> {
        self.symbols.get_mut(kind).unwrap()
    }
}
