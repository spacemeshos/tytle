use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};

struct SymbolTableGenerator {
    sym_table: SymbolTable,
    global_ref: u64,
    proc_ref: u64,
    proc_locals_ref: u64,
}

type SymbolTableResult<'a> = Result<&'a SymbolTable, AstWalkError>;

macro_rules! walk_err {
    ($($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            let err = AstWalkError::new(&msg);
            Err(err)
        }
    }
}

impl<'a> AstWalker<'a> for SymbolTableGenerator {
    // * TODO: ensure expression-literal symbol exists
    // * TODO: ensure expression-procedure call symbol exists
    // * TODO: avoid duplicate global/local declarations
    // * TODO: ensure that each global variable reference in a procedure
    //         takes place only after global variable

    fn on_make_local_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_local_var_symbol(&make_stmt)
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_start(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_param(&mut self, proc_stmt: &ProcedureStmt, param: &ProcParam) -> AstWalkResult {
        // TODO: create local symbol for param

        Ok(())
    }

    fn on_block_stmt_start(&mut self, _block_stmt: &BlockStatement) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_block_stmt_end(&mut self, _block_stmt: &BlockStatement) -> AstWalkResult {
        self.end_scope();
        Ok(())
    }
}

impl SymbolTableGenerator {
    pub fn new() -> Self {
        Self {
            sym_table: SymbolTable::new(),
            global_ref: 0,
            proc_ref: 0,
            proc_locals_ref: 0,
        }
    }

    pub fn generate(&mut self, ast: &Ast) -> SymbolTableResult {
        self.start_scope();
        self.prewalk_ast(ast)?;
        self.walk_ast(ast);
        self.end_scope();

        Ok(&self.sym_table)
    }

    pub fn prewalk_ast(&mut self, ast: &Ast) -> AstWalkResult {
        for stmt in &ast.statements {
            match stmt {
                Statement::Make(make_stmt) => match make_stmt.kind {
                    MakeStmtKind::Global => {
                        self.create_global_var_symbol(make_stmt)?;
                    }
                    MakeStmtKind::Local => {
                        return walk_err!(
                            "not allowed to delcare local variables under root scope (`{}`)",
                            make_stmt.var
                        );
                    }
                    MakeStmtKind::Assign => {
                        // TODO: will return an error in case there is no global variable `make_stmt.var`
                        self.get_var_symbol(&make_stmt.var)?;
                    }
                },
                Statement::Procedure(proc_stmt) => {
                    self.create_proc_symbol(proc_stmt)?;
                }
                _ => continue,
            }
        }

        Ok(())
    }

    fn get_var_symbol(&self, var_name: &str) -> Result<&Variable, AstWalkError> {
        let symbol = self.try_get_symbol(var_name, SymbolKind::Var);

        if symbol.is_some() {
            if let Symbol::Var(ref var) = symbol.unwrap() {
                Ok(var)
            } else {
                panic!("symbol should have been a variable")
            }
        } else {
            let err = AstWalkError::MissingVarDeclaration(var_name.to_owned());
            Err(err)
        }
    }

    fn get_proc_symbol(&self, proc_name: &str) -> Result<&Procedure, AstWalkError> {
        let symbol = self.try_get_symbol(proc_name, SymbolKind::Proc);

        if symbol.is_some() {
            if let Symbol::Proc(ref proc) = symbol.unwrap() {
                Ok(proc)
            } else {
                walk_err!("expected a variable for symbol {}", proc_name)
            }
        } else {
            walk_err!("procedure declaration missing for {}", proc_name)
        }
    }

    fn try_get_symbol(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table
            .recursive_lookup_sym(current_scope_id, name, &kind)
    }

    fn create_proc_symbol(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol(&proc_stmt.name, SymbolKind::Proc);

        if symbol.is_some() {
            panic!("duplicate direction!");
        }

        let proc = Procedure {
            name: proc_stmt.name.to_owned(),
            reference: Some(self.proc_ref),
            params_types: None,
            return_type: None,
        };

        self.proc_ref += 1;
        self.proc_locals_ref = 0; // we reset the new procedure locals counter

        self.sym_table.create_proc_symbol(proc);

        Ok(())
    }

    fn create_global_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol(&make_stmt.var, SymbolKind::Var);

        if symbol.is_none() {
            self.create_var_symbol(make_stmt, true, self.global_ref)
        } else {
            let err = AstWalkError::DuplicateGlobalVar(make_stmt.var.to_owned());
            Err(err)
        }
    }

    fn create_local_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_var_symbol(make_stmt, false, self.proc_locals_ref)
    }

    fn create_var_symbol(
        &mut self,
        make_stmt: &MakeStmt,
        is_global: bool,
        reference: u64,
    ) -> AstWalkResult {
        let var = Variable {
            global: is_global,
            name: make_stmt.var.to_owned(),
            reference: Some(reference),
            resolved_type: None,
        };

        self.sym_table.create_var_symbol(var);

        if is_global {
            self.global_ref += 1;
        } else {
            self.proc_locals_ref += 1;
        }

        Ok(())
    }

    fn start_scope(&mut self) {
        self.sym_table.start_scope();
    }

    fn end_scope(&mut self) {
        self.sym_table.end_scope();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_global_assign_before_declare() {
        let code = r#"
            MAKE "A=20
        "#;

        let expected = AstWalkError::MissingVarDeclaration("A".to_string());

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let actual = generator.generate(&ast).err().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_duplicate_global_variable_declaration() {
        let code = r#"
            MAKEGLOBAL "A=10
            MAKEGLOBAL "A=20
        "#;

        let expected = AstWalkError::DuplicateGlobalVar("A".to_string());

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let actual = generator.generate(&ast).err().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn prewalk_make_and_procs() {
        let code = r#"
            MAKE "A=20

            TO MOVE_FORWARD
                FORWARD 10
            END

            TO MOVE_BACKWARD
                BACKWARD 10
            END

            MAKE "B=30
            MAKE "C=40

            "#;

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        generator.generate(&ast);
    }
}
