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

    fn build_stmt(&mut self, node_id: CfgNodeId, stmt: &Statement) -> CfgNodeId {
        match stmt {
            Statement::NOP | Statement::EOF => node_id,
            Statement::Command(cmd) => self.build_cmd(node_id, cmd),
            Statement::Direction(direct_stmt) => self.build_direct(node_id, direct_stmt),
            Statement::Expression(expr) => self.build_expr(node_id, expr),
            Statement::Make(make_stmt) => self.build_make(node_id, make_stmt),
            Statement::If(if_stmt) => self.build_if(node_id, if_stmt),
            _ => unimplemented!(),
        }
    }

    fn build_cmd(&mut self, node_id: CfgNodeId, cmd: &Command) -> CfgNodeId {
        let inst = CfgInstruction::Command(cmd.clone());

        self.append_inst(node_id, inst);

        node_id
    }

    fn build_direct(&mut self, node_id: CfgNodeId, direct_stmt: &DirectionStmt) -> CfgNodeId {
        self.build_expr(node_id, &direct_stmt.expr);

        let direct = direct_stmt.direction.clone();
        let inst = CfgInstruction::Direction(direct);

        self.append_inst(node_id, inst);

        node_id
    }

    fn build_make(&mut self, node_id: CfgNodeId, make_stmt: &MakeStmt) -> CfgNodeId {
        self.build_expr(node_id, &make_stmt.expr);

        let var_id = make_stmt.var_id.unwrap();
        let inst = CfgInstruction::Store(var_id);

        self.append_inst(node_id, inst);

        node_id
    }

    fn build_expr(&mut self, node_id: CfgNodeId, expr: &Expression) -> CfgNodeId {
        match expr.expr_ast {
            ExpressionAst::Literal(_) => self.build_lit_expr(node_id, expr),
            ExpressionAst::Not(_) => self.build_not_expr(node_id, expr),
            ExpressionAst::Binary(..) => self.build_bin_expr(node_id, expr),
            ExpressionAst::Parentheses(_) => self.build_parentheses_expr(node_id, expr),
            ExpressionAst::ProcCall(..) => unimplemented!(),
        }

        node_id
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

    fn build_if(&mut self, node_id: CfgNodeId, if_stmt: &IfStmt) -> CfgNodeId {
        // 1) let's mark current CFG node as `CURRENT_NODE_ID` (the `node_id` paramter)
        //    this node is assumed to be empty
        // 2) generate expression-instructions for `if-stmt` conditional-expression (within `CURRENT_NODE_ID` node)
        // 3) create a new empty CFG node. let's mark its node id as `TRUE_NODE_ID`
        // 4) generate statement-instructions for `if-stmt` `true-block` (within `TRUE_NODE_ID` node)
        //    the CFG generation will return `LAST_TRUE_BLOCK_NODE_ID` node_id
        // 5) add edge `CURRENT_NODE_ID` --> `TRUE_NODE_ID`
        // 6) if `if-stmt` has `else-block`:
        //      6.1) create a new empty CFG node. let's mark its node id as `FALSE_NODE_ID`
        //      6.2) generate statement-instructions for `false-block` (within `FALSE_NODE_ID` node)
        //           the CFG generation will return `LAST_FALSE_BLOCK_NODE_ID` node_id
        //      6.3) add edge `CURRENT_NODE_ID` --> `FALSE_NODE_ID`
        // 7) create a new empty CFG node. let's mark its node id as `AFTER_NODE_ID`
        // 8) add edge `LAST_TRUE_BLOCK_NODE_ID` --> `AFTER_NODE_ID`
        // 9) if `if-stmt` has `else-block`:
        //    9.1) add edge `LAST_TRUE_BLOCK_NODE_ID` --> `AFTER_NODE_ID`
        // 10) return `AFTER_NODE_ID` node_id (empty CFG node to be used for the next statement)

        self.build_expr(node_id, &if_stmt.cond_expr);

        let true_node_id = self.cfg_graph.new_node();
        let last_true_block_node_id = self.build_block(true_node_id, &if_stmt.true_block);
        self.add_edge(node_id, true_node_id, CfgJumpType::WhenTrue);

        let mut last_false_block_node_id = None;

        if if_stmt.false_block.is_some() {
            let false_node_id = self.cfg_graph.new_node();

            let last_node_id =
                self.build_block(false_node_id, if_stmt.false_block.as_ref().unwrap());
            last_false_block_node_id = Some(last_node_id);

            self.add_edge(node_id, false_node_id, CfgJumpType::Fallback);
        }

        let after_node_id = self.cfg_graph.new_node();
        self.add_edge(last_true_block_node_id, after_node_id, CfgJumpType::Always);

        if if_stmt.false_block.is_some() {
            self.add_edge(
                last_false_block_node_id.unwrap(),
                after_node_id,
                CfgJumpType::Always,
            );
        }

        after_node_id
    }

    fn build_block(&mut self, node_id: CfgNodeId, block_stmt: &BlockStatement) -> CfgNodeId {
        let mut last_node_id = node_id;

        for stmt in &block_stmt.stmts {
            last_node_id = self.build_stmt(node_id, stmt);
        }

        last_node_id
    }

    fn append_inst(&mut self, node_id: CfgNodeId, inst: CfgInstruction) {
        let node = self.cfg_graph.get_node_mut(node_id);

        node.append_inst(inst);
    }

    fn add_edge(&mut self, src_node_id: CfgNodeId, dst_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        let mut src_node = self.cfg_graph.get_node_mut(src_node_id);
        src_node.add_outgoing_edge(dst_node_id, jmp_type);

        let mut dst_node = self.cfg_graph.get_node_mut(dst_node_id);
        dst_node.add_incoming_edge(src_node_id, jmp_type);
    }
}
