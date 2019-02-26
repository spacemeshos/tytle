extern crate tytle;

use tytle::ast::expression::*;

#[test]
fn pp_lit_true() {
    let ast = ExpressionAst::Literal(LiteralExpr::Bool(true));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "TRUE".to_string());
}

#[test]
fn pp_lit_false() {
    let ast = ExpressionAst::Literal(LiteralExpr::Bool(false));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "FALSE".to_string());
}

#[test]
fn pp_lit_int() {
    let ast = ExpressionAst::Literal(LiteralExpr::Int(10));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "10".to_string());
}

#[test]
fn pp_lit_str() {
    let ast = ExpressionAst::Literal(LiteralExpr::Str("Hello".to_string()));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "\"Hello\"".to_string());
}

#[test]
fn pp_lit_var() {
    let ast = ExpressionAst::Literal(LiteralExpr::Var("MYVAR".to_string()));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "MYVAR".to_string());
}

#[test]
fn pp_binary_expr_not() {
    let inner_ast = ExpressionAst::Literal(LiteralExpr::Bool(true));
    let inner_expr = Expression::new(inner_ast);

    let ast = ExpressionAst::Not(Box::new(inner_expr));
    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "NOT TRUE".to_string());
}

#[test]
fn pp_binary_expr_add() {
    let ast1 = ExpressionAst::Literal(LiteralExpr::Int(1));
    let ast2 = ExpressionAst::Literal(LiteralExpr::Var("X".to_string()));

    let ast = ExpressionAst::Binary(
        BinaryOp::Add,
        Box::new(Expression::new(ast1)),
        Box::new(Expression::new(ast2)),
    );

    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "1 + X".to_string());
}

#[test]
fn pp_binary_expr_and() {
    let ast1 = ExpressionAst::Literal(LiteralExpr::Bool(true));
    let ast2 = ExpressionAst::Literal(LiteralExpr::Bool(false));

    let ast = ExpressionAst::Binary(
        BinaryOp::And,
        Box::new(Expression::new(ast1)),
        Box::new(Expression::new(ast2)),
    );

    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "TRUE AND FALSE".to_string());
}

#[test]
fn pp_proc_call_expr() {
    let ast1 = ExpressionAst::Literal(LiteralExpr::Int(1));
    let ast2 = ExpressionAst::Literal(LiteralExpr::Int(2));

    let proc_args = vec![Expression::new(ast1), Expression::new(ast2)];
    let ast = ExpressionAst::ProcCall("MYPROC".to_string(), proc_args);

    let expr = Expression::new(ast);
    let buffer = PrettyPrintAst::pprint_expr(&expr);

    assert_eq!(buffer, "MYPROC(1, 2)".to_string());
}
