use crate::ast::semantic::{Scope, Variable};
use std::collections::HashMap;

type ScopeId = u64;

pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: Default::default() }
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id, scope);
    }

    pub fn get_scope(&self, scope_id: ScopeId) -> &Scope {
        self.scopes.get(&scope_id).unwrap()
    }

    pub fn lookup_var(&self, scope_id: ScopeId, var_name: &str) -> Option<&Variable> {
        let scope = self.scopes.get(&scope_id);

        if scope.is_none() {
            return None;
        }

        scope.unwrap().lookup_var(var_name)
    }

    pub fn recursive_lookup_var(&self, root_scope_id: ScopeId, var_name: &str) -> Option<&Variable> {
        let mut scope = self.get_scope(root_scope_id);

        loop {
            let var = self.lookup_var(scope.id, var_name);
            if var.is_some() {
                return var
            }

            if scope.parent_id.is_none() {
                return None;
            }

            let parent_id = scope.parent_id.unwrap();
            scope = self.get_scope(parent_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_scope_var_does_not_exist() {
        let scope = Scope::new(100, None);

        let mut sym_table = SymbolTable::new();
        sym_table.add_scope(scope);

        assert_eq!(None, sym_table.lookup_var(100, "A"));
    }

    #[test]
    fn one_scope_var_exist_under_scope() {
        let var = Variable::build_global("A");

        let mut scope = Scope::new(100, None);
        scope.store(var.clone());

        let mut sym_table = SymbolTable::new();
        sym_table.add_scope(scope);

        assert_eq!(var, *sym_table.lookup_var(100, "A").unwrap());
    }

    #[test]
    fn multiple_scopes_inner_scope_var_exists_while_shadowing_an_outer_scope() {
        //
        // Scope 100
        // |
        // | variable A=101 (outer)
        // |
        // |---- Scope 200
        //     |
        //     | variable A=201 (inner)
        //     |

        let mut sym_table = SymbolTable::new();

        // outer scope
        let mut outer_scope = Scope::new(100, None);
        let mut var_outer = Variable::build_local("A");
        var_outer.set_reference(101);
        outer_scope.store(var_outer.clone());
        sym_table.add_scope(outer_scope);

        // inner scope
        let mut inner_scope = Scope::new(200, Some(100));
        let mut var_inner = Variable::build_local("A");
        var_inner.set_reference(201);
        inner_scope.store(var_inner.clone());
        sym_table.add_scope(inner_scope);

        assert_eq!(var_inner, *sym_table.lookup_var(200, "A").unwrap());
        assert_eq!(var_outer, *sym_table.lookup_var(100, "A").unwrap());
        assert_ne!(var_inner, var_outer);
    }

    #[test]
    fn multiple_scopes_var_does_exist_on_parent_scope() {
        //
        // Scope 100
        // |
        // | variable A=101 (outer)
        // |
        // |---- Scope 200
        //     |
        //     |
        //     |---- Scope 300

        let mut sym_table = SymbolTable::new();

        // scope 100
        let mut scope_100 = Scope::new(100, None);
        let mut var = Variable::build_local("A");
        var.set_reference(101);
        scope_100.store(var.clone());
        sym_table.add_scope(scope_100);

        // scope 200
        let mut scope_200 = Scope::new(200, Some(100));
        sym_table.add_scope(scope_200);

        // scope 300
        let mut scope_300 = Scope::new(300, Some(200));
        sym_table.add_scope(scope_300);

        assert_eq!(var, *sym_table.recursive_lookup_var(300, "A").unwrap());
        assert_eq!(var, *sym_table.recursive_lookup_var(200, "A").unwrap());
        assert_eq!(var, *sym_table.recursive_lookup_var(100, "A").unwrap());
    }

    #[test]
    fn multiple_scopes_var_does_not_exist_at_no_scope() {
        //
        // Scope 100
        // |
        // |---- Scope 200
        //     |
        //     |---- Scope 300

        let mut sym_table = SymbolTable::new();

        // scope 100
        let mut scope_100 = Scope::new(100, None);
        sym_table.add_scope(scope_100);

        // scope 200
        let mut scope_200 = Scope::new(200, Some(100));
        sym_table.add_scope(scope_200);

        // scope 300
        let mut scope_300 = Scope::new(300, Some(200));
        sym_table.add_scope(scope_300);

        assert_eq!(None, sym_table.recursive_lookup_var(300, "A"));
    }
}
