use crate::ast::semantic::{ScopeId, Symbol, SymbolKind, SymbolTable};

pub struct SymbolTableVisitor<'a> {
    sym_table: &'a SymbolTable,
    scope_id: ScopeId,
}

impl<'a> SymbolTableVisitor<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self {
            sym_table,
            scope_id: 0,
        }
    }

    pub fn next_scope(&mut self) {
        self.scope_id += 1;
    }

    pub fn lookup_symbol(&self, sym_name: &str, sym_kind: &SymbolKind) -> Option<&Symbol> {
        self.sym_table
            .lookup_symbol(self.scope_id, sym_name, sym_kind)
    }

    pub fn lookup_symbol_recur(&self, sym_name: &str, sym_kind: &SymbolKind) -> Option<&Symbol> {
        self.sym_table
            .lookup_symbol_recur(self.scope_id, sym_name, sym_kind)
    }
}
