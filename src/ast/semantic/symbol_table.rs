use crate::ast::semantic::{PrimitiveType, Procedure, Scope, Symbol, Variable};
use std::collections::{HashMap, HashSet};

type ScopeId = u64;

pub enum SymbolType {
    Constant,
    Variable,
    Procedure,
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    current_scope_id: u64,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Default::default(),
            current_scope_id: 0,
        }
    }

    pub fn add_scope(&mut self) -> &mut Scope {
        let parent_scope = match self.current_scope_id {
            0 => None,
            _ => Some(self.current_scope_id),
        };

        self.current_scope_id += 1;

        let scope = Scope::new(self.current_scope_id, parent_scope);

        self.scopes.insert(scope.id, scope);

        self.scopes.get_mut(&self.current_scope_id).unwrap()
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

    pub fn recursive_lookup_var(&self, root_scope_id: ScopeId, sym_name: &str) -> Option<&Symbol> {
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
        let mut var_sym = self.lookup_symbol(self.current_scope_id, &var.name);

        if var_sym.is_some() {
            panic!("variable {} already exists under the scope", var.name);
        }

        let scope = self.get_scope_mut(self.current_scope_id);
        scope.store(Symbol::Var(var));
    }

    pub fn create_proc_symbol(&mut self, proc: Procedure) {
        let mut proc_sym = self.lookup_symbol(self.current_scope_id, &proc.name);

        if proc_sym.is_some() {
            panic!("procedure {} already exists under the scope", proc.name);
        }

        let scope = self.get_scope_mut(self.current_scope_id);
        scope.store(Symbol::Proc(proc));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_scope_var_does_not_exist() {
        let mut sym_table = SymbolTable::new();
        let scope = sym_table.add_scope();
        let scope_id = scope.id;

        assert_eq!(None, sym_table.lookup_symbol(scope_id, "A"));
    }

    #[test]
    fn one_scope_var_exists() {
        let var = Variable::build_global("A");

        let mut sym_table = SymbolTable::new();
        let scope = sym_table.add_scope();
        let scope_id = scope.id;

        sym_table.create_var_symbol(var.clone());

        assert_eq!(
            Symbol::Var(var),
            *sym_table.lookup_symbol(scope_id, "A").unwrap()
        );
    }

    #[test]
    fn multiple_scopes_inner_scope_var_exists_while_shadowing_an_outer_scope_var() {
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
        let outer_scope = sym_table.add_scope();
        let outer_scope_id = outer_scope.id;
        let mut var_outer = Variable::build_local("A");
        var_outer.set_reference(100);
        sym_table.create_var_symbol(var_outer.clone());

        let mut var_inner = Variable::build_local("A");
        var_inner.set_reference(200);
        let inner_scope = sym_table.add_scope();
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
    fn multiple_scopes_var_does_exist_on_parent_scope() {
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
        let scope_x = sym_table.add_scope();
        let scope_x_id = scope_x.id;

        // var
        let mut var = Variable::build_local("A");
        var.set_reference(100);
        sym_table.create_var_symbol(var.clone());

        // scope Y
        let scope_y = sym_table.add_scope();
        let scope_y_id = scope_y.id;

        // scope Z
        let scope_z = sym_table.add_scope();
        let scope_z_id = scope_z.id;

        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_var(scope_z_id, "A").unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_var(scope_y_id, "A").unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table.recursive_lookup_var(scope_x_id, "A").unwrap()
        );
    }

    #[test]
    fn multiple_scopes_var_does_not_exist_at_any_scope() {
        //
        // Scope X
        // |
        // |---- Scope Y
        //     |
        //     |---- Scope Z

        let mut sym_table = SymbolTable::new();
        sym_table.add_scope(); // scope X
        sym_table.add_scope(); // scope Y
        let scope_z = sym_table.add_scope(); // scope Z
        let scope_z_id = scope_z.id;

        assert_eq!(None, sym_table.recursive_lookup_var(scope_z_id, "A"));
    }
}
