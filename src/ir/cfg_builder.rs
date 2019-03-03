pub use crate::ast::expression::*;
pub use crate::ast::semantic::*;
pub use crate::ast::statement::*;
pub use crate::ast::Ast;
pub use crate::ir::{Instruction, Opcode};

pub struct CfgEdge {
    pub src_node: usize,
    pub dst_node: usize,
}

pub struct CfgNode {
    pub id: usize,
    pub inst: Instruction,
    pub incoming_edges: Vec<CfgEdge>,
    pub outgoing_edges: Vec<CfgEdge>,
}

pub struct CfgGraph {
    //
}

pub struct CfgBuilder<'a, 'b: 'a> {
    cfg_graph: &'a mut CfgGraph,
    sym_visitor: &'a mut SymbolTableVisitor<'b>,
}

impl<'a, 'b> CfgBuilder<'a, 'b> {
    pub fn new(cfg_graph: &'a mut CfgGraph, sym_visitor: &'a mut SymbolTableVisitor<'b>) -> Self {
        Self {
            cfg_graph,
            sym_visitor,
        }
    }

    pub fn build(&mut self, ast: &mut Ast) -> &mut CfgGraph {
        self.walk_ast(ast);

        self.cfg_graph
    }
}

impl<'a, 'b> AstWalker<'a> for CfgBuilder<'a, 'b> {
    fn on_proc_end(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        let proc = self.sym_visitor.lookup_proc(&proc_stmt.name);

        if proc.return_type == ExpressionType::Unit {
            self.emit_ret();
        }

        Ok(())
    }

    fn on_proc_param(&mut self, ctx_proc: &str, proc_param: &mut ProcParam) -> AstWalkResult {
        // TODO: generate a local variable for `proc_param`
        Ok(())
    }

    // expression
    fn on_literal_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let lit_expr = expr.as_lit_expr();

        match lit_expr {
            LiteralExpr::Bool(v) => self.emit_pushb(v.clone()),
            LiteralExpr::Int(v) => self.emit_pushi(v.clone()),
            LiteralExpr::Var(v) => {
                let var = self.sym_visitor.lookup_var(v);

                // TODO
                if var.global {
                }
                else {

                }
            },
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn on_binary_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let (bin_op, _, _) = expr.as_binary_expr();

        let opcode = Opcode::from(bin_op);

        self.emit_op(opcode);

        Ok(())
    }

    fn on_not_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        self.emit_op(Opcode::NOT);

        Ok(())
    }

    fn on_expr_stmt(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        self.emit_pop();
        Ok(())
    }

    fn on_proc_call_expr_end(&mut self, ctx_proc: &str, proc_name: &str) -> AstWalkResult {
        let addr = 100;

        self.emit_jmp(addr);

        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        // TODO: create a new global variable on the heap
        Ok(())
    }

    fn on_if_stmt(&mut self, ctx_proc: &str, if_stmt: &mut IfStmt) -> AstWalkResult {
        // let src_node = if_stmt.true_block.id();
        // let dst_node = if_stmt.false_block.id();

        // let edge = CfgEdge { src_node, dst_node };

        // TODO:
        // 1) generate instructions for `if_stmt.cond_expr`
        // 2) generate `POP`. generate `JMP` to the `else block` (with an empty place for the relative offset)
        // 3) generate instructions for `if_stmt.true_block`
        // 4) generate `POP`. generate `JMP` after the `else block` (with an empty place for the relative offset)
        // 5) generate instructions for `if_stmt.false_block`
        // 6) fill in the offsets in the `JMP` instructions
        Ok(())
    }

    fn on_repeat_stmt(&mut self, ctx_proc: &str, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        // TODO:
        // 2) generate `JUMP_IF_ZERO` instruction after the block
        // 2) generate instructions for `repeat_stmt.block`
        // 3) execute instruction `DEC`
        // 4) generate `JUMMP_IF_NOT_ZERO` instruction to the start of the block
        Ok(())
    }

    fn on_ret_stmt(&mut self, ctx_proc: &str, return_stmt: &mut ReturnStmt) -> AstWalkResult {
        self.emit_ret();
        Ok(())
    }

    fn on_command_stmt(&mut self, ctx_proc: &str, cmd_stmt: &mut CommandStmt) -> AstWalkResult {
        let opcode = match cmd_stmt {
            CommandStmt::Clean => Opcode::CLR,
            CommandStmt::ClearScreen => Opcode::CLS,
            CommandStmt::PenUp => Opcode::PU,
            CommandStmt::PenDown => Opcode::PD,
            CommandStmt::PenErase => Opcode::PE,
            CommandStmt::ShowTurtle => Opcode::ST,
            CommandStmt::HideTurtle => Opcode::HT,
            CommandStmt::Wait => Opcode::WT,
            CommandStmt::Stop => Opcode::STP,
            CommandStmt::XCor => Opcode::GETX,
            CommandStmt::YCor => Opcode::GETY,
            _ => unimplemented!(),
        };

        self.emit_op(opcode);

        Ok(())
    }

    fn on_direct_stmt(&mut self, ctx_proc: &str, direct_stmt: &mut DirectionStmt) -> AstWalkResult {
        let opcode = match direct_stmt.direction {
            Direction::Forward => Opcode::FD,
            Direction::Backward => Opcode::BK,
            Direction::Right => Opcode::RT,
            Direction::Left => Opcode::LT,
            Direction::SetX => Opcode::SX,
            Direction::SetY => Opcode::SY,
        };

        self.emit_op(opcode);

        Ok(())
    }
}

impl<'a, 'b> CfgBuilder<'a, 'b> {
    fn generate_expr(&mut self, expr: &Expression) {
        //
    }

    fn start_block(&mut self) {}

    fn end_block(&mut self) {}

    fn emit_op(&mut self, opcode: Opcode) {
        // self.current_block.emit_op(opcode);
    }

    fn emit_pushi(&mut self, number: usize) {
        //
    }

    fn emit_pushb(&mut self, boolean: bool) {
        //
    }

    fn emit_pushv(&mut self, var: &str) {
        // LOAD var
        // PUSH var
    }

    fn emit_pop(&mut self) {
        self.emit_op(Opcode::POP);
    }

    fn emit_ret(&mut self) {
        //
    }

    fn emit_jmp(&mut self, address: usize) {
        //
    }

    fn emit_jmp_if_zero(&mut self) {}
}
