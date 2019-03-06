pub use crate::ast::expression::*;
pub use crate::ast::semantic::*;
pub use crate::ast::statement::*;
pub use crate::ast::Ast;
pub use crate::ir::*;

pub struct CfgBuilder {
    cfg_graph: CfgGraph,
}

impl CfgBuilder {
    pub fn new() -> Self {
        let mut cfg_graph = CfgGraph::new();

        Self { cfg_graph }
    }

    pub fn build(mut self, ast: &Ast) -> CfgGraph {
        for stmt in &ast.statements {
            let node_id = self.cfg_graph.get_current_id();

            self.build_stmt(node_id, stmt);
        }

        self.cfg_graph
    }

    fn build_stmt(&mut self, node_id: CfgNodeId, stmt: &Statement) {
        match stmt {
            Statement::NOP | Statement::EOF => return,
            Statement::Command(cmd) => self.build_cmd(node_id, cmd),
            Statement::Direction(direct_stmt) => self.build_direct(node_id, direct_stmt),
            Statement::Expression(expr) => self.build_expr(node_id, expr),
            Statement::Make(make_stmt) => self.build_make(node_id, make_stmt),
            Statement::If(if_stmt) => self.build_if(node_id, if_stmt),
            _ => unimplemented!(),
        }
    }

    fn build_cmd(&mut self, node_id: CfgNodeId, cmd: &Command) {
        let inst = CfgInstruction::Command(cmd.clone());

        self.append_inst(node_id, inst);
    }

    fn build_direct(&mut self, node_id: CfgNodeId, direct_stmt: &DirectionStmt) {
        self.build_expr(node_id, &direct_stmt.expr);

        let direct = direct_stmt.direction.clone();
        let inst = CfgInstruction::Direction(direct);

        self.append_inst(node_id, inst);
    }

    fn build_make(&mut self, node_id: CfgNodeId, make_stmt: &MakeStmt) {
        self.build_expr(node_id, &make_stmt.expr);

        let var_id = make_stmt.var_id.unwrap();
        let inst = CfgInstruction::Store(var_id);

        self.append_inst(node_id, inst);
    }

