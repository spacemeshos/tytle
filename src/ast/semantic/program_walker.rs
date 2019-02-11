use std::collections::HashMap;

use crate::ast::semantic::{AstWalker, Procedure, Program, Scope, ScopeId, SymbolTable, Variable};
use crate::ast::statement::{BlockStatement, ProcedureStmt};
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

use crate::parser::{Parser, TytleParser};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum ProgramError {
    VariableNoDeclaration(String),
}

struct ProgramWalker {
    sym_table: SymbolTable,
    current_scope_id: ScopeId,
    global_ref: u64,
}

type ProgramResult<'a> = Result<&'a SymbolTable, ProgramError>;

impl ProgramWalker {
    fn walk_ast(&mut self, ast: &Ast) -> ProgramResult {
        self.scope_start();
        self.prewalk_procs_and_globals(&ast);
        self.scope_end();

        Ok(&self.sym_table)
    }
}

impl<'a> AstWalker<'a> for ProgramWalker {
    fn on_block_stmt_start(&mut self, _block_stmt: &BlockStatement) {
        self.scope_start();
    }

    fn on_block_stmt_end(&mut self, _block_stmt: &BlockStatement) {
        self.scope_end();
    }

    fn on_proc_param(&mut self, proc_stmt: &ProcedureStmt, param: &ProcParam) {}

    fn on_literal_expr(&mut self, expr: &LiteralExpr) {
        match expr {
            LiteralExpr::Int(n) => self.gen_const_int_symbol(*n),
            LiteralExpr::Str(ref s) => unimplemented!("not supported yet..."),
            LiteralExpr::Var(ref v) => self.ensure_var_symbol(v),
        }
    }
}

impl ProgramWalker {
    fn new() -> Self {
        Self {
            sym_table: SymbolTable::new(),
            current_scope_id: 0,
            global_ref: 0,
        }
    }

    fn ensure_var_symbol(&mut self, var_ref: &str) {
        let var_sym = self
            .sym_table
            .recursive_lookup_var(self.current_scope_id, var_ref);

        if var_sym.is_none() {
            panic!("variable declaration is missing for {}", var_ref);
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
                Statement::Make(ref make_stmt) => self.gen_global_var(&make_stmt),
                _ => continue,
            }
        }
    }

    fn gen_global_var(&mut self, make_stmt: &MakeStmt) {
        let var = Variable {
            global: true,
            name: make_stmt.var.to_owned(),
            reference: Some(self.global_ref),
            resolved_type: None,
        };

        self.sym_table.create_var_symbol(var);

        self.global_ref += 1;
    }

    fn gen_const_int_symbol(&mut self, n: usize) {
        //
    }

    fn scope_start(&mut self) {
        self.sym_table.add_scope();
        self.current_scope_id += 1;
    }

    fn scope_end(&mut self) {
        self.current_scope_id -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn prewalk_make_stmt() {
    //     let ast = TytleParser.parse("MAKE \"A=20").unwrap();
    //
    //     let mut walker = ProgramWalker::new();
    //
    //     let sym_table = walker.walk_ast(&ast);
    //     dbg!(sym_table);
    // }
}
