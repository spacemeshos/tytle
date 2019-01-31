extern crate logos;

use logos::ast::program_ast::ProgramAst;
use logos::parser::simple_parser::SimpleParser;
use logos::parser::Parser;

use logos::ast::expression::BinaryOp;
use logos::ast::expression::Expression;
use logos::ast::expression::LiteralExpr;

use logos::ast::statement::{
    block_stmt::BlockStatement,
    command_stmt::CommandStmt,
    direction::{Direction, DirectionStmt},
    if_stmt::IfStmt,
    make_stmt::MakeStmt,
    procedure_stmt::ProcedureStmt,
    repeat_stmt::RepeatStmt,
    Statement,
};

#[test]
fn direction_forward() {
    let actual = SimpleParser.parse("FORWARD 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_backward() {
    let actual = SimpleParser.parse("BACKWARD 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Backward,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_left() {
    let actual = SimpleParser.parse("LEFT 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Left,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_right() {
    let actual = SimpleParser.parse("RIGHT 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Right,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_setx() {
    let actual = SimpleParser.parse("SETX 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::SetX,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_sety() {
    let actual = SimpleParser.parse("SETY 20").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::SetY,
            expr: Expression::Literal(LiteralExpr::Int(20)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_no_empty_lines() {
    let actual = SimpleParser.parse("FORWARD 10\nRIGHT 20").unwrap();

    let expected = ProgramAst {
        statements: vec![
            Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Literal(LiteralExpr::Int(10)),
            }),
            Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                expr: Expression::Literal(LiteralExpr::Int(20)),
            }),
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_with_empty_lines() {
    let actual = SimpleParser
        .parse("\n\nFORWARD 10\n\nRIGHT 20\n\n")
        .unwrap();

    let expected = ProgramAst {
        statements: vec![
            Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Literal(LiteralExpr::Int(10)),
            }),
            Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                expr: Expression::Literal(LiteralExpr::Int(20)),
            }),
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_integer_surrounded_by_parentheses() {
    let actual = SimpleParser.parse("FORWARD (10)").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Literal(LiteralExpr::Int(10)),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_with_spaces() {
    let actual = SimpleParser.parse("FORWARD 1 + 2").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Binary(
                BinaryOp::Add,
                Box::new(Expression::Literal(LiteralExpr::Int(1))),
                Box::new(Expression::Literal(LiteralExpr::Int(2))),
            ),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_without_spaces() {
    let actual = SimpleParser.parse("FORWARD 1 + 2").unwrap();

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Binary(
                BinaryOp::Add,
                Box::new(Expression::Literal(LiteralExpr::Int(1))),
                Box::new(Expression::Literal(LiteralExpr::Int(2))),
            ),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_and_mul_integers() {
    let actual = SimpleParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

    let clause1 = Expression::Binary(
        BinaryOp::Mul,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let clause2 = Expression::Binary(
        BinaryOp::Mul,
        Box::new(Expression::Literal(LiteralExpr::Int(3))),
        Box::new(Expression::Literal(LiteralExpr::Int(4))),
    );

    let expr = Expression::Binary(BinaryOp::Add, Box::new(clause1), Box::new(clause2));

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mul_integers_without_spaces() {
    let actual = SimpleParser.parse("FORWARD 1 * 2").unwrap();

    let expr = Expression::Binary(
        BinaryOp::Mul,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            expr,
            direction: Direction::Forward,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
    let actual = SimpleParser.parse("FORWARD (1*1 + 2) * (3*3 + 4)").unwrap();

    let ones_mul = Expression::Binary(
        BinaryOp::Mul,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
    );

    let three_mul = Expression::Binary(
        BinaryOp::Mul,
        Box::new(Expression::Literal(LiteralExpr::Int(3))),
        Box::new(Expression::Literal(LiteralExpr::Int(3))),
    );

    let add_1_2 = Expression::Binary(
        BinaryOp::Add,
        Box::new(ones_mul),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );
    let add_3_4 = Expression::Binary(
        BinaryOp::Add,
        Box::new(three_mul),
        Box::new(Expression::Literal(LiteralExpr::Int(4))),
    );

    let expr = Expression::Binary(BinaryOp::Mul, Box::new(add_1_2), Box::new(add_3_4));

    let expected = ProgramAst {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_integer() {
    let actual = SimpleParser.parse("MAKE \"MyVar = 2").unwrap();

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "MyVar".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(2)),
    });

    let expected = ProgramAst {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_a_string() {
    let actual = SimpleParser.parse("MAKE \"MyVar = \"Hello").unwrap();

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "MyVar".to_string(),
        expr: Expression::Literal(LiteralExpr::Str("Hello".to_string())),
    });

    let expected = ProgramAst {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_expr() {
    let actual = SimpleParser.parse("MAKE \"MyVar = 1 + 2").unwrap();

    let expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "MyVar".to_string(),
        expr,
    });

    let expected = ProgramAst {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn make_variable_must_be_prefixed_with_quotation_marks() {
    SimpleParser.parse("MAKE A=1").unwrap();
}

#[test]
fn make_variable_assign_an_expr_containing_another_var() {
    let actual = SimpleParser.parse("MAKE \"A = :B + 2").unwrap();

    let expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Var("B".to_string()))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr,
    });

    let expected = ProgramAst {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn if_stmt_without_else() {
    let actual = SimpleParser
        .parse("IF 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let cond_expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let mut true_block = BlockStatement::new();
    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(3)),
    }));

    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(4)),
    }));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: None,
    });

    let expected = ProgramAst {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn if_stmt_with_else() {
    let actual = SimpleParser
        .parse("IF 1 + 2 [MAKE \"A = 1] [MAKE \"B = 2]")
        .unwrap();

    let cond_expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let mut true_block = BlockStatement::new();
    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(1)),
    }));

    let mut false_block = BlockStatement::new();
    false_block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(2)),
    }));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: Some(false_block),
    });

    let expected = ProgramAst {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn repeat_stmt() {
    let actual = SimpleParser
        .parse("REPEAT 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let count_expr = Expression::Binary(
        BinaryOp::Add,
        Box::new(Expression::Literal(LiteralExpr::Int(1))),
        Box::new(Expression::Literal(LiteralExpr::Int(2))),
    );

    let mut block = BlockStatement::new();
    block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(3)),
    }));

    block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(4)),
    }));

    let repeat_stmt = Statement::Repeat(RepeatStmt { count_expr, block });

    let expected = ProgramAst {
        statements: vec![repeat_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn procedure_stmt_without_params() {
    let actual = SimpleParser
        .parse("TO MyProc \n MAKE \"A = 3 \n MAKE \"B = 4 \n END")
        .unwrap();

    let mut block = BlockStatement::new();
    block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(3)),
    }));

    block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(4)),
    }));

    let proc_stmt = Statement::Procedure(ProcedureStmt {
        name: "MyProc".to_string(),
        params: vec![],
        block,
    });

    let expected = ProgramAst {
        statements: vec![proc_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn procedure_stmt_with_params() {
    let actual = SimpleParser
        .parse("TO MyProc :A :B \n MAKE \"C = 10 END")
        .unwrap();

    let mut block = BlockStatement::new();
    block.add_statement(Statement::Make(MakeStmt {
        symbol: "C".to_string(),
        expr: Expression::Literal(LiteralExpr::Int(10)),
    }));

    let proc_stmt = Statement::Procedure(ProcedureStmt {
        name: "MyProc".to_string(),
        params: vec!["A".to_string(), "B".to_string()],
        block,
    });

    let expected = ProgramAst {
        statements: vec![proc_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_xcor() {
    let actual = SimpleParser.parse("XCOR").unwrap();

    let stmt = Statement::Command(CommandStmt::XCor);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_ycor() {
    let actual = SimpleParser.parse("YCOR").unwrap();

    let stmt = Statement::Command(CommandStmt::YCor);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_up() {
    let actual = SimpleParser.parse("PENUP").unwrap();

    let stmt = Statement::Command(CommandStmt::PenUp);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_down() {
    let actual = SimpleParser.parse("PENDOWN").unwrap();

    let stmt = Statement::Command(CommandStmt::PenDown);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_show_turtle() {
    let actual = SimpleParser.parse("SHOWTURTLE").unwrap();

    let stmt = Statement::Command(CommandStmt::ShowTurtle);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_hide_turtle() {
    let actual = SimpleParser.parse("HIDETURTLE").unwrap();

    let stmt = Statement::Command(CommandStmt::HideTurtle);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_erase() {
    let actual = SimpleParser.parse("PENERASE").unwrap();

    let stmt = Statement::Command(CommandStmt::PenErase);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_clean() {
    let actual = SimpleParser.parse("CLEAN").unwrap();

    let stmt = Statement::Command(CommandStmt::Clean);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_clear_screen() {
    let actual = SimpleParser.parse("CLEARSCREEN").unwrap();

    let stmt = Statement::Command(CommandStmt::ClearScreen);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_pen_color() {
    let actual = SimpleParser.parse("SETPENCOLOR").unwrap();

    let stmt = Statement::Command(CommandStmt::SetPenColor);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_background_color() {
    let actual = SimpleParser.parse("SETBACKGROUND").unwrap();

    let stmt = Statement::Command(CommandStmt::SetBackgroundColor);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_wait() {
    let actual = SimpleParser.parse("WAIT").unwrap();

    let stmt = Statement::Command(CommandStmt::Wait);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_stop() {
    let actual = SimpleParser.parse("STOP").unwrap();

    let stmt = Statement::Command(CommandStmt::Stop);

    let expected = ProgramAst {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}
