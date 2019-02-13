extern crate tytle;

use tytle::ast::semantic::{Symbol, SymbolKind, SymbolTable, Variable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_scope_var_does_not_exist() {
        let mut sym_table = SymbolTable::new();
        let scope = sym_table.start_scope();
        let scope_id = scope.id;

        assert_eq!(
            None,
            sym_table.lookup_symbol(scope_id, "A", &SymbolKind::Var)
        );
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
            *sym_table
                .lookup_symbol(scope_id, "A", &SymbolKind::Var)
                .unwrap()
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
            *sym_table
                .lookup_symbol(inner_scope_id, "A", &SymbolKind::Var)
                .unwrap()
        );

        assert_eq!(
            Symbol::Var(var_outer),
            *sym_table
                .lookup_symbol(outer_scope_id, "A", &SymbolKind::Var)
                .unwrap()
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
            *sym_table
                .recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
                .unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table
                .recursive_lookup_sym(scope_y_id, "A", &SymbolKind::Var)
                .unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table
                .recursive_lookup_sym(scope_x_id, "A", &SymbolKind::Var)
                .unwrap()
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

        assert_eq!(
            None,
            sym_table.recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
        );
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

        assert_eq!(
            None,
            sym_table.recursive_lookup_sym(scope_x_id, "A", &SymbolKind::Var)
        );
        assert_eq!(
            None,
            sym_table.recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *sym_table
                .recursive_lookup_sym(scope_y_id, "A", &SymbolKind::Var)
                .unwrap()
        );
    }
}