    fn build_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        match expr.expr_ast {
            ExpressionAst::Literal(_) => self.build_lit_expr(node_id, expr),
            ExpressionAst::Not(_) => self.build_not_expr(node_id, expr),
            ExpressionAst::Binary(..) => self.build_bin_expr(node_id, expr),
            ExpressionAst::Parentheses(_) => self.build_parentheses_expr(node_id, expr),
            ExpressionAst::ProcCall(..) => unimplemented!(),
        }
    }

    fn build_proc_call_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        //
    }

    fn build_parentheses_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        let expr = expr.as_parentheses_expr();
        self.build_expr(node_id, expr);
    }

    fn build_bin_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        let (bin_op, lexpr, rexpr) = expr.as_binary_expr();

        self.build_expr(node_id, lexpr);
        self.build_expr(node_id, rexpr);

        let inst = match bin_op {
            BinaryOp::Add => CfgInstruction::Add,
            BinaryOp::Mul => CfgInstruction::Mul,
            BinaryOp::And => CfgInstruction::And,
            BinaryOp::Or => CfgInstruction::Or,
            BinaryOp::GT => CfgInstruction::GT,
            BinaryOp::LT => CfgInstruction::LT,
        };

        self.append_inst(node_id, inst);
    }

    fn build_not_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        let expr = expr.as_not_expr();

        self.build_expr(node_id, expr);
        self.append_inst(node_id, CfgInstruction::Not);
    }

    fn build_lit_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        let expr = expr.as_lit_expr();

        match expr {
            LiteralExpr::Bool(v) => self.append_bool_lit(node_id, *v),
            LiteralExpr::Int(v) => self.append_int_lit(node_id, *v),
            LiteralExpr::Str(v) => self.append_str_lit(node_id, v),
            LiteralExpr::Var(_, var_id) => self.append_var_lit(node_id, var_id),
        }
    }

    fn append_bool_lit(&mut self, node_id: CfgNodeId, lit: bool) {
        self.append_inst(node_id, CfgInstruction::Bool(lit));
    }

    fn append_int_lit(&mut self, node_id: CfgNodeId, lit: usize) {
        self.append_inst(node_id, CfgInstruction::Int(lit));
    }

    fn append_str_lit(&mut self, node_id: CfgNodeId, lit: &str) {
        self.append_inst(node_id, CfgInstruction::Str(lit.to_string()));
    }

    fn append_var_lit(&mut self, node_id: CfgNodeId, lit: &Option<SymbolId>) {
        let var_id = lit.as_ref().unwrap();
        let inst = CfgInstruction::Load(*var_id);

        self.append_inst(node_id, inst);
    }

    fn build_if(&mut self, node_id: CfgNodeId, if_stmt: &IfStmt) {
        // 1) let's mark current CFG node as `CURRENT_NODE_ID`
        // 2) generate instructions for `if-stmt` conditional expression (within `CURRENT_NODE_ID` node)
        // 3) create a new empty CFG node. let's mark its node id as `TRUE_NODE_ID`
        // 4) create a new empty CFG node. let's mark its node id as `NEXT_NODE_ID`
        // 5) append `jump-if-true TRUE_NODE_ID` instruction to node `CURRENT_NODE_ID`
        // 6) generate instructions for `true-block` starting at node-context `TRUE_NODE_ID`
        // 7) append `jump NEXT_NODE_ID` instruction to the last node of (6)
        // 8) if `if-stmt` has `else-block`:
        //    8.1) create a new empty CFG node. let's mark its node id as `FALSE_NODE_ID`
        //    8.2) append `jump FALSE_NODE_ID` instruction to node `CURRENT_NODE_ID`
        //    8.3) generate instructions for `false-block` starting at node-context `FALSE_NODE_ID`
        //    8.4) append `jump NEXT_NODE_ID` instruction to the last node of (8.3)
        // 9) if `if-stmt` has no `else-block`:
        //    9.1) append `jump NEXT_NODE_ID` instruction to node `CURRENT_NODE_ID`

        self.build_expr(node_id, &if_stmt.cond_expr);

        let true_node_id = self.cfg_graph.new_node();
        let next_node_id = self.cfg_graph.new_node();
        self.append_jump_true_inst(node_id, true_node_id);

        self.build_block(true_node_id, &if_stmt.true_block);
        let true_block_last_node_id = self.cfg_graph.get_current_id();
        self.append_jump_inst(true_block_last_node_id, next_node_id);

        if if_stmt.false_block.is_some() {
            let false_node_id = self.cfg_graph.new_node();
            self.append_jump_inst(node_id, false_node_id);
            self.build_block(false_node_id, if_stmt.false_block.as_ref().unwrap());

            let false_block_last_node_id = self.cfg_graph.get_current_id();
            self.append_jump_inst(false_block_last_node_id, next_node_id);
        }
        else {
            self.append_jump_inst(node_id, next_node_id);
        }
    }

    fn build_block(&mut self, node_id: CfgNodeId, block_stmt: &BlockStatement) {
        for stmt in &block_stmt.stmts {
            self.build_stmt(node_id, stmt);
        }
    }

    fn append_jump_true_inst(&mut self, node_id: CfgNodeId, dst_id: CfgNodeId) {
        let inst = CfgInstruction::JumpIfTrue(dst_id);
        self.append_inst(node_id, inst);
    }

    fn append_inst(&mut self, node_id: CfgNodeId, inst: CfgInstruction) {
        let node = self.cfg_graph.get_node_mut(node_id);

        node.append_inst(inst);
    }

    fn append_jump_inst(&mut self, node_id: CfgNodeId, dst_id: CfgNodeId) {
        let inst = CfgInstruction::Jump(dst_id);
        self.append_inst(node_id, inst);
    }
}
