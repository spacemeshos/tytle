extern crate logos;

use logos::ast::program::Program;
use logos::parser::program_parser::ProgramParser;
use logos::parser::Parser;

use logos::ast::expression::Expression;

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
    let actual = ProgramParser.parse("FORWARD 20").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Int(20),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_backward() {
    let actual = ProgramParser.parse("BACKWARD 20").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Backward,
            expr: Expression::Int(20),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_left() {
    let actual = ProgramParser.parse("LEFT 20").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Left,
            expr: Expression::Int(20),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_right() {
    let actual = ProgramParser.parse("RIGHT 20").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Right,
            expr: Expression::Int(20),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_no_empty_lines() {
    let actual = ProgramParser.parse("FORWARD 10\nRIGHT 20").unwrap();

    let expected = Program {
        statements: vec![
            Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Int(10),
            }),
            Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                expr: Expression::Int(20),
            }),
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_with_empty_lines() {
    let actual = ProgramParser
        .parse("\n\nFORWARD 10\n\nRIGHT 20\n\n")
        .unwrap();

    let expected = Program {
        statements: vec![
            Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Int(10),
            }),
            Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                expr: Expression::Int(20),
            }),
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_integer_surrounded_by_parentheses() {
    let actual = ProgramParser.parse("FORWARD (10)").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Int(10),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_with_spaces() {
    let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_without_spaces() {
    let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_and_mul_integers() {
    let actual = ProgramParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

    let clause1 = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));
    let clause2 = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(4)));
    let expr = Expression::Add(Box::new(clause1), Box::new(clause2));

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mul_integers_without_spaces() {
    let actual = ProgramParser.parse("FORWARD 1 * 2").unwrap();

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr: Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
    let actual = ProgramParser
        .parse("FORWARD (1*1 + 2) * (3*3 + 4)")
        .unwrap();

    let ones_mul = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(1)));
    let three_mul = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(3)));

    let add_1_2 = Expression::Add(Box::new(ones_mul), Box::new(Expression::Int(2)));
    let add_3_4 = Expression::Add(Box::new(three_mul), Box::new(Expression::Int(4)));

    let expr = Expression::Mul(Box::new(add_1_2), Box::new(add_3_4));

    let expected = Program {
        statements: vec![Statement::Direction(DirectionStmt {
            direction: Direction::Forward,
            expr,
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_integer() {
    let actual = ProgramParser.parse("MAKE \"MyVar = 2").unwrap();

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "MyVar".to_string(),
        expr: Expression::Int(2),
    });

    let expected = Program {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_expr() {
    let actual = ProgramParser.parse("MAKE \"MyVar = 1 + 2").unwrap();

    let expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

    let make_stmt = Statement::Make(MakeStmt {
        symbol: "MyVar".to_string(),
        expr,
    });

    let expected = Program {
        statements: vec![make_stmt],
    };

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn make_variable_must_be_prefixed_with_quotation_marks() {
    ProgramParser.parse("MAKE A=1").unwrap();
}

#[test]
fn if_stmt_without_else() {
    let actual = ProgramParser
        .parse("IF 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let cond_expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

    let mut true_block = BlockStatement::new();
    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Int(3),
    }));

    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Int(4),
    }));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: None,
    });

    let expected = Program {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn if_stmt_with_else() {
    let actual = ProgramParser
        .parse("IF 1 + 2 [MAKE \"A = 1] [MAKE \"B = 2]")
        .unwrap();

    let cond_expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

    let mut true_block = BlockStatement::new();
    true_block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Int(1),
    }));

    let mut false_block = BlockStatement::new();
    false_block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Int(2),
    }));

    let if_stmt = Statement::If(IfStmt {
        cond_expr,
        true_block,
        false_block: Some(false_block),
    });

    let expected = Program {
        statements: vec![if_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn repeat_stmt() {
    let actual = ProgramParser
        .parse("REPEAT 1 + 2 [MAKE \"A = 3 \n MAKE \"B = 4]")
        .unwrap();

    let count_expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

    let mut block = BlockStatement::new();
    block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Int(3),
    }));

    block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Int(4),
    }));

    let repeat_stmt = Statement::Repeat(RepeatStmt { count_expr, block });

    let expected = Program {
        statements: vec![repeat_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn to_stmt() {
    let actual = ProgramParser
        .parse("TO MyProc \n MAKE \"A = 3 \n MAKE \"B = 4 \n END")
        .unwrap();

    let mut block = BlockStatement::new();
    block.add_statement(Statement::Make(MakeStmt {
        symbol: "A".to_string(),
        expr: Expression::Int(3),
    }));

    block.add_statement(Statement::Make(MakeStmt {
        symbol: "B".to_string(),
        expr: Expression::Int(4),
    }));

    let proc_stmt = Statement::Procedure(ProcedureStmt {
        name: "MyProc".to_string(),
        block,
    });

    let expected = Program {
        statements: vec![proc_stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_up() {
    let actual = ProgramParser.parse("PENUP").unwrap();

    let stmt = Statement::Command(CommandStmt::PenUp);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_down() {
    let actual = ProgramParser.parse("PENDOWN").unwrap();

    let stmt = Statement::Command(CommandStmt::PenDown);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_show_turtle() {
    let actual = ProgramParser.parse("SHOWTURTLE").unwrap();

    let stmt = Statement::Command(CommandStmt::ShowTurtle);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_hide_turtle() {
    let actual = ProgramParser.parse("HIDETURTLE").unwrap();

    let stmt = Statement::Command(CommandStmt::HideTurtle);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_erase() {
    let actual = ProgramParser.parse("PENERASE").unwrap();

    let stmt = Statement::Command(CommandStmt::PenErase);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_clear_screen() {
    let actual = ProgramParser.parse("CLEARSCREEN").unwrap();

    let stmt = Statement::Command(CommandStmt::ClearScreen);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_pen_color() {
    let actual = ProgramParser.parse("SETPENCOLOR").unwrap();

    let stmt = Statement::Command(CommandStmt::SetPenColor);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_background_color() {
    let actual = ProgramParser.parse("SETBACKGROUND").unwrap();

    let stmt = Statement::Command(CommandStmt::SetBackgroundColor);

    let expected = Program {
        statements: vec![stmt],
    };

    assert_eq!(expected, actual);
}
