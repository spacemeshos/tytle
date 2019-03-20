pub use crate::ast::{expression::*, semantic::*, statement::*, Ast};
pub use crate::ir::*;
pub use std::collections::HashMap;

pub struct CfgBuilder<'env> {
    cfg_graph: CfgGraph,
    env: &'env mut Environment,
    proc_jmp_table: HashMap<u64, CfgProc>,
    current_proc_id: u64,
}

impl<'env> CfgBuilder<'env> {
    pub fn new(env: &'env mut Environment) -> Self {
        let mut cfg_graph = CfgGraph::new();

        let main_proc = env.symbol_table.get_proc_by_name("__main__");

        Self {
            current_proc_id: main_proc.id,
            cfg_graph,
            env,
            proc_jmp_table: HashMap::new(),
        }
    }

    pub fn build(mut self, ast: &Ast) -> CfgObject {
        let mut node_id = self.cfg_graph.get_entry_node_id();

        for stmt in &ast.statements {
            node_id = self.build_stmt(node_id, stmt);
        }

        // TODO: fix the orphans deletions
        // self.cfg_graph.compact();

        let jmp_table: HashMap<CfgNodeId, u64> = self
            .proc_jmp_table
            .iter()
            .map(|(proc_id, cfg_proc)| (cfg_proc.node_id, *proc_id))
            .collect();

        CfgObject {
            graph: self.cfg_graph,
            jmp_table,
        }
    }

    fn build_stmt(&mut self, node_id: CfgNodeId, stmt: &Statement) -> CfgNodeId {
        match stmt {
            Statement::NOP | Statement::EOF => node_id,
            Statement::Command(cmd) => self.build_cmd(node_id, cmd),
            Statement::Direction(direct_stmt) => self.build_direct(node_id, direct_stmt),
            Statement::Expression(expr) => self.build_expr(node_id, expr),
            Statement::Make(make_stmt) => self.build_make(node_id, make_stmt),
            Statement::If(if_stmt) => self.build_if(node_id, if_stmt),
            Statement::Repeat(repeat_stmt) => self.build_repeat(node_id, repeat_stmt),
            Statement::Procedure(proc_stmt) => self.build_proc(node_id, proc_stmt),
            Statement::Return(return_stmt) => self.build_return(node_id, return_stmt),
        }
    }

    fn build_return(&mut self, node_id: CfgNodeId, return_stmt: &ReturnStmt) -> CfgNodeId {
        if return_stmt.expr.is_some() {
            let expr: &Expression = return_stmt.expr.as_ref().unwrap();
            self.build_expr(node_id, expr);
        }

        let node = self.cfg_graph.get_node_mut(node_id);
        node.append_inst(CfgInstruction::Return);

        node_id
    }

    fn build_proc(&mut self, node_id: CfgNodeId, proc_stmt: &ProcedureStmt) -> CfgNodeId {
        let proc_id = proc_stmt.id.unwrap();
        let cfg_proc = self.proc_jmp_table.get(&proc_id);

        self.current_proc_id = proc_id;

        let mut proc_node_id;

        if cfg_proc.is_some() {
            let cfg_proc = cfg_proc.unwrap();

            if cfg_proc.built == true {
                // we've already built the CFG for the procedure
                // so we just return the start node id
                return cfg_proc.node_id;
            }

            // we have allocated already the proc start node
            // (even though we didn't build the proc instructions yet)
            proc_node_id = cfg_proc.node_id;
        } else {
            // there is no proc CFG node, so we'll allocate one
            proc_node_id = self.cfg_graph.new_node();

            // we explicitly save immediately the cfg proc in order
            // to support recursive procedures

            let cfg_proc = CfgProc {
                node_id: proc_node_id,
                proc_id,
                built: false,
            };
            self.proc_jmp_table.insert(proc_id, cfg_proc);
        }

        self.build_block(proc_node_id, &proc_stmt.block);

        // marking the cfg proc as built
        let cfg_proc = CfgProc {
            node_id: proc_node_id,
            proc_id,
            built: true,
        };
        self.proc_jmp_table.insert(proc_id, cfg_proc);

        // the empty CFG node `node_id` will be used in the next non-procedure statement
        node_id
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
        let expr = &make_stmt.expr;
        let var_id = make_stmt.var_id.unwrap();

        self.build_assign(node_id, var_id, expr)
    }

