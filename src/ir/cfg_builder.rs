pub use crate::ast::expression::*;
pub use crate::ast::semantic::*;
pub use crate::ast::statement::*;
pub use crate::ast::Ast;
pub use crate::ir::*;
use std::collections::HashMap;

pub type NodeId = usize;

pub struct CfgEdge(NodeId, NodeId);

pub struct CfgNode {
    pub id: NodeId,
    pub stmts: Vec<Statement>,
}

impl CfgNode {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            stmts: Vec::new(),
        }
    }

    pub fn append_stmt(&mut self, stmt: &Statement) {
        self.stmts.push(stmt.clone());
    }
}

pub struct CfgGraph {
    pub next_id: NodeId,
    pub nodes: HashMap<NodeId, CfgNode>,
}

impl CfgGraph {
    pub fn new() -> Self {
        let mut graph = Self {
            nodes: HashMap::new(),
            next_id: 0,
        };

        graph.new_node();

        graph
    }

    pub fn current_node_mut(&mut self) -> &mut CfgNode {
        let node_id = self.next_id - 1;
        self.nodes.get_mut(&node_id).unwrap()
    }

    pub fn new_node(&mut self) -> &mut CfgNode {
        let node = CfgNode::new(self.next_id);

        let mut nodes = HashMap::new();
        nodes.insert(node.id, node);

        self.current_node_mut()
    }
}

pub struct CfgBuilder<'a, 'b: 'a> {
    cfg_graph: CfgGraph,
    vars_refs: VarsRefs,
    sym_visitor: &'a mut SymbolTableVisitor<'b>,
}

impl<'a, 'b: 'a> CfgBuilder<'a, 'b> {
    pub fn new(sym_visitor: &'a mut SymbolTableVisitor<'b>) -> Self {
        let mut cfg_graph = CfgGraph::new();
        let mut vars_refs = VarsRefs::new();

        Self {
            cfg_graph,
            vars_refs,
            sym_visitor,
        }
    }

    pub fn build(mut self, ast: &Ast) -> (CfgGraph, VarsRefs) {
        for stmt in &ast.statements {
            self.build_stmt(stmt);
        }

        (self.cfg_graph, self.vars_refs)
    }

    fn build_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::NOP | Statement::EOF => return,
            Statement::Command(_) => self.append_stmt(stmt),
            Statement::Expression(_) => self.append_stmt(stmt),
            Statement::Direction(_) => self.append_stmt(stmt),
            Statement::Make(make_stmt) => self.visit_make_stmt(make_stmt),
            Statement::If(if_stmt) => self.visit_if_stmt(if_stmt),
            _ => unimplemented!(),
        }
    }

    fn append_stmt(&mut self, stmt: &Statement) {
        let node = self.cfg_graph.current_node_mut();

        node.append_stmt(stmt);
    }

    fn visit_make_stmt(&mut self, make_stmt: &MakeStmt) {
        let var_name = &make_stmt.var;
        let var: &Variable = self.sym_visitor.lookup_var(var_name);

        let var_data = match var.var_type {
            Some(ExpressionType::Int) => VarData::Int,
            Some(ExpressionType::Str) => VarData::Str,
            Some(ExpressionType::Bool) => VarData::Bool,
            _ => unreachable!(),
        };

        let var = if var.global {
            let global_var = GlobalVar {
                offset: 0,
                data: var_data,
            };
            Var::Global(global_var)
        } else {
            let local_var = LocalVar {
                index: 0,
                data: var_data,
            };
            Var::Local(local_var)
        };

        self.vars_refs.store_var(var);
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
