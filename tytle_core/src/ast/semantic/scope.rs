use crate::ast::semantic::{SymbolId, SymbolKind};
use std::collections::HashMap;

pub type ScopeId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,

    // `symbols` is a dedicated `HashMap` per symbol-kind
    // for example all variables are organized under their own `HashMap`
    //
    // each such `HashMap` key is a String, standing for the symbol name.
    // for example: variable name / procedure name
    // the value are a symbol-id (global integer)
    //
    // once we've a symbol-id, the symbol can be retrieved from the `SymbolTable` `lookup_by_symbol_id` method
    symbols: HashMap<SymbolKind, HashMap<String, SymbolId>>,
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

    pub fn store(&mut self, symbol_name: String, symbol_id: SymbolId, kind: &SymbolKind) {
        let table = self.get_kind_table_mut(kind);

        table.insert(symbol_name, symbol_id);
    }

    pub fn lookup(&self, sym_name: &str, kind: &SymbolKind) -> Option<&SymbolId> {
        let table = self.get_kind_table(kind);

        table.get(sym_name)
    }

    pub fn is_root_scope(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn is_inner_scope(&self) -> bool {
        !(self.is_root_scope())
    }

    fn get_kind_table(&self, kind: &SymbolKind) -> &HashMap<String, SymbolId> {
        self.symbols.get(kind).unwrap()
    }

    fn get_kind_table_mut(&mut self, kind: &SymbolKind) -> &mut HashMap<String, SymbolId> {
        self.symbols.get_mut(kind).unwrap()
    }
}
