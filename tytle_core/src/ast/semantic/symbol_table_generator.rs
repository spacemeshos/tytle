use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

pub struct SymbolTableGenerator {
    env: Environment,
}

type EnvironmentResult = Result<Environment, AstWalkError>;

impl AstWalker for SymbolTableGenerator {
    fn on_make_global_stmt(&mut self, _ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        if self.env.symbol_table.is_inner_scope() {
            let err = AstWalkError::ProcNotAllowedToDeclareGlobals(make_stmt.var_name.to_string());
            Err(err)
        } else {
            Ok(())
        }
    }

    fn on_make_local_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.create_local_var_symbol(ctx_proc, make_stmt)
    }

    fn on_make_assign_stmt(&mut self, _ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var = self.get_var_symbol(&make_stmt.var_name)?;

        make_stmt.var_id = Some(var.id);

        Ok(())
    }

    fn on_proc_call_expr(&mut self, _ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let (proc_name, _proc_args, proc_id) = expr.as_proc_call_expr_mut();

        let proc = self.env.symbol_table.get_proc_by_name(&proc_name);

        proc_id.replace(proc.id);

        Ok(())
    }

    fn on_proc_param(&mut self, ctx_proc: &str, proc_param: &mut ProcParam) -> AstWalkResult {
        let symbol = self.try_get_symbol(&proc_param.param_name, SymbolKind::Var);

        if symbol.is_none() {
            let param_type = ExpressionType::from(proc_param.param_type.as_str());

            self.create_var_symbol(
                ctx_proc,
                &proc_param.param_name,
                Some(param_type),
                false,
                true,
            )?;

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProcParam(
                ctx_proc.to_string(),
                proc_param.param_name.to_string(),
            );
            Err(err)
        }
    }

    fn on_proc_start(&mut self, _ctx_proc: &str, _proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_proc_end(&mut self, _ctx_proc: &str, _proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.end_scope();
        Ok(())
    }

    fn on_block_stmt_start(
        &mut self,
        _ctx_proc: &str,
        _block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        self.start_scope();
        Ok(())
    }

    fn on_block_stmt_end(
        &mut self,
        _ctx_proc: &str,
        _block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        self.end_scope();
        Ok(())
    }

    fn on_literal_expr(&mut self, _ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let lit_expr: &mut LiteralExpr = expr.as_lit_expr_mut();

        match lit_expr {
            LiteralExpr::Var(var_name, var_id) => {
                let var = self.get_var_symbol(var_name);

                if var.is_ok() {
                    let var = var.unwrap();

                    var_id.replace(var.id);
                } else {
                    // TODO: return error if variable isn't found
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
            env: Environment::new(),
        }
    }

    pub fn generate(mut self, ast: &mut Ast) -> EnvironmentResult {
        self.generate_main_symbol();
        self.prewalk_ast(ast)?;
        self.walk_ast(ast)?;

        Ok(self.env)
    }

    pub fn prewalk_ast(&mut self, ast: &mut Ast) -> AstWalkResult {
        for stmt in &mut ast.statements {
            match stmt {
                Statement::Make(make_stmt) => match make_stmt.kind {
                    MakeStmtKind::Global => {
                        // only `__main__` can declare globals
                        self.create_global_var_symbol("__main__", make_stmt)?;
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

    fn create_proc_symbol(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        let symbol = self.try_get_symbol_recur(&proc_stmt.name, SymbolKind::Proc);

        if symbol.is_none() {
            let ret_type_str = proc_stmt.return_type.clone();
            let return_type = ExpressionType::from(ret_type_str.as_str());

            let params_types = proc_stmt
                .params
                .iter()
                .map(|param| ExpressionType::from(param.param_type.as_str()))
                .collect::<Vec<ExpressionType>>();

            let proc_id = self
                .env
                .create_proc(&proc_stmt.name, params_types, return_type);

            proc_stmt.id = Some(proc_id);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProc(proc_stmt.name.to_owned());
            Err(err)
        }
    }

    fn create_global_var_symbol(
        &mut self,
        ctx_proc: &str,
        make_stmt: &mut MakeStmt,
    ) -> AstWalkResult {
        let var_name = &make_stmt.var_name;

        let symbol = self.try_get_symbol_recur(var_name, SymbolKind::Var);

        if symbol.is_none() {
            let var_id = self.create_var_symbol(ctx_proc, var_name, None, true, false)?;

            make_stmt.var_id = Some(var_id);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateGlobalVar(make_stmt.var_name.to_owned());
            Err(err)
        }
    }

    fn create_local_var_symbol(
        &mut self,
        ctx_proc: &str,
        make_stmt: &mut MakeStmt,
    ) -> AstWalkResult {
        let var_name = &make_stmt.var_name;
        let symbol = self.try_get_symbol(var_name, SymbolKind::Var);

        if symbol.is_none() {
            let var_id = self.create_var_symbol(ctx_proc, var_name, None, false, false)?;

            make_stmt.var_id = Some(var_id);

            Ok(())
        } else {
            let err = AstWalkError::DuplicateProcLocalVar(var_name.to_owned());
            Err(err)
        }
    }

    fn create_var_symbol(
        &mut self,
        ctx_proc: &str,
        var_name: &str,
        var_type: Option<ExpressionType>,
        is_global: bool,
        is_param: bool,
    ) -> Result<SymbolId, AstWalkError> {
        let var_id = self.env.id_generator.get_next_id();

        if is_global {
            self.env.create_global_var(var_id, var_name, var_type);
        } else {
            let proc = self.env.symbol_table.get_proc_by_name(ctx_proc);

            self.env
                .create_local_var(proc.id, var_id, var_name, var_type, is_param);
        }

        Ok(var_id)
    }

    fn try_get_symbol_recur(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.env.symbol_table.get_current_scope_id();

        self.env
            .symbol_table
            .lookup_recur(current_scope_id, name, &kind)
    }

    fn try_get_symbol(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
        let current_scope_id = self.env.symbol_table.get_current_scope_id();

        self.env.symbol_table.lookup(current_scope_id, name, &kind)
    }

    fn start_scope(&mut self) {
        self.env.symbol_table.start_scope();
    }

    fn end_scope(&mut self) {
        self.env.symbol_table.end_scope();
    }

    fn generate_main_symbol(&mut self) {
        self.env
            .create_proc("__main__", vec![], ExpressionType::Unit);
    }
}
