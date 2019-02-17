use crate::ast::semantic::*;
use std::collections::{HashMap, HashSet};

type ScopeId = u64;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    depth_scopes_stack: HashMap<u64, Vec<ScopeId>>,
    next_scope_id: ScopeId,
    next_scope_depth: u64,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self {
            scopes: Default::default(),
            depth_scopes_stack: Default::default(),
            next_scope_id: 0,
            next_scope_depth: 0,
        };

        // we always start a new `SymbolTable` with a default scope (a.k.a the "root scope")
        // this scope has:
        // * `id` = 0
        // * `parent_id = None`
        //
        // each root scope child scope holds: `parent_id = Some(0)`

        table.start_scope();

        table
    }

    pub fn start_scope(&mut self) -> &mut Scope {
        let parent_scope_id = self.get_next_scope_parent_id();

        let scope_id = self.next_scope_id;
        let scope_depth = self.next_scope_depth;

        let scope = Scope::new(scope_id, parent_scope_id);

        self.next_scope_depth += 1;
        self.next_scope_id += 1;

        let entry = self
            .depth_scopes_stack
            .entry(scope_depth)
            .or_insert(Vec::new());

        entry.push(scope_id);

        self.scopes.insert(scope_id, scope);

        self.scopes.get_mut(&scope_id).unwrap()
    }

    pub fn end_scope(&mut self) {
        assert!(self.next_scope_depth > 0);

        let scope_depth = self.next_scope_depth - 1;
        let stack = self.depth_scopes_stack.get_mut(&scope_depth).unwrap();
        stack.pop();

        if stack.len() == 0 {
            self.depth_scopes_stack.remove(&scope_depth);
        }

        self.next_scope_depth -= 1;
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

    pub fn lookup_symbol_recur(
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
        let mut proc_sym = self.lookup_symbol(self.next_scope_depth, &proc.name, &SymbolKind::Proc);

        if proc_sym.is_some() {
            panic!("procedure {} already exists under the scope", proc.name);
        }

        self.store_symbol_under_current_scope(Symbol::Proc(proc), &SymbolKind::Proc);
    }

    pub fn get_current_scope_id(&self) -> u64 {
        self.next_scope_id - 1
    }

    pub fn get_next_scope_parent_id(&self) -> Option<u64> {
        match self.next_scope_depth {
            0 => None, // root scope
            _ => {
                let parent_scope_depth = self.next_scope_depth - 1;
                let stack = self.depth_scopes_stack.get(&parent_scope_depth).unwrap();
                let pscope_id: u64 = *stack.last().unwrap();

                Some(pscope_id)
            }
        }
    }

    pub fn get_current_scope(&self) -> &Scope {
        let scope_depth = self.next_scope_depth - 1;

        let stack = self.depth_scopes_stack.get(&scope_depth).unwrap();
        let scope_id: u64 = *stack.last().unwrap();

        self.scopes.get(&scope_id).unwrap()
    }

    pub fn is_root_scope(&self) -> bool {
        self.get_current_scope().parent_id.is_none()
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
