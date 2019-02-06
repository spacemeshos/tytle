use std::collections::HashMap;

use crate::ast::expression::*;
use crate::ast::semantic::{AstWalker, Program, Scope, Variable};
use crate::ast::statement::*;
use crate::ast::Ast;
use crate::ir::{Instruction, Opcode, Operand};

pub struct BytecodeGenerator {}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&mut self, ast: Ast) -> Vec<Instruction> {
        let root_proc = ast.as_proc_stmt();

        self.emit_code_start();
        self.walk(&root_proc);
        self.emit_code_end();

        vec![]
    }

    fn emit_code_start(&mut self) {
        self.emit(Opcode::SOC);
    }

    fn emit_code_end(&mut self) {
        self.emit(Opcode::EOC);
    }

    fn emit(&mut self, opcode: Opcode) {
        //
    }
}

impl<'a> AstWalker<'a> for BytecodeGenerator {
    fn on_proc_start(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {
        self.on_block_stmt_start(proc);
    }

    fn on_proc_end(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {
        self.on_block_stmt_end(proc);

        // emit: pop each proc param
        // emit: pop the return address
        // emit: jump to the return address
    }

    fn on_proc_param(&mut self, proc: &ProcedureStmt, param: &ProcParam) {
        // create symbol for the param
        // save a unique integer identifier for the param
    }

    fn on_block_stmt_start(&mut self, proc: &ProcedureStmt) {
        // create a new scope and push it to the scopes stack
    }

    fn on_block_stmt_end(&mut self, proc: &ProcedureStmt) {
        // pop current stack out of the scopes stack
    }

    fn on_make_stmt(&mut self, proc: &ProcedureStmt, make_stmt: &MakeStmt) {
        // create a symbol for variable under the current scope
    }

    fn on_command_stmt(&mut self, proc: &ProcedureStmt, cmd: &CommandStmt) {
        // notify the host about the command
    }

    fn on_literal_expr(&mut self, proc: &ProcedureStmt, expr: &LiteralExpr) {
        //  when literal is a number:
        //      push number to the stack machine
        //  when literal is a variable:
        //      get literal symbol reference
        //      push symbol to the stack machine
        //
    }

    fn on_binary_expr_end(&mut self, proc: &ProcedureStmt, binary_op: &BinaryOp) {
        //  when BinaryOp::Add
        //      emit: `+` opcode
        //  when BinaryOp::Mul
        //      emit: `*` opcode
    }

    fn on_proc_call_expr_start(&mut self, proc: &ProcedureStmt, proc_name: &str) {
        // push the `return address` to the stack machine
    }

    fn on_proc_call_expr_end(&mut self, proc: &ProcedureStmt, proc_name: &str) {
        // pop the `return address` from the stack machine
    }

    fn on_proc_param_expr_end(&mut self, proc: &ProcedureStmt, param_expr: &Expression) {
        // the expression result should already reside on the top of the stack machine
    }
}