    fn build_assign(&mut self, node_id: CfgNodeId, var_id: u64, expr: &Expression) -> CfgNodeId {
        self.build_expr(node_id, expr);

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
            ExpressionAst::ProcCall(..) => self.build_proc_call_expr(node_id, expr),
        }

        node_id
    }

    fn build_proc_call_expr(&mut self, node_id: CfgNodeId, expr: &Expression) {
        let (proc_name, proc_args_exprs, proc_id_option) = expr.as_proc_call_expr();

        for proc_arg_expr in proc_args_exprs {
            self.build_expr(node_id, proc_arg_expr);
        }

        let proc_id = proc_id_option.unwrap();
        let mut cfg_proc = self.proc_jmp_table.get(&proc_id);

        let mut jmp_node_id;

        if cfg_proc.is_none() {
            let proc_node_id = self.cfg_graph.new_node();

            let cfg_proc = CfgProc {
                node_id: proc_node_id,
                proc_id,
                built: false,
            };
            self.proc_jmp_table.insert(proc_id, cfg_proc);

            jmp_node_id = proc_node_id;
        } else {
            jmp_node_id = cfg_proc.unwrap().node_id;
        }

        self.append_inst(node_id, CfgInstruction::Call(jmp_node_id));
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
        self.append_inst(node_id, CfgInstruction::Int(lit as isize));
    }

    fn append_str_lit(&mut self, node_id: CfgNodeId, lit: &str) {
        self.append_inst(node_id, CfgInstruction::Str(lit.to_string()));
    }

    fn append_var_lit(&mut self, node_id: CfgNodeId, lit: &Option<SymbolId>) {
        let var_id = lit.as_ref().unwrap();
        let inst = CfgInstruction::Load(*var_id);

        self.append_inst(node_id, inst);
    }

    fn build_repeat(&mut self, node_id: CfgNodeId, repeat_stmt: &RepeatStmt) -> CfgNodeId {
        // 1)  allocate a new local variable of type `INT`, let's call it `TMPVAR_A`
        // 2)  allocate a new local variable of type `INT`, let's call it `TMPVAR_B`
        // 3)  emit instructions for `MAKE TMPVAR_A = 0               (within `CURRENT_NODE_ID node)
        // 4)  emit instructions for `MAKE TMPVAR_B = `cond_expr`     (within `CURRENT_NODE_ID node)
        // 5)  emit expression-instructions for `TMPVAR_A < TMPVAR_B` (within `CURRENT_NODE_ID node)
        // 6)  create a new empty CFG node. let's mark its node id as `WHILE_NODE_ID`
        // 7)  add edge `CURRENT_NODE_ID` --jmp-when-true--> `WHILE_NODE_ID`
        // 8)  generate statement-instructions for `block_stmt`  (within `WHILE_NODE_ID` node)
        //     the CFG generation will return `LAST_WHILE_BLOCK_NODE_ID` node_id
        // 9)  emit instructions for `TMPVAR_A = TMPVAR_A + 1`   (within `LAST_TRUE_BLOCK_NODE_ID`)
        // 10) emit expression-instructions for `TMPVAR_A < TMPVAR_B` (within `LAST_TRUE_BLOCK_NODE_ID`)
        // 11) add edge `LAST_TRUE_BLOCK_NODE_ID` --jmp-when-true--> `WHILE_NODE_ID`
        // 12) create a new empty CFG node. let's mark its node id as `AFTER_NODE_ID`
        // 13) add edge `LAST_TRUE_BLOCK_NODE_ID` --jmp-fallback--> `AFTER_NODE_ID`
        // 14) add edge `CURRENT_NODE_ID` --jmp-fallback--> `AFTER_NODE_ID`
        // 15) return `AFTER_NODE_ID` node_id (empty CFG node to be used for the next statement)

        // allocating temporary variables: `TMPVAR_A` and `TMPVAR_B`
        let (var_id_a, var_name_a) = self
            .env
            .new_tmp_var(self.current_proc_id, ExpressionType::Int);
        let (var_id_b, var_name_b) = self
            .env
            .new_tmp_var(self.current_proc_id, ExpressionType::Int);

        // MAKE TMPVAR_A = 0
        let expr = &repeat_stmt.count_expr;
        let zero_lit = LiteralExpr::Int(0);
        let zero_expr = Expression {
            expr_type: Some(ExpressionType::Int),
            expr_ast: ExpressionAst::Literal(zero_lit),
        };
        self.build_assign(node_id, var_id_a, &zero_expr);

        // MAKE TMPVAR_B = `cond_expr`
        self.build_assign(node_id, var_id_b, &repeat_stmt.count_expr);

        // TMPVAR_A < TMPVAR_B
        let var_lit_a = LiteralExpr::Var(var_name_a, Some(var_id_a));
        let var_lit_b = LiteralExpr::Var(var_name_b, Some(var_id_b));
        let var_lit_a_clone = var_lit_a.clone();
        let var_lit_b_clone = var_lit_b.clone();

        let var_expr_a = Expression {
            expr_ast: ExpressionAst::Literal(var_lit_a),
            expr_type: Some(ExpressionType::Int),
        };
        let var_expr_b = Expression {
            expr_ast: ExpressionAst::Literal(var_lit_b),
            expr_type: Some(ExpressionType::Int),
        };
        let cond_ast =
            ExpressionAst::Binary(BinaryOp::GT, Box::new(var_expr_a), Box::new(var_expr_b));
        let cond_expr = Expression {
            expr_ast: cond_ast,
            expr_type: Some(ExpressionType::Bool),
        };
        self.build_expr(node_id, &cond_expr);

        // `Repeat block`
        let while_node_id = self.cfg_graph.new_node();
        self.add_edge(node_id, while_node_id, CfgJumpType::WhenTrue);
        let last_while_block_node_id = self.build_block(while_node_id, &repeat_stmt.block);

        // TMPVAR_A = TMPVAR_A + 1
        let one_lit = LiteralExpr::Int(1);
        let one_expr = Expression {
            expr_type: Some(ExpressionType::Int),
            expr_ast: ExpressionAst::Literal(one_lit),
        };
        let var_expr_a = Expression {
            expr_ast: ExpressionAst::Literal(var_lit_a_clone),
            expr_type: Some(ExpressionType::Int),
        };
        let incr_var_a_ast =
            ExpressionAst::Binary(BinaryOp::Add, Box::new(var_expr_a), Box::new(one_expr));
        let incr_expr = Expression {
            expr_type: Some(ExpressionType::Int),
            expr_ast: incr_var_a_ast,
        };
        self.build_assign(last_while_block_node_id, var_id_a, &incr_expr);

        // TMPVAR_A < TMPVAR_B
        self.build_expr(last_while_block_node_id, &cond_expr);

        // jump when-true to the start of the loop
        self.add_edge(
            last_while_block_node_id,
            while_node_id,
            CfgJumpType::WhenTrue,
        );

        let after_node_id = self.cfg_graph.new_node();
        self.add_edge(
            last_while_block_node_id,
            after_node_id,
            CfgJumpType::Fallback,
        );
        self.add_edge(node_id, after_node_id, CfgJumpType::Fallback);

        after_node_id
    }

    fn build_if(&mut self, node_id: CfgNodeId, if_stmt: &IfStmt) -> CfgNodeId {
        // 1)  let's mark current CFG node as `CURRENT_NODE_ID` (the `node_id` parameter)
        //     this node is assumed to be empty
        // 2)  generate expression-instructions for `if-stmt` conditional-expression (within `CURRENT_NODE_ID` node)
        // 3)  create a new empty CFG node. let's mark its node id as `TRUE_NODE_ID`
        // 4)  generate statement-instructions for `if-stmt` `true-block` (within `TRUE_NODE_ID` node)
        //     the CFG generation will return `LAST_TRUE_BLOCK_NODE_ID` node_id
        // 5)  add edge `CURRENT_NODE_ID` --jmp-when-true--> `TRUE_NODE_ID`
        // 6)  if `if-stmt` has `else-block`:
        //       6.1) create a new empty CFG node. let's mark its node id as `FALSE_NODE_ID`
        //       6.2) generate statement-instructions for `false-block` (within `FALSE_NODE_ID` node)
        //            the CFG generation will return `LAST_FALSE_BLOCK_NODE_ID` node_id
        //       6.3) add edge `CURRENT_NODE_ID` --jmp-fallback--> `FALSE_NODE_ID`
        // 7)  create a new empty CFG node. let's mark its node id as `AFTER_NODE_ID`
        // 8)  add edge `LAST_TRUE_BLOCK_NODE_ID` --jmp-always--> `AFTER_NODE_ID`
        // 9)  if `if-stmt` has `else-block`:
        //       9.1) add edge `LAST_FALSE_BLOCK_NODE_ID` --jmp-always--> `AFTER_NODE_ID`
        //     else:
        //       9.1) add edge `CURRENT_NODE_ID` --jmp-fallback--> `AFTER_NODE_ID`
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

        let true_block_ends_with_empty_node = self.cfg_graph.node_is_empty(last_true_block_node_id);
        let true_block_ends_with_return = self.cfg_graph.ends_with_return(last_true_block_node_id);

        let mut draw_true_block_to_after_node_edge = false;

        let after_node_id = match true_block_ends_with_empty_node {
            true => Some(last_true_block_node_id), // we'll reuse this empty node
            false => {
                // we know the `true-block last node` isn't empty
                // but we want to allocate a new empty CFG node **only**
                // when the last node-statement *IS NOT* a `RETURN`-statement

                if true_block_ends_with_return {
                    // no need to draw edge `LAST_TRUE_BLOCK_NODE_ID` --jmp-always--> `AFTER_NODE_ID`
                    None
                } else {
                    draw_true_block_to_after_node_edge = true;
                    Some(self.cfg_graph.new_node())
                }
            }
        };

        if draw_true_block_to_after_node_edge {
            self.add_edge(
                last_true_block_node_id,
                after_node_id.unwrap(),
                CfgJumpType::Always,
            );
        }

        if if_stmt.false_block.is_some() {
            // we draw edge `LAST_FALSE_BLOCK_NODE_ID` --jmp-always--> `AFTER_NODE_ID`
            // only if the `else-block` statement *IS NOT* a `RETURN`-statement
            let last_false_block_node_id = last_false_block_node_id.unwrap();

            let false_block_ends_with_return =
                self.cfg_graph.ends_with_return(last_false_block_node_id);

            if false_block_ends_with_return == false {
                self.add_edge(
                    last_false_block_node_id,
                    after_node_id.unwrap(),
                    CfgJumpType::Always,
                );
            }
        } else {
            // there is no `else-block`
            // we'll draw edge `CURRENT_NODE_ID` --jmp-fallback--> `AFTER_NODE_ID`
            // in case there is an after node

            if after_node_id.is_some() {
                self.add_edge(node_id, after_node_id.unwrap(), CfgJumpType::Fallback);
            }
        }

        if after_node_id.is_some() {
            after_node_id.unwrap()
        } else {
            self.cfg_graph.new_node()
        }
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

    fn add_edge(&mut self, src_id: CfgNodeId, dst_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.cfg_graph.add_edge(src_id, dst_id, jmp_type);
    }
}
