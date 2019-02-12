use crate::ast::semantic::{PrimitiveType, Procedure, Scope, Symbol, Variable};
use std::collections::{HashMap, HashSet};

type ScopeId = u64;

#[derive(Debug)]
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    next_scope_id: u64,
    scope_depth: u64,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Default::default(),
            next_scope_id: 1,
            scope_depth: 0,
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

        self.scopes.insert(scope_id, scope);

        self.scopes.get_mut(&scope_id).unwrap()
    }

    pub fn end_scope(&mut self) {
        assert!(self.scope_depth > 0);

        self.scope_depth -= 1;
    }

    pub fn get_scope(&self, scope_id: ScopeId) -> &Scope {
        self.scopes.get(&scope_id).unwrap()
    }

    pub fn get_scope_mut(&mut self, scope_id: ScopeId) -> &mut Scope {
        self.scopes.get_mut(&scope_id).unwrap()
    }

    pub fn lookup_symbol(&self, scope_id: ScopeId, sym_name: &str) -> Option<&Symbol> {
        let scope = self.scopes.get(&scope_id);

        if scope.is_none() {
            return None;
        }

        scope.unwrap().lookup_symbol(sym_name)
    }

    pub fn recursive_lookup_sym(&self, root_scope_id: ScopeId, sym_name: &str) -> Option<&Symbol> {
        let mut scope = self.get_scope(root_scope_id);

        loop {
            let var = self.lookup_symbol(scope.id, sym_name);
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
        let mut var_sym = self.lookup_symbol(self.get_current_scope_id(), &var.name);

        if var_sym.is_some() {
            panic!("variable {} already exists under the scope", var.name);
        }

        self.store_symbol_under_current_scope(Symbol::Var(var));
    }

    pub fn create_proc_symbol(&mut self, proc: Procedure) {
        let mut proc_sym = self.lookup_symbol(self.scope_depth, &proc.name);

        if proc_sym.is_some() {
            panic!("procedure {} already exists under the scope", proc.name);
        }

        self.store_symbol_under_current_scope(Symbol::Proc(proc));
    }

    pub fn get_current_scope_id(&self) -> u64 {
        self.next_scope_id - 1
    }

    fn store_symbol_under_current_scope(&mut self, symbol: Symbol) {
        let scope_id = self.get_current_scope_id();

        let scope = self.get_scope_mut(scope_id);
        scope.store(symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_scope_var_does_not_exist() {
        let mut sym_table = SymbolTable::new();
        let scope = sym_table.start_scope();
        let scope_id = scope.id;

        assert_eq!(None, sym_table.lookup_symbol(scope_id, "A"));
    }

    #[test]
    fn one_scope_var_exists() {
        let var = Variable::build_global("A");

        let mut sym_table = SymbolTable::new();
        let scope = sym_table.start_scope();
        let scope_id = scope.id;

        sym_table.create_var_symbol(var.clone());

        assert_eq!(
            Symbol::Var(var),
            *sym_table.lookup_symbol(scope_id, "A").unwrap()
        );
    }

    #[test]
    fn multiple_nested_scopes_inner_scope_var_exists_while_shadowing_an_outer_scope_var() {
        //
        // Scope outer
        // |
        // | variable A=100 (outer)
        // |
        // |---- Scope inner
        //     |
        //     | variable A=200 (inner)
        //     |

        let mut sym_table = SymbolTable::new();

        // outer scope
        let outer_scope = sym_table.start_scope();
        let outer_scope_id = outer_scope.id;
        let mut var_outer = Variable::build_local("A");
        var_outer.set_reference(100);
        sym_table.create_var_symbol(var_outer.clone());

        let mut var_inner = Variable::build_local("A");
        var_inner.set_reference(200);
        let inner_scope = sym_table.start_scope();
        let inner_scope_id = inner_scope.id;
        sym_table.create_var_symbol(var_inner.clone());

        assert_eq!(outer_scope_id, 1);
        assert_eq!(inner_scope_id, 2);

        assert_eq!(
            Symbol::Var(var_inner),
            *sym_table.lookup_symbol(inner_scope_id, "A").unwrap()
        );

        assert_eq!(
            Symbol::Var(var_outer),
            *sym_table.lookup_symbol(outer_scope_id, "A").unwrap()
        );
    }

    #[test]
    fn multiple_nested_scopes_var_does_exist_on_parent_scope() {
        //
        // Scope X
        // |
        // | variable A=100
        // |
        // |---- Scope Y
        //     |
        //     |
        //     |---- Scope Z

        let mut sym_table = SymbolTable::new();

        // scope X
        let scope_x = sym_table.start_scope();
        let scope_x_id = scope_x.id;

        // var
        let mut var = Variable::build_local("A");
        var.set_reference(100);
        sym_table.create_var_symbol(var.clone());

        // scope Y
        let scope_y = sym_table.start_scope();
        let scope_y_id = scope_y.id;

        // scope Z
        let scope_z = sym_table.start_scope();
        let scope_z_id = scope_z.id;

        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_sym(scope_z_id, "A").unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_sym(scope_y_id, "A").unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_sym(scope_x_id, "A").unwrap()
        );
    }

    #[test]
    fn multiple_nested_scopes_var_does_not_exist_at_any_scope() {
        //
        // Scope X
        // |
        // |---- Scope Y
        //     |
        //     |---- Scope Z

        let mut sym_table = SymbolTable::new();
        sym_table.start_scope(); // scope X
        sym_table.start_scope(); // scope Y
        let scope_z = sym_table.start_scope(); // scope Z
        let scope_z_id = scope_z.id;

        assert_eq!(None, sym_table.recursive_lookup_sym(scope_z_id, "A"));
    }

    #[test]
    fn multiple_not_nested_scopes_var_exist_under_exactly_one_scope() {
        //
        // Scope X
        // |
        // |------
        //
        //  Scope Y
        // |
        // | variable A (reference=100)
        // |----
        //
        // Scope Z
        // |
        // |------

        let mut sym_table = SymbolTable::new();

        // scope X
        let scope_x = sym_table.start_scope(); // scope X
        let scope_x_id = scope_x.id;
        sym_table.end_scope();

        // scope Y
        let scope_y = sym_table.start_scope(); // scope Y
        let scope_y_id = scope_y.id;
        let mut var = Variable::build_local("A");
        var.set_reference(100);
        sym_table.create_var_symbol(var.clone());
        sym_table.end_scope();

        // scope Z
        let scope_z = sym_table.start_scope(); // scope Z
        let scope_z_id = scope_z.id;
        sym_table.end_scope();

        assert_eq!(scope_x_id, 1);
        assert_eq!(scope_y_id, 2);
        assert_eq!(scope_z_id, 3);

        assert_eq!(None, sym_table.recursive_lookup_sym(scope_x_id, "A"));
        assert_eq!(None, sym_table.recursive_lookup_sym(scope_z_id, "A"));
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_sym(scope_y_id, "A").unwrap()
        );
    }
}
