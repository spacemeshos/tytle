use crate::ast::semantic::*;
use std::collections::{HashMap, HashSet};

type ScopeId = u64;

#[derive(Debug)]
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    next_scope_id: u64,
    current_scope_id: u64,
    scope_depth: u64,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Default::default(),
            next_scope_id: 1,
            scope_depth: 0,
            current_scope_id: 0,
        }
    }

    pub fn start_scope(&mut self) -> &mut Scope {
        let parent_scope_id = match self.scope_depth {
            0 => None,
            _ => Some(self.next_scope_id - 1),
        };

        let scope_id = self.next_scope_id;

        let scope = Scope::new(scope_id, parent_scope_id);

        self.scope_depth += 1;
        self.next_scope_id += 1;
        self.current_scope_id = scope.id;

        self.scopes.insert(scope_id, scope);

        self.scopes.get_mut(&scope_id).unwrap()
    }

    pub fn end_scope(&mut self) {
        assert!(self.scope_depth > 0);

        let scope = self.get_current_scope().unwrap();

        if let Some(pscope_id) = scope.parent_id {
            self.current_scope_id = pscope_id;
        } else {
            self.current_scope_id = 0;
        }

        self.scope_depth -= 1;
    }

    pub fn get_scope(&self, scope_id: ScopeId) -> &Scope {
        self.scopes.get(&scope_id).unwrap()
    }

    pub fn get_scope_mut(&mut self, scope_id: ScopeId) -> &mut Scope {
        self.scopes.get_mut(&scope_id).unwrap()
    }

    pub fn lookup_symbol(
        &self,
        scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&Symbol> {
        let scope = self.scopes.get(&scope_id);

        if scope.is_none() {
            return None;
        }

        scope.unwrap().lookup_symbol(sym_name, &sym_kind)
    }

    pub fn recursive_lookup_sym(
        &self,
        root_scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&Symbol> {
        let mut scope = self.get_scope(root_scope_id);

        loop {
            let var = self.lookup_symbol(scope.id, sym_name, sym_kind);
            if var.is_some() {
                return var;
            }

            if scope.parent_id.is_none() {
                return None;
            }

            let parent_id = scope.parent_id.unwrap();
            scope = self.get_scope(parent_id);
        }
    }

    pub fn create_var_symbol(&mut self, var: Variable) {
        let mut var_sym =
            self.lookup_symbol(self.get_current_scope_id(), &var.name, &SymbolKind::Var);

        if var_sym.is_some() {
            panic!("variable {} already exists under the scope", var.name);
        }

        self.store_symbol_under_current_scope(Symbol::Var(var), &SymbolKind::Var);
    }

    pub fn create_proc_symbol(&mut self, proc: Procedure) {
        let mut proc_sym = self.lookup_symbol(self.scope_depth, &proc.name, &SymbolKind::Proc);

        if proc_sym.is_some() {
            panic!("procedure {} already exists under the scope", proc.name);
        }

        self.store_symbol_under_current_scope(Symbol::Proc(proc), &SymbolKind::Proc);
    }

    pub fn get_current_scope_id(&self) -> u64 {
        self.next_scope_id - 1
    }

    pub fn get_current_scope(&self) -> Option<&Scope> {
        self.scopes.get(&self.current_scope_id)
    }

    pub fn is_root_scope(&self) -> bool {
        self.get_current_scope().is_none()
    }

    pub fn is_inner_scope(&self) -> bool {
        !(self.is_root_scope())
    }

    fn store_symbol_under_current_scope(&mut self, symbol: Symbol, kind: &SymbolKind) {
        let scope_id = self.get_current_scope_id();

        let scope = self.get_scope_mut(scope_id);
        scope.store(symbol, kind);
    }
}
