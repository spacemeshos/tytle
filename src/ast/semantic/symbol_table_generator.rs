use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};

pub struct SymbolTableGenerator {
    sym_table: SymbolTable,
    global_ref: u64,
    proc_ref: u64,
    proc_locals_ref: u64,
}

type SymbolTableResult<'a> = Result<&'a SymbolTable, AstWalkError>;

impl<'a> AstWalker<'a> for SymbolTableGenerator {
    fn on_make_global_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        if self.sym_table.is_inner_scope() {
            let err = AstWalkError::ProcNotAllowedToDeclareGlobals(make_stmt.var.to_string());
            Err(err)
        } else {
            Ok(())
        }
    }

    fn on_make_local_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.create_local_var_symbol(&make_stmt)
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        self.get_var_symbol(&make_stmt.var)?;
        Ok(())
    }

    fn on_proc_param(&mut self, proc_stmt: &ProcedureStmt, param: &ProcParam) -> AstWalkResult {
        let symbol = self.try_get_symbol(&param.param_name, SymbolKind::Var);

        if symbol.is_none() {
            self.create_var_symbol(&param.param_name, false, self.proc_locals_ref)
        } else {
            let err = AstWalkError::DuplicateProcParam(
                proc_stmt.name.to_string(),
                param.param_name.to_string(),
            );
            Err(err)
        }
    }

    fn on_proc_start(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_proc_end(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        self.end_scope();
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
        self.prewalk_ast(ast)?;
        self.walk_ast(ast)?;

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
                        let err =
                            AstWalkError::LocalsNotAllowedUnderRootScope(make_stmt.var.clone());
                        return Err(err);
                    }
                    _ => continue,
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
        let symbol = self.try_get_symbol_recur(var_name, SymbolKind::Var);

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

    fn create_proc_symbol(&mut self, proc_stmt: &ProcedureStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol_recur(&proc_stmt.name, SymbolKind::Proc);

        if symbol.is_none() {
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
        } else {
            let err = AstWalkError::DuplicateProc(proc_stmt.name.to_owned());
            Err(err)
        }
    }

    fn create_global_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol_recur(&make_stmt.var, SymbolKind::Var);

        if symbol.is_none() {
            self.create_var_symbol(&make_stmt.var, true, self.global_ref)
        } else {
            let err = AstWalkError::DuplicateGlobalVar(make_stmt.var.to_owned());
            Err(err)
        }
    }

    fn create_local_var_symbol(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol(&make_stmt.var, SymbolKind::Var);

        if symbol.is_none() {
            self.create_var_symbol(&make_stmt.var, false, self.proc_locals_ref)
        } else {
            let err = AstWalkError::DuplicateProcLocalVar(make_stmt.var.to_owned());
            Err(err)
        }
    }

    fn create_var_symbol(
        &mut self,
        var_name: &str,
        is_global: bool,
        reference: u64,
    ) -> AstWalkResult {
        let var = Variable {
            global: is_global,
            name: var_name.to_owned(),
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

    fn try_get_symbol_recur(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table
            .lookup_symbol_recur(current_scope_id, name, &kind)
    }

    fn try_get_symbol(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table.lookup_symbol(current_scope_id, name, &kind)
    }

    fn start_scope(&mut self) {
        self.sym_table.start_scope();
    }

    fn end_scope(&mut self) {
        self.sym_table.end_scope();
    }
}
