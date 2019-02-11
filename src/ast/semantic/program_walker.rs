use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    VariableNoDeclaration(String),
}

struct ProgramWalker {
    sym_table: SymbolTable,
    global_ref: u64,
}

type ProgramResult<'a> = Result<&'a SymbolTable, ProgramError>;

impl ProgramWalker {
    fn walk_ast(&mut self, ast: &Ast) -> ProgramResult {
        self.start_scope();
        self.prewalk_procs_and_globals(&ast);
        self.end_scope();

        Ok(&self.sym_table)
    }
}

impl<'a> AstWalker<'a> for ProgramWalker {
    fn on_block_stmt_start(&mut self, _block_stmt: &BlockStatement) {
        self.start_scope();
    }

    fn on_block_stmt_end(&mut self, _block_stmt: &BlockStatement) {
        self.end_scope();
    }

    fn on_proc_param(&mut self, proc_stmt: &ProcedureStmt, param: &ProcParam) {}

    fn on_literal_expr(&mut self, expr: &LiteralExpr) -> Option<ExpressionType> {
        match expr {
            LiteralExpr::Bool(b) => Some(ExpressionType::Bool),
            LiteralExpr::Int(n) => Some(ExpressionType::Int),
            LiteralExpr::Str(ref s) => Some(ExpressionType::Str),
            LiteralExpr::Var(ref v) => {
                let var = self.get_var_symbol(v);
                Self::primitive_to_expr_type(&var.resolved_type)
            }
        }
    }

    fn resolve_proc_call_expr(&self, proc_name: &str) -> Option<ExpressionType> {
        let proc = self.get_proc_symbol(proc_name);
        Self::primitive_to_expr_type(&proc.return_type)
    }

    fn resolve_binary_expr(
        &self,
        binary_op: &BinaryOp,
        lexpr_type: Option<ExpressionType>,
        rexpr_type: Option<ExpressionType>,
    ) -> Option<ExpressionType> {
        if lexpr_type.is_none() || rexpr_type.is_none() {
            return None;
        }

        if lexpr_type == rexpr_type {
            match binary_op {
                BinaryOp::Add | BinaryOp::Mul => lexpr_type,
                _ => unimplemented!(),
            }
        } else {
            panic!(
                "type mismatch between `{:?}` and `{:?}` expressions",
                lexpr_type.unwrap(),
                rexpr_type.unwrap()
            );
        }
    }
}

impl ProgramWalker {
    fn new() -> Self {
        Self {
            sym_table: SymbolTable::new(),
            global_ref: 0,
        }
    }

    fn get_var_symbol(&self, var_name: &str) -> &Variable {
        let symbol = self.try_get_symbol(var_name);

        if symbol.is_some() {
            if let Symbol::Var(ref var) = symbol.unwrap() {
                var
            } else {
                panic!("expected a variable for symbol {}", var_name);
            }
        } else {
            panic!("variable declaration is missing for {}", var_name);
        }
    }

    fn get_proc_symbol(&self, proc_name: &str) -> &Procedure {
        let symbol = self.try_get_symbol(proc_name);

        if symbol.is_some() {
            if let Symbol::Proc(ref proc) = symbol.unwrap() {
                proc
            } else {
                panic!("expected a variable for symbol {}", proc_name);
            }
        } else {
            panic!("procedure declaration missing for {}", proc_name);
        }
    }

    fn try_get_symbol(&self, name: &str) -> Option<&Symbol> {
        let current_scope_id = self.sym_table.get_current_scope_id();

        self.sym_table.recursive_lookup_sym(current_scope_id, name)
    }

    fn primitive_to_expr_type(pt: &Option<PrimitiveType>) -> Option<ExpressionType> {
        match pt {
            Some(PrimitiveType::Int) => Some(ExpressionType::Int),
            Some(PrimitiveType::Str) => Some(ExpressionType::Str),
            None => None,
        }
    }

    // looking for `ast` statements of type `ProcedureStmt` or `MakeStmt`
    // when encountering a `ProcedureStmt` adding a symbol for the Procedure signature,
    // and generating a symbol for global variables for `MakeStmt` statements.
    fn prewalk_procs_and_globals(&mut self, ast: &Ast) {
        let mut procs_ref = 0;

        for stmt in &ast.statements {
            match stmt {
                Statement::Procedure(ref proc_stmt) => {
                    let proc = Procedure {
                        name: proc_stmt.name.to_owned(),
                        reference: Some(procs_ref),
                        params_types: None,
                        return_type: None,
                    };

                    procs_ref += 1;

                    self.sym_table.create_proc_symbol(proc);
                }
                Statement::Make(ref make_stmt) => self.create_global_var_symbol(&make_stmt),
                _ => continue,
            }
        }
    }

    fn create_global_var_symbol(&mut self, make_stmt: &MakeStmt) {
        let var = Variable {
            global: true,
            name: make_stmt.var.to_owned(),
            reference: Some(self.global_ref),
            resolved_type: None,
        };

        self.sym_table.create_var_symbol(var);

        self.global_ref += 1;
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

        let mut walker = ProgramWalker::new();

        let sym_table = walker.walk_ast(&ast);
        dbg!(sym_table);
    }
}
