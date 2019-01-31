#[macro_use]
extern crate logos;

use logos::ast::Ast;
use logos::parser::{LogosParser, Parser};

use logos::ast::expression::*;
use logos::ast::statement::*;

#[test]
fn direction_forward() {
    let actual = LogosParser.parse("FORWARD 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(FORWARD, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_backward() {
    let actual = LogosParser.parse("BACKWARD 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(BACKWARD, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_left() {
    let actual = LogosParser.parse("LEFT 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(LEFT, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_right() {
    let actual = LogosParser.parse("RIGHT 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(RIGHT, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_setx() {
    let actual = LogosParser.parse("SETX 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(SETX, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_sety() {
    let actual = LogosParser.parse("SETY 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(SETY, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_no_empty_lines() {
    let actual = LogosParser.parse("FORWARD 10\nRIGHT 20").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(FORWARD, 10), direct_lit_expr!(RIGHT, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_with_empty_lines() {
    let actual = LogosParser.parse("\n\nFORWARD 10\n\nRIGHT 20\n\n").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(FORWARD, 10), direct_lit_expr!(RIGHT, 20)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_integer_surrounded_by_parentheses() {
    let actual = LogosParser.parse("FORWARD (10)").unwrap();

    let expected = Ast {
        statements: vec![direct_lit_expr!(FORWARD, 10)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_with_spaces() {
    let actual = LogosParser.parse("FORWARD 1 + 2").unwrap();

    let expected = Ast {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: direction!(FORWARD),
            expr: binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2))
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_without_spaces() {
    let actual = LogosParser.parse("FORWARD 1 + 2").unwrap();

    let expected = Ast {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: direction!(FORWARD),
            expr: binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2))
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_and_mul_integers() {
    let actual = LogosParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

    let clause1 = binary_expr!(BinaryOp::Mul, boxed_int_expr!(1), boxed_int_expr!(2));
    let clause2 = binary_expr!(BinaryOp::Mul, boxed_int_expr!(3), boxed_int_expr!(4));
    let expr = binary_expr!(BinaryOp::Add, Box::new(clause1), Box::new(clause2));

    let expected = Ast {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: direction!(FORWARD),
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mul_integers_without_spaces() {
    let actual = LogosParser.parse("FORWARD 1 * 2").unwrap();

    let expr = binary_expr!(BinaryOp::Mul, boxed_int_expr!(1), boxed_int_expr!(2));

    let expected = Ast {
        statements: vec![Statement::Direction(DirectionStmt {
            expr,
            direction: direction!(FORWARD),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
    let actual = LogosParser.parse("FORWARD (1*1 + 2) * (3*3 + 4)").unwrap();

    let ones_mul = binary_expr!(BinaryOp::Mul, boxed_int_expr!(1), boxed_int_expr!(1));
    let three_mul = binary_expr!(BinaryOp::Mul, boxed_int_expr!(3), boxed_int_expr!(3));

    let add_1_2 = binary_expr!(BinaryOp::Add, Box::new(ones_mul), boxed_int_expr!(2));
    let add_3_4 = binary_expr!(BinaryOp::Add, Box::new(three_mul), boxed_int_expr!(4));

    let expr = binary_expr!(BinaryOp::Mul, Box::new(add_1_2), Box::new(add_3_4));

    let expected = Ast {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: direction!(FORWARD),
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_integer() {
    let actual = LogosParser.parse("MAKE \"MyVar = 2").unwrap();

    let make_stmt = make_stmt!("MyVar", int_expr!(2));

    let expected = Ast {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_a_string() {
    let actual = LogosParser.parse("MAKE \"MyVar = \"Hello").unwrap();

    let make_stmt = make_stmt!(
        "MyVar",
        Expression::Literal(LiteralExpr::Str("Hello".to_string()))
    );

    let expected = Ast {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_expr() {
    let actual = LogosParser.parse("MAKE \"MyVar = 1 + 2").unwrap();

    let expr = binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2));

    let make_stmt = make_stmt!("MyVar", expr);

    let expected = Ast {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn make_variable_must_be_prefixed_with_quotation_marks() {
    LogosParser.parse("MAKE A=1").unwrap();
}

#[test]
fn make_variable_assign_an_expr_containing_another_var() {
    let actual = LogosParser.parse("MAKE \"A = :B + 2").unwrap();

    let expr = binary_expr!(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Var("B".to_string()))),
        boxed_int_expr!(2)
    );

    let make_stmt = make_stmt!("A", expr);

    let expected = Ast {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn if_stmt_without_else() {
    let actual = LogosParser
        .parse("IF 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let cond_expr = binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2));

    let mut true_block = BlockStatement::new();
    true_block.add_statement(make_stmt!("A".to_string(), int_expr!(3)));
    true_block.add_statement(make_stmt!("B".to_string(), int_expr!(4)));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: None,
    });

    let expected = Ast {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn if_stmt_with_else() {
    let actual = LogosParser
        .parse("IF 1 + 2 [MAKE \"A = 1] [MAKE \"B = 2]")
        .unwrap();

    let cond_expr = binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2));

    let mut true_block = BlockStatement::new();
    true_block.add_statement(make_stmt!("A", int_expr!(1)));

    let mut false_block = BlockStatement::new();
    false_block.add_statement(make_stmt!("B", int_expr!(2)));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: Some(false_block),
    });

    let expected = Ast {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn repeat_stmt() {
    let actual = LogosParser
        .parse("REPEAT 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let count_expr = binary_expr!(BinaryOp::Add, boxed_int_expr!(1), boxed_int_expr!(2));

    let mut block = BlockStatement::new();
    block.add_statement(make_stmt!("A", int_expr!(3)));
    block.add_statement(make_stmt!("B", int_expr!(4)));

    let repeat_stmt = Statement::Repeat(RepeatStmt { count_expr, block });

    let expected = Ast {
        statements: vec![repeat_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn procedure_stmt_without_params() {
    let actual = LogosParser
        .parse("TO MyProc \n MAKE \"A = 3 \n MAKE \"B = 4 \n END")
        .unwrap();

    let mut block = BlockStatement::new();
    block.add_statement(make_stmt!("A".to_string(), int_expr!(3)));
    block.add_statement(make_stmt!("B".to_string(), int_expr!(4)));

    let proc_stmt = Statement::Procedure(ProcedureStmt {
        name: "MyProc".to_string(),
        params: vec![],
        block,
    });

    let expected = Ast {
        statements: vec![proc_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn procedure_stmt_with_params() {
    let actual = LogosParser
        .parse("TO MyProc :A :B \n MAKE \"C = 10 END")
        .unwrap();

    let mut block = BlockStatement::new();
     block.add_statement(make_stmt!("C".to_string(), int_expr!(10)));

    let proc_stmt = Statement::Procedure(ProcedureStmt {
        name: "MyProc".to_string(),
        params: vec!["A".to_string(), "B".to_string()],
        block,
    });

    let expected = Ast {
        statements: vec![proc_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_xcor() {
    let actual = LogosParser.parse("XCOR").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(XCOR)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_ycor() {
    let actual = LogosParser.parse("YCOR").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(YCOR)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_up() {
    let actual = LogosParser.parse("PENUP").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(PENUP)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_down() {
    let actual = LogosParser.parse("PENDOWN").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(PENDOWN)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_show_turtle() {
    let actual = LogosParser.parse("SHOWTURTLE").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(SHOWTURTLE)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_hide_turtle() {
    let actual = LogosParser.parse("HIDETURTLE").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(HIDETURTLE)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_erase() {
    let actual = LogosParser.parse("PENERASE").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(PENERASE)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_clean() {
    let actual = LogosParser.parse("CLEAN").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(CLEAN)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_clear_screen() {
    let actual = LogosParser.parse("CLEARSCREEN").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(CLEARSCREEN)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_pen_color() {
    let actual = LogosParser.parse("SETPENCOLOR").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(SETPENCOLOR)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_background_color() {
    let actual = LogosParser.parse("SETBACKGROUND").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(SETBACKGROUND)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_wait() {
    let actual = LogosParser.parse("WAIT").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(WAIT)],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_stop() {
    let actual = LogosParser.parse("STOP").unwrap();

    let expected = Ast {
        statements: vec![command_stmt!(STOP)],
    };

    assert_eq!(expected, actual);
}
