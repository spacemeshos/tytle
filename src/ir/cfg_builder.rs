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
            Statement::Command(cmd) => self.append_cmd(cmd),
            Statement::Expression(_) => self.append_stmt(stmt),
            Statement::Direction(_) => self.append_stmt(stmt),
            Statement::Make(make_stmt) => self.visit_make_stmt(make_stmt),
            Statement::If(if_stmt) => self.visit_if_stmt(if_stmt),
            _ => unimplemented!(),
        }
    }

    fn append_cmd(&mut self, cmd: &Command) {
        let inst = CfgInstruction::Command(cmd.clone());

        let node = self.cfg_graph.current_node_mut();
        node.append_inst(inst);
    }

    fn append_stmt(&mut self, stmt: &Statement) {
        let node = self.cfg_graph.current_node_mut();

        // node.append_stmt(stmt);
    }

    fn visit_make_stmt(&mut self, make_stmt: &MakeStmt) {
        // let var_name = &make_stmt.var;
        //
        // let var_data = match var.var_type {
        //     Some(ExpressionType::Int) => VarData::Int,
        //     Some(ExpressionType::Str) => VarData::Str,
        //     Some(ExpressionType::Bool) => VarData::Bool,
        //     _ => unreachable!(),
        // };
        //
        // let var = if var.global {
        //     let global_var = GlobalVar {
        //         offset: 0,
        //         data: var_data,
        //     };
        //     Var::Global(global_var)
        // } else {
        //     let local_var = LocalVar {
        //         index: 0,
        //         data: var_data,
        //     };
        //     Var::Local(local_var)
        // };
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt) {
        // generate `if_stmt.cond_expr` instructions

        let next_id = self.cfg_graph.next_id;

        // let jmp_stmt = ...
        // self.cfg_graph.current_node_mut().append_stmt(jmp_stmt);

        self.cfg_graph.new_node();
        self.visit_block_stmt(&if_stmt.true_block);

        if if_stmt.false_block.is_some() {
            self.cfg_graph.new_node();
            self.visit_block_stmt(if_stmt.false_block.as_ref().unwrap());
        }
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStatement) {
        for stmt in &block_stmt.stmts {
            self.build_stmt(stmt);
        }
    }
}
