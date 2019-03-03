use crate::ast::semantic::{Procedure, Variable, ScopeId, Symbol, SymbolKind, SymbolTable};

#[derive(Debug)]
pub struct SymbolTableVisitor<'a> {
    sym_table: &'a mut SymbolTable,
    scope_id: ScopeId, // current visited scope
}

impl<'a> SymbolTableVisitor<'a> {
    pub fn new(sym_table: &'a mut SymbolTable) -> Self {
        Self {
            sym_table,
            scope_id: 0, // we start visiting from the root scope
        }
    }

    pub fn next_scope(&mut self) {
        self.scope_id += 1;
    }

    pub fn lookup(&self, sym_name: &str, sym_kind: &SymbolKind) -> Option<&Symbol> {
        self.sym_table.lookup(self.scope_id, sym_name, sym_kind)
    }

    pub fn lookup_recur(&self, sym_name: &str, sym_kind: &SymbolKind) -> Option<&Symbol> {
        self.sym_table
            .lookup_recur(self.scope_id, sym_name, sym_kind)
    }

    pub fn lookup_recur_mut(
        &mut self,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&mut Symbol> {
        self.sym_table
            .lookup_recur_mut(self.scope_id, sym_name, sym_kind)
    }

    pub fn lookup_var(&self, name: &str) -> &Variable {
        self.lookup_recur(name, &SymbolKind::Var)
            .unwrap()
            .as_var()
    }

    pub fn lookup_proc(&self, name: &str) -> &Procedure {
        self.lookup_recur(name, &SymbolKind::Proc)
            .unwrap()
            .as_proc()
    }
}
