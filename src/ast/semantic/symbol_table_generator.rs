use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};

struct SymbolTableGenerator {
    sym_table: SymbolTable,
    global_ref: u64,
    proc_ref: u64,
}

type SymbolTableResult<'a> = Result<&'a SymbolTable, AstWalkError>;

impl<'a> AstWalker<'a> for SymbolTableGenerator {
    // * TODO: ensure expression-literal symbol exists
    // * TODO: ensure expression-procedure call symbol exists
    // * TODO: avoid duplicate global/local declarations
    // * TODO: ensure that each global variable reference in a procedure
    // * takes place only after global global variable

    fn on_make_global_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_global_var_symbol(&make_stmt)
    }

    fn on_make_local_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_local_var_symbol(&make_stmt)
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_start(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        let proc = Procedure {
            name: proc_stmt.name.to_owned(),
            reference: Some(self.proc_ref),
            params_types: None,
            return_type: None,
        };

        self.proc_ref += 1;

        self.sym_table.create_proc_symbol(proc);

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
        }
    }

    pub fn build_sym_table(&mut self, ast: &Ast) -> SymbolTableResult {
        self.start_scope();
        self.walk_ast(ast);
        self.end_scope();

        Ok(&self.sym_table)
    }

    fn get_var_symbol(&self, var_name: &str) -> Result<&Variable, AstWalkError> {
        let symbol = self.try_get_symbol(var_name);

        if symbol.is_some() {
            if let Symbol::Var(ref var) = symbol.unwrap() {
                Ok(var)
            } else {
                let err = AstWalkError::new(format!("expected a variable for symbol {}", var_name));
                Err(err)
            }
        } else {
            let err =
                AstWalkError::new(format!("variable declaration is missing for {}", var_name));
            Err(err)
        }
    }

    fn get_proc_symbol(&self, proc_name: &str) -> Result<&Procedure, AstWalkError> {
        let symbol = self.try_get_symbol(proc_name);

        if symbol.is_some() {
            if let Symbol::Proc(ref proc) = symbol.unwrap() {
                Ok(proc)
            } else {
                let err =
                    AstWalkError::new(format!("expected a variable for symbol {}", proc_name));
                Err(err)
            }
        } else {
            let err = AstWalkError::new(format!("procedure declaration missing for {}", proc_name));
            Err(err)
        }
    }

    fn try_get_symbol(&self, name: &str) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table.recursive_lookup_sym(current_scope_id, name)
    }

    fn create_global_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_var_symbol(make_stmt, true)
    }

    fn create_local_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_var_symbol(make_stmt, false)
    }

    fn create_var_symbol(&mut self, make_stmt: &MakeStmt, is_global: bool) -> AstWalkResult {
        let var = Variable {
            global: is_global,
            name: make_stmt.var.to_owned(),
            reference: Some(self.global_ref),
            resolved_type: None,
        };

        self.sym_table.create_var_symbol(var);

        if is_global {
            self.global_ref += 1;
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
    }
}
