use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::Ast;

pub struct PrettyPrintAst;

impl PrettyPrintAst {
    pub fn pprint_ast(ast: &Ast) -> String {
        let mut buffer = Vec::<String>::new();

        for stmt in &ast.statements {
            Self::do_pprint_stmt(&mut buffer, stmt);
        }

        buffer.join("")
    }

    pub fn pprint_stmt(stmt: &Statement) -> String {
        let mut buffer = Vec::<String>::new();

        Self::do_pprint_stmt(&mut buffer, stmt);

        buffer.join("")
    }

    pub fn pprint_expr(expr: &Expression) -> String {
        let mut buffer = Vec::<String>::new();

        Self::do_pprint_expr(&mut buffer, expr);

        buffer.join("")
    }

    fn do_pprint_stmt(buffer: &mut Vec<String>, stmt: &Statement) {
        match stmt {
            Statement::NOP | Statement::EOF => return,
            Statement::Command(cmd_stmt) => Self::pp_command_stmt(buffer, cmd_stmt),
            Statement::Direction(direct_stmt) => Self::pp_direction_stmt(buffer, direct_stmt),
            Statement::Make(make_stmt) => Self::pp_make_stmt(buffer, make_stmt),
            Statement::If(if_stmt) => Self::pp_if_stmt(buffer, if_stmt),
            Statement::Repeat(repeat_stmt) => Self::pp_repeat_stmt(buffer, repeat_stmt),
            Statement::Procedure(proc_stmt) => Self::pp_proc_stmt(buffer, proc_stmt),
            _ => unimplemented!(),
        };
    }

    fn do_pprint_expr(buffer: &mut Vec<String>, expr: &Expression) {
        match expr.expr_ast {
            ExpressionAst::Literal(ref lit_expr) => Self::pp_lit_expr(buffer, &lit_expr),
            ExpressionAst::Binary(_, _, _) => Self::pp_binary_expr(buffer, expr),
            ExpressionAst::ProcCall(_, _) => Self::pp_proc_call_expr(buffer, expr),
            ExpressionAst::Not(_) => Self::pp_not_expr(buffer, expr),
            ExpressionAst::Parentheses(_) => Self::pp_parentheses_expr(buffer, expr),
            _ => unimplemented!(),
        };
    }

    fn pp_lit_expr(buffer: &mut Vec<String>, lit_expr: &LiteralExpr) {
        match lit_expr {
            LiteralExpr::Bool(true) => buffer.push("TRUE".to_string()),
            LiteralExpr::Bool(false) => buffer.push("FALSE".to_string()),
            LiteralExpr::Int(num) => buffer.push(num.to_string()),
            LiteralExpr::Str(s) => buffer.push(format!("\"{}\"", s)),
            LiteralExpr::Var(v) => buffer.push(v.clone()),
        }
    }

    fn pp_not_expr(buffer: &mut Vec<String>, not_expr: &Expression) {
        let inner_expr = not_expr.as_not_expr();

        Self::pp_binary_op(buffer, &BinaryOp::Not);

        Self::do_pprint_expr(buffer, inner_expr);
    }

    fn pp_binary_expr(buffer: &mut Vec<String>, bin_expr: &Expression) {
        let (binary_op, lexpr, rexpr) = bin_expr.as_binary_expr();

        Self::do_pprint_expr(buffer, lexpr);
        Self::pp_binary_op(buffer, binary_op);
        Self::do_pprint_expr(buffer, rexpr);
    }

    fn pp_proc_call_expr(buffer: &mut Vec<String>, proc_call_expr: &Expression) {
        let (proc_name, proc_args) = proc_call_expr.as_proc_call_expr();

        buffer.push(format!("{}(", proc_name));

        let mut i = 0;
        while i < proc_args.len() - 1 {
            Self::do_pprint_expr(buffer, &proc_args[i]);
            buffer.push(", ".to_string());

            i += 1
        }

        Self::do_pprint_expr(buffer, &proc_args[i]);

        buffer.push(")".to_string());
    }

    fn pp_parentheses_expr(buffer: &mut Vec<String>, expr: &Expression) {
        let inner_expr = expr.as_parentheses_expr();

        buffer.push("(".to_string());
        Self::do_pprint_expr(buffer, inner_expr);
        buffer.push(")".to_string());
    }

    fn pp_binary_op(buffer: &mut Vec<String>, binary_op: &BinaryOp) {
        let s = match binary_op {
            BinaryOp::Not => "NOT ",
            BinaryOp::And => " AND ",
            BinaryOp::Or => " OR ",
            BinaryOp::Add => " + ",
            BinaryOp::Mul => " * ",
            BinaryOp::GT => " > ",
            BinaryOp::LT => " < ",
            _ => unimplemented!(),
        };

        buffer.push(s.to_string());
    }

    fn pp_command_stmt(buffer: &mut Vec<String>, cmd_stmt: &CommandStmt) {
        unimplemented!()
       // TODO: command into string
    }

    fn pp_direction_stmt(buffer: &mut Vec<String>, direct_stmt: &DirectionStmt) {
        unimplemented!()
       // TODO: direction into string
    }

    fn pp_make_stmt(buffer: &mut Vec<String>, make_stmt: &MakeStmt) {
        let kind_str =
            match make_stmt.kind {
                MakeStmtKind::Global => "MAKEGLOBAL ",
                MakeStmtKind::Local => "MAKELOCAL ",
                MakeStmtKind::Assign => "MAKE "
            };

        buffer.push(format!("{} {} = ", kind_str, make_stmt.var));
        Self::do_pprint_expr(buffer, &make_stmt.expr)
    }

    fn pp_if_stmt(buffer: &mut Vec<String>, if_stmt: &IfStmt) {
        buffer.push("IF ".to_string());
        Self::do_pprint_expr(buffer, &if_stmt.cond_expr);

        buffer.push("\n".to_string());
        Self::pp_block_stmt(buffer, &if_stmt.true_block);
    }

    fn pp_repeat_stmt(buffer: &mut Vec<String>, repeat_stmt: &RepeatStmt) {
        buffer.push("REPEAT ".to_string());
        Self::do_pprint_expr(buffer, &repeat_stmt.count_expr);

        buffer.push("\n".to_string());
        Self::pp_block_stmt(buffer, &repeat_stmt.block);
    }

    fn pp_proc_stmt(buffer: &mut Vec<String>, proc_stmt: &ProcedureStmt) {
        unimplemented!()
    }

    fn pp_block_stmt(buffer: &mut Vec<String>, block_stmt: &BlockStatement) {
        buffer.push("[\n".to_string());
        for stmt in &block_stmt.stmts {
            let stmt_str: String = Self::pprint_stmt(stmt);
            buffer.push(format!("   {}", stmt_str));
        }
        buffer.push("\n]".to_string());
    }
}
