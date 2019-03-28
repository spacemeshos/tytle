use crate::ast::semantic::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<SymbolId, Symbol>,
    scopes: HashMap<ScopeId, Scope>,
    depth_scopes_stack: HashMap<usize, Vec<ScopeId>>,
    next_scope_id: ScopeId,
    next_scope_depth: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self {
            symbols: Default::default(),
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
        // each root's child-scope holds: `parent_id = Some(0)`

        table.start_scope();

        table
    }

    pub fn is_root_scope(&self) -> bool {
        self.get_current_scope().parent_id.is_none()
    }

    pub fn is_inner_scope(&self) -> bool {
        !(self.is_root_scope())
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

    pub fn get_var_by_id(&self, var_id: SymbolId) -> &Variable {
        let symbol = self.lookup_by_symbol_id(var_id);
        symbol.unwrap().as_var()
    }

    pub fn get_var_by_id_mut(&mut self, var_id: SymbolId) -> &mut Variable {
        let symbol = self.lookup_by_symbol_id_mut(var_id);
        symbol.unwrap().as_var_mut()
    }

    pub fn get_proc_by_id(&self, proc_id: SymbolId) -> &Procedure {
        let symbol = self.lookup_by_symbol_id(proc_id);
        symbol.unwrap().as_proc()
    }

    pub fn get_proc_by_id_mut(&mut self, proc_id: SymbolId) -> &mut Procedure {
        let symbol = self.lookup_by_symbol_id_mut(proc_id);
        symbol.unwrap().as_proc_mut()
    }

    pub fn lookup(
        &self,
        scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&Symbol> {
        let scope = self.scopes.get(&scope_id);

        if scope.is_none() {
            return None;
        }

        let symbol_id = scope.unwrap().lookup(sym_name, &sym_kind);

        if symbol_id.is_some() {
            let symbol_id = symbol_id.unwrap();

            self.symbols.get(symbol_id)
        } else {
            return None;
        }
    }

    pub fn lookup_mut(
        &mut self,
        scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&mut Symbol> {
        let scope: Option<&mut Scope> = self.scopes.get_mut(&scope_id);

        if scope.is_none() {
            return None;
        }

        let symbol_id = scope.unwrap().lookup(sym_name, &sym_kind);

        if symbol_id.is_some() {
            let symbol_id = symbol_id.unwrap();

            self.symbols.get_mut(symbol_id)
        } else {
            return None;
        }
    }

    pub fn create_var_symbol(&mut self, var: Variable) {
        let mut var_sym = self.lookup(self.get_current_scope_id(), &var.name, &SymbolKind::Var);

        if var_sym.is_some() {
            panic!("variable {} already exists under the scope", var.name);
        }

        self.store_var(var);
    }

    pub fn create_proc_symbol(&mut self, proc: Procedure) {
        let mut proc_sym = self.lookup(self.next_scope_depth, &proc.name, &SymbolKind::Proc);

        if proc_sym.is_some() {
            panic!("procedure {} already exists under the scope", proc.name);
        }

        self.store_proc(proc);
    }

    pub fn get_proc_by_name(&self, proc_name: &str) -> &Procedure {
        let symbol = self.lookup(0, proc_name, &SymbolKind::Proc);
        symbol.unwrap().as_proc()
    }

    pub fn get_current_scope_id(&self) -> usize {
        self.next_scope_id - 1
    }

    pub fn get_next_scope_parent_id(&self) -> Option<usize> {
        match self.next_scope_depth {
            0 => None, // root scope
            _ => {
                let parent_scope_depth = self.next_scope_depth - 1;
                let stack = self.depth_scopes_stack.get(&parent_scope_depth).unwrap();
                let pscope_id = *stack.last().unwrap();

                Some(pscope_id)
            }
        }
    }

    pub fn get_current_scope(&self) -> &Scope {
        let scope_depth = self.next_scope_depth - 1;

        let stack = self.depth_scopes_stack.get(&scope_depth).unwrap();
        let scope_id: usize = *stack.last().unwrap();

        self.scopes.get(&scope_id).unwrap()
    }

    fn store_var(&mut self, var: Variable) {
        let var_id = var.id;
        let var_name = var.name.to_string();

        let symbol = Symbol::Var(var);

        self.store_symbol(var_name, var_id, symbol);
    }

    fn store_proc(&mut self, proc: Procedure) {
        let proc_id = proc.id;
        let proc_name = proc.name.to_string();

        let symbol = Symbol::Proc(proc);

        self.store_symbol(proc_name, proc_id, symbol);
    }

    fn store_symbol(&mut self, symbol_name: String, symbol_id: SymbolId, symbol: Symbol) {
        let scope_id = self.get_current_scope_id();
        let scope = self.get_scope_mut(scope_id);

        scope.store(symbol_name, symbol_id, symbol.kind());

        self.symbols.insert(symbol_id, symbol);
    }

    fn lookup_by_symbol_id(&self, symbol_id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(&symbol_id)
    }

    fn lookup_by_symbol_id_mut(&mut self, symbol_id: SymbolId) -> Option<&mut Symbol> {
        self.symbols.get_mut(&symbol_id)
    }

    pub fn lookup_recur(
        &self,
        start_scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&Symbol> {
        let mut scope_id = start_scope_id;

        loop {
            let mut scope = self.get_scope(scope_id);

            let var = self.lookup(scope.id, sym_name, sym_kind);
            if var.is_some() {
                return var;
            }

            if scope.is_root_scope() {
                return None;
            }

            scope_id = scope.parent_id.unwrap();
        }
    }

    pub fn lookup_recur_mut(
        &mut self,
        start_scope_id: ScopeId,
        sym_name: &str,
        sym_kind: &SymbolKind,
    ) -> Option<&mut Symbol> {
        let mut scope_id = start_scope_id;

        loop {
            let mut scope = self.get_scope(scope_id);

            let var = self.lookup(scope.id, sym_name, sym_kind);
            if var.is_some() {
                // we've found the variable (variable `sym_name` resides under scope `scode.id`)
                // since we borrowed it as immutable,
                // we re-borrow it again, but this time in a mutable manner
                return self.lookup_mut(scope_id, sym_name, sym_kind);
            }

            if scope.is_root_scope() {
                return None;
            }

            scope_id = scope.parent_id.unwrap();
        }
    }
}
