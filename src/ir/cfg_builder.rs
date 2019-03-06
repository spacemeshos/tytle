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
            self.build_stmt(stmt);
        }

        self.cfg_graph
    }

    fn build_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::NOP | Statement::EOF => return,
            Statement::Command(cmd) => self.build_cmd(cmd),
            Statement::Direction(direct_stmt) => self.build_direct(direct_stmt),
            Statement::Expression(expr) => self.build_expr(expr),
            Statement::Make(make_stmt) => self.build_make(make_stmt),
            Statement::If(if_stmt) => self.build_if(if_stmt),
            _ => unimplemented!(),
        }
    }

    fn build_cmd(&mut self, cmd: &Command) {
        let inst = CfgInstruction::Command(cmd.clone());

        self.append_inst(inst);
    }

    fn build_direct(&mut self, direct_stmt: &DirectionStmt) {
        self.build_expr(&direct_stmt.expr);

        let direct = direct_stmt.direction.clone();
        let inst = CfgInstruction::Direction(direct);

        self.append_inst(inst);
    }

    fn build_make(&mut self, make_stmt: &MakeStmt) {
        self.build_expr(&make_stmt.expr);

        let var_id = make_stmt.var_id.unwrap();
        let inst = CfgInstruction::Store(var_id);

        self.append_inst(inst);
    }

    fn build_expr(&mut self, expr: &Expression) {
        match expr.expr_ast {
            ExpressionAst::Literal(_) => self.build_lit_expr(expr),
            ExpressionAst::Not(_) => self.build_not_expr(expr),
            ExpressionAst::Binary(..) => self.build_bin_expr(expr),
            ExpressionAst::Parentheses(_) => self.build_parentheses_expr(expr),
            ExpressionAst::ProcCall(..) => unimplemented!(),
        }
    }

    fn build_proc_call_expr(&mut self, expr: &Expression) {
        //
    }

    fn build_parentheses_expr(&mut self, expr: &Expression) {
        let expr = expr.as_parentheses_expr();
        self.build_expr(expr);
    }

    fn build_bin_expr(&mut self, expr: &Expression) {
        let (bin_op, lexpr, rexpr) = expr.as_binary_expr();

        self.build_expr(lexpr);
        self.build_expr(rexpr);

        let inst = match bin_op {
            BinaryOp::Add => CfgInstruction::Add,
            BinaryOp::Mul => CfgInstruction::Mul,
            BinaryOp::And => CfgInstruction::And,
            BinaryOp::Or => CfgInstruction::Or,
            BinaryOp::GT => CfgInstruction::GT,
            BinaryOp::LT => CfgInstruction::LT,
        };

        self.append_inst(inst);
    }

    fn build_not_expr(&mut self, expr: &Expression) {
        let expr = expr.as_not_expr();

        self.build_expr(expr);
        self.append_inst(CfgInstruction::Not);
    }

    fn build_lit_expr(&mut self, expr: &Expression) {
        let expr = expr.as_lit_expr();

        match expr {
            LiteralExpr::Bool(v) => self.append_bool_lit(*v),
            LiteralExpr::Int(v) => self.append_int_lit(*v),
            LiteralExpr::Str(v) => self.append_str_lit(v),
            LiteralExpr::Var(_, var_id) => self.append_var_lit(var_id),
        }
    }

    fn append_bool_lit(&mut self, lit: bool) {
        self.append_inst(CfgInstruction::Bool(lit));
    }

    fn append_int_lit(&mut self, lit: usize) {
        self.append_inst(CfgInstruction::Int(lit));
    }

    fn append_str_lit(&mut self, lit: &str) {
        self.append_inst(CfgInstruction::Str(lit.to_string()));
    }

    fn append_var_lit(&mut self, lit: &Option<SymbolId>) {
        let var_id = lit.as_ref().unwrap();
        let inst = CfgInstruction::Load(*var_id);

        self.append_inst(inst);
    }

    fn build_if(&mut self, if_stmt: &IfStmt) {}

    fn append_inst(&mut self, inst: CfgInstruction) {
        let node = self.cfg_graph.current_node_mut();

        node.append_inst(inst);
    }
}
