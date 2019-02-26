use crate::ast::expression::*;

pub struct PrettyPrintExpr;

impl PrettyPrintExpr {
    pub fn pprint_expr(expr: &Expression) -> String {
        let mut buffer = Vec::<String>::new();

        Self::do_pprint_expr(&mut buffer, expr);

        buffer.join("")
    }

    fn do_pprint_expr(buffer: &mut Vec<String>, expr: &Expression) {
        match expr.expr_ast {
            ExpressionAst::Literal(ref lit_expr) => Self::pp_lit_expr(buffer, &lit_expr),
            ExpressionAst::Binary(_, _, _) => Self::pp_binary_expr(buffer, expr),
            ExpressionAst::ProcCall(_, _) => Self::pp_proc_call_expr(buffer, expr),
            ExpressionAst::Not(_) => Self::pp_not_expr(buffer, expr),
            _ => panic!("..."),
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
}
