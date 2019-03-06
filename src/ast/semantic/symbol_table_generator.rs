use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};

pub struct SymbolTableGenerator {
    sym_table: SymbolTable,
    id_generator: IdGenerator,
    globals_index: u64,
    proc_locals_index: u64,
}

type SymbolTableResult<'a> = Result<&'a mut SymbolTable, AstWalkError>;

impl<'a> AstWalker<'a> for SymbolTableGenerator {
    fn on_make_global_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        if self.sym_table.is_inner_scope() {
            let err = AstWalkError::ProcNotAllowedToDeclareGlobals(make_stmt.var_name.to_string());
            Err(err)
        } else {
            Ok(())
        }
    }

    fn on_make_local_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.create_local_var_symbol(make_stmt)
    }

    fn on_make_assign_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var = self.get_var_symbol(&make_stmt.var_name)?;

        make_stmt.var_id = Some(var.id);

        Ok(())
    }

    fn on_proc_param(&mut self, ctx_proc: &str, proc_param: &mut ProcParam) -> AstWalkResult {
        let symbol = self.try_get_symbol(&proc_param.param_name, SymbolKind::Var);

        if symbol.is_none() {
            let param_type = ExpressionType::from(proc_param.param_type.as_str());
            let index = self.proc_locals_index;

            self.create_var_symbol(&proc_param.param_name, Some(param_type), false, index)?;

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProcParam(
                ctx_proc.to_string(),
                proc_param.param_name.to_string(),
            );
            Err(err)
        }
    }

    fn on_proc_start(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_proc_end(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.end_scope();
        Ok(())
    }

    fn on_block_stmt_start(
        &mut self,
        ctx_proc: &str,
        block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_block_stmt_end(
        &mut self,
        ctx_proc: &str,
        block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        self.end_scope();
        Ok(())
    }

    fn on_literal_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let lit_expr: &mut LiteralExpr = expr.as_lit_expr_mut();

        match lit_expr {
            LiteralExpr::Var(var_name, var_id) => {
                let var = self.get_var_symbol(var_name);

                if var.is_ok() {
                    let var = var.unwrap();

                    var_id.replace(var.id);
                } else {
                    // TODO: return error is variable isn't found
                    unimplemented!()
                }
            }
            _ => {}
        };

        Ok(())
    }
}

impl SymbolTableGenerator {
    pub fn new() -> Self {
        Self {
            sym_table: SymbolTable::new(),
            id_generator: IdGenerator::new(),
            globals_index: 0,
            proc_locals_index: 0,
        }
    }

    pub fn generate(&mut self, ast: &mut Ast) -> SymbolTableResult {
        self.gen_main_proc_symbol();
        self.prewalk_ast(ast)?;
        self.walk_ast(ast)?;

        Ok(&mut self.sym_table)
    }

    pub fn prewalk_ast(&mut self, ast: &mut Ast) -> AstWalkResult {
        for stmt in &mut ast.statements {
            match stmt {
                Statement::Make(make_stmt) => match make_stmt.kind {
                    MakeStmtKind::Global => {
                        self.create_global_var_symbol(make_stmt)?;
                    }
                    MakeStmtKind::Local => {
                        let err = AstWalkError::LocalsNotAllowedUnderRootScope(
                            make_stmt.var_name.clone(),
                        );
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
            let ret_type_str = proc_stmt.return_type.clone();
            let return_type = ExpressionType::from(ret_type_str.as_str());

            let params_types = proc_stmt
                .params
                .iter()
                .map(|param| ExpressionType::from(param.param_type.as_str()))
                .collect::<Vec<ExpressionType>>();

            let id = self.get_next_id();

            let proc = Procedure {
                id,
                name: proc_stmt.name.to_owned(),
                params_types,
                return_type,
            };

            self.proc_locals_index = 0; // we reset the new procedure locals counter

            self.sym_table.create_proc_symbol(proc);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProc(proc_stmt.name.to_owned());
            Err(err)
        }
    }

    fn create_global_var_symbol(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var_name = &make_stmt.var_name;

        let symbol = self.try_get_symbol_recur(var_name, SymbolKind::Var);

        if symbol.is_none() {
            let index = self.globals_index;

            let var_id = self.create_var_symbol(var_name, None, true, index)?;
            make_stmt.var_id = Some(var_id);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateGlobalVar(make_stmt.var_name.to_owned());
            Err(err)
        }
    }

    fn create_local_var_symbol(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var_name = &make_stmt.var_name;
        let symbol = self.try_get_symbol(var_name, SymbolKind::Var);

        if symbol.is_none() {
            let index = self.proc_locals_index;

            let var_id: u64 = self.create_var_symbol(var_name, None, false, index)?;
            make_stmt.var_id = Some(var_id);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProcLocalVar(var_name.to_owned());
            Err(err)
        }
    }

    fn create_var_symbol(
        &mut self,
        var_name: &str,
        var_type: Option<ExpressionType>,
        is_global: bool,
        index: u64,
    ) -> Result<SymbolId, AstWalkError> {
        let var_id = self.get_next_id();

        let var = Variable {
            id: var_id,
            index,
            global: is_global,
            name: var_name.to_owned(),
            var_type,
        };

        self.sym_table.create_var_symbol(var);

        if is_global {
            self.globals_index += 1;
        } else {
            self.proc_locals_index += 1;
        }

        Ok(var_id)
    }

    fn try_get_symbol_recur(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table.lookup_recur(current_scope_id, name, &kind)
    }

    fn try_get_symbol(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table.lookup(current_scope_id, name, &kind)
    }

    fn start_scope(&mut self) {
        self.sym_table.start_scope();
    }

    fn end_scope(&mut self) {
        self.sym_table.end_scope();
    }

    fn gen_main_proc_symbol(&mut self) {
        let id = self.get_next_id();
        let proc = Procedure::new("__main__", id);

        self.sym_table.create_proc_symbol(proc);
    }

    fn get_next_id(&mut self) -> u64 {
        self.id_generator.get_next_id()
    }
}
