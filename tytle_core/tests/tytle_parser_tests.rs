#[macro_use]
extern crate tytle;

use tytle::ast::{expression::*, statement::*};
use tytle::parser::{ParseError, Parser, TytleParser};

macro_rules! assert_parse_err {
    ($expected:expr, $code:expr) => {{
        let actual = TytleParser.parse($code).err().unwrap();
        assert_eq!($expected, actual);
    }};
}

macro_rules! assert_reserved_word {
    ($keyword:expr) => {
        let var_code = format!("MAKEGLOBAL {} = 1", $keyword);
        let proc_code = format!("TO {}() END", $keyword);

        let expected = ParseError::ReservedKeyword($keyword.to_string());

        assert_parse_err!(expected, var_code.as_str());
        assert_parse_err!(expected, proc_code.as_str());
    };
}

macro_rules! assert_invalid_identifier {
    ($keyword:expr) => {{
        let var_code = format!("MAKEGLOBAL {} = 1", $keyword);
        let proc_code = format!("TO {}() END", $keyword);

        let expected = ParseError::IdentifierExpected;

        assert_parse_err!(expected, var_code.as_str());
        assert_parse_err!(expected, proc_code.as_str());
    }};
}

#[test]
fn parse_nop_stmt() {
    let actual = TytleParser.parse("").unwrap();
    let expected = ast! { eof!() };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_forward() {
    let actual = TytleParser.parse("FORWARD 20").unwrap();
    let expected = ast! { direct_lit_expr!(FORWARD, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_backward() {
    let actual = TytleParser.parse("BACKWARD 20").unwrap();
    let expected = ast! { direct_lit_expr!(BACKWARD, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_left() {
    let actual = TytleParser.parse("LEFT 20").unwrap();
    let expected = ast! { direct_lit_expr!(LEFT, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_right() {
    let actual = TytleParser.parse("RIGHT 20").unwrap();
    let expected = ast! { direct_lit_expr!(RIGHT, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_multiple_multiple_direction_statements_under_single_line() {
    let actual = TytleParser
        .parse("RIGHT 10 LEFT 20 BACKWARD 30 FORWARD 40")
        .unwrap();

    let expected = ast! {
        direct_lit_expr!(RIGHT, 10),
        direct_lit_expr!(LEFT, 20),
        direct_lit_expr!(BACKWARD, 30),
        direct_lit_expr!(FORWARD, 40)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_setx() {
    let actual = TytleParser.parse("SETX 20").unwrap();
    let expected = ast! { direct_lit_expr!(SETX, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_sety() {
    let actual = TytleParser.parse("SETY 20").unwrap();
    let expected = ast! { direct_lit_expr!(SETY, 20) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_forward_and_then_backward_no_empty_lines() {
    let code = r#"
        FORWARD 10
        RIGHT 20
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        direct_lit_expr!(FORWARD, 10),
        direct_lit_expr!(RIGHT, 20)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_direction_forward_and_then_backward_with_empty_lines() {
    let code = r#"

        FORWARD 10

        RIGHT 20

    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        direct_lit_expr!(FORWARD, 10),
        direct_lit_expr!(RIGHT, 20)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_integer_surrounded_by_parentheses() {
    let actual = TytleParser.parse("FORWARD (10)").unwrap();
    let expected = ast! { direct_lit_expr!(FORWARD, 10, parens: true) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_add_integers_with_spaces() {
    let actual = TytleParser.parse("FORWARD 1 + 2").unwrap();

    let expected = ast! {
        direct_stmt!(
            FORWARD,
            binary_expr!(
                "+",
                boxed_int_lit_expr!(1),
                boxed_int_lit_expr!(2)
            )
        )
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_add_integers_without_spaces() {
    let actual = TytleParser.parse("FORWARD 1 + 2").unwrap();

    let expected = ast! {
        direct_stmt!(
            FORWARD,
            binary_expr!(
                "+",
                boxed_int_lit_expr!(1),
                boxed_int_lit_expr!(2)
            )
        )
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_add_and_mul_integers() {
    let actual = TytleParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

    let clause1 = binary_expr!("*", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));
    let clause2 = binary_expr!("*", boxed_int_lit_expr!(3), boxed_int_lit_expr!(4));
    let expr = binary_expr!("+", Box::new(clause1), Box::new(clause2));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_mul_integers_without_spaces() {
    let actual = TytleParser.parse("FORWARD 1*2*3").unwrap();

    let expr = binary_expr!(
        "*",
        boxed_int_lit_expr!(1),
        boxed_expr! {
            binary_expr!("*",
            boxed_int_lit_expr!(2),
            boxed_int_lit_expr!(3))
        }
    );

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_div_integers_without_spaces() {
    let actual = TytleParser.parse("FORWARD 1/2").unwrap();

    let expr = binary_expr!("/", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_mul_and_div_integers() {
    let actual = TytleParser.parse("FORWARD 2 * 3 / 5").unwrap();

    let expr = binary_expr!(
        "*",
        boxed_int_lit_expr!(2),
        boxed_expr! {
           binary_expr!("/",
             boxed_int_lit_expr!(3),
             boxed_int_lit_expr!(5)
           )
        }
    );

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
    let actual = TytleParser.parse("FORWARD (1*1 + 2) * (3*3 + 4)").unwrap();

    let ones_mul = binary_expr!("*", boxed_int_lit_expr!(1), boxed_int_lit_expr!(1));
    let three_mul = binary_expr!("*", boxed_int_lit_expr!(3), boxed_int_lit_expr!(3));

    let add_1_2 = binary_expr!("+", Box::new(ones_mul), boxed_int_lit_expr!(2), parens: true);
    let add_3_4 = binary_expr!("+", Box::new(three_mul), boxed_int_lit_expr!(4), parens: true);

    let expr = binary_expr!("*", Box::new(add_1_2), Box::new(add_3_4));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_stmt_consisting_of_arithmetic_expr() {
    let actual = TytleParser.parse("1 + 2").unwrap();

    let expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let expected = ast! { expr_stmt!(expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_stmt_consisting_of_proc_call() {
    let actual = TytleParser.parse("FOO(1, 2)").unwrap();

    let call_expr = proc_call_expr! {
        name: "FOO",
        params: [int_lit_expr!(1), int_lit_expr!(2)]
    };

    let expected = ast! { expr_stmt!(call_expr) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_expr_proc_call_as_part_of_expr() {
    let actual = TytleParser
        .parse("FORWARD FOO(10, X + 1, BAR(2, 3))")
        .unwrap();

    let expected = ast! {
        direct_stmt!(
            FORWARD,
            proc_call_expr! {
                name: "FOO",
                params: [
                    int_lit_expr!(10),
                    binary_expr!(
                        "+",
                        boxed_var_lit_expr!("X"),
                        boxed_int_lit_expr!(1)
                    ),
                    proc_call_expr! {
                        name: "BAR",
                        params: [int_lit_expr!(2), int_lit_expr!(3)]
                    }
                ]
            }
        )
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_print_const() {
    let actual = TytleParser.parse("PRINT 10").unwrap();

    let expected = ast! {
        print_stmt!(int_lit_expr!(10))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_print_var_expr() {
    let code = r#"
        MAKEGLOBAL X = 5
        PRINT X + 10
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expr = binary_expr!("+", boxed_var_lit_expr!("X"), boxed_int_lit_expr!(10));

    let expected = ast! {
        make_global_stmt!("X", int_lit_expr!(5)),
        print_stmt!(expr)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKE MYVAR = 2").unwrap();

    let expected = ast! {
        make_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_variable_assign_a_string() {
    let code = r#"
        MAKE MYVAR = "Hello"
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        make_stmt!("MYVAR", str_lit_expr!("Hello"))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_variable_assign_an_expr() {
    let actual = TytleParser.parse("MAKE MYVAR = 1 + 2").unwrap();

    let expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let expected = ast! {
        make_stmt!("MYVAR", expr)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_variable_assign_an_expr_containing_another_var() {
    let actual = TytleParser.parse("MAKE A = B + 2").unwrap();

    let expr = binary_expr!("+", boxed_var_lit_expr!("B"), boxed_int_lit_expr!(2));

    let expected = ast! {
        make_stmt!("A", expr)
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_global_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKEGLOBAL MYVAR = 2").unwrap();

    let expected = ast! {
        make_global_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_local_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKELOCAL MYVAR = 2").unwrap();

    let expected = ast! {
        make_local_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_global_variable_assign_a_boolean_true() {
    let actual = TytleParser.parse("MAKEGLOBAL MYVAR = TRUE").unwrap();

    let expected = ast! {
        make_global_stmt!("MYVAR", bool_lit_expr!(true))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_global_variable_assign_a_boolean_false() {
    let actual = TytleParser.parse("MAKEGLOBAL MYVAR = FALSE").unwrap();

    let expected = ast! {
        make_global_stmt!("MYVAR", bool_lit_expr!(false))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_make_global_variable_assign_a_boolean_not_true() {
    let code = r#"
        MAKEGLOBAL A = NOT TRUE
        MAKEGLOBAL B = NOT(FALSE)
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        make_global_stmt!("A", not_expr!(bool_lit_expr!(true))),
        make_global_stmt!("B", not_expr!(bool_lit_expr!(false, parens: true)))
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_if_stmt_without_else() {
    let code = r#"
        IF 1 > 2 [
            MAKE A = 3
            MAKE B = 4
        ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let cond_expr = binary_expr!(">", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! {
            make_stmt!("A", int_lit_expr!(3)),
            make_stmt!("B", int_lit_expr!(4))
        }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn parse_if_stmt_and_clauses() {
    let code = r#"
        IF 1 > 2 AND 3 < 4 [
            MAKE A = 10
        ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let lexpr = binary_expr!(">", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));
    let rexpr = binary_expr!("<", boxed_int_lit_expr!(3), boxed_int_lit_expr!(4));
    let cond_expr = binary_expr!("AND", boxed_expr!(lexpr), boxed_expr!(rexpr));

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! {
            make_stmt!("A", int_lit_expr!(10))
        }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn parse_if_stmt_and_or_parens_clauses() {
    let code = r#"
        IF ((((1 > 2) OR ((3 < 4))) AND (5 < 6)) OR (7 < 8)) [
            MAKE A = 10
        ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expr12 = binary_expr!(">", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2), parens: true);
    let expr34 = with_parentheses!(
        binary_expr!("<", boxed_int_lit_expr!(3), boxed_int_lit_expr!(4), parens: true)
    );
    let expr56 = binary_expr!("<", boxed_int_lit_expr!(5), boxed_int_lit_expr!(6), parens: true);
    let expr78 = binary_expr!("<", boxed_int_lit_expr!(7), boxed_int_lit_expr!(8), parens: true);

    // (1 > 2) OR (3 < 4)
    let or_clause = binary_expr!("OR", boxed_expr!(expr12), boxed_expr!(expr34), parens: true);

    // (((1 > 2) OR ((3 < 4))) AND (5 < 6))
    let and_clause = binary_expr!("AND", boxed_expr!(or_clause), boxed_expr!(expr56), parens: true);

    // ((((1 > 2) OR ((3 < 4))) AND (5 < 6)) OR (7 < 8))
    let cond_expr = binary_expr!("OR", boxed_expr!(and_clause), boxed_expr!(expr78), parens: true);

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! {
            make_stmt!("A", int_lit_expr!(10))
        }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn parse_if_stmt_or_clauses() {
    let code = r#"
        IF 1 > 2 OR 3 < 4 [
            MAKE A = 10
        ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let lexpr = binary_expr!(">", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));
    let rexpr = binary_expr!("<", boxed_int_lit_expr!(3), boxed_int_lit_expr!(4));
    let cond_expr = binary_expr!("OR", boxed_expr!(lexpr), boxed_expr!(rexpr));

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! {
            make_stmt!("A", int_lit_expr!(10))
        }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn parse_if_stmt_with_else() {
    let code = r#"
        IF 1 < 2 [MAKE A = 1] [MAKE B = 2]
    "#;
    let actual = TytleParser.parse(code).unwrap();

    let cond_expr = binary_expr!("<", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! { make_stmt!("A", int_lit_expr!(1))  },
        when_false: block_stmt! { make_stmt!("B", int_lit_expr!(2)) }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn parse_repeat_stmt() {
    let code = r#"
        REPEAT 1 + 2 [
            MAKE A = 3
            MAKE B = 4
        ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let count_expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let block = block_stmt! {
        make_stmt!("A", int_lit_expr!(3)),
        make_stmt!("B", int_lit_expr!(4))
    };

    let expected = ast! {
        repeat_stmt! { count_expr, block }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_proc_with_empty_block() {
    let code = r#"
        TO MYPROC()
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            returns: UNIT,
            body: block_stmt! { }
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_proc_stmt_without_params_with_implicit_return_type() {
    let code = r#"
        TO MYPROC()
            MAKELOCAL A = 3
            MAKELOCAL B = 4
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let block = block_stmt! {
        make_local_stmt!("A", int_lit_expr!(3)),
        make_local_stmt!("B", int_lit_expr!(4))
    };

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            returns: UNIT,
            body: block
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_proc_stmt_without_params_with_explicit_return_type() {
    let code = r#"
        TO MYPROC() : BOOL
            MAKELOCAL A = 3
            MAKELOCAL B = 4
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let block = block_stmt! {
        make_local_stmt!("A", int_lit_expr!(3)),
        make_local_stmt!("B", int_lit_expr!(4))
    };

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            returns: BOOL,
            body: block
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_proc_stmt_with_params_and_explicit_return_value() {
    let code = r#"
        TO MYPROC(A: INT, B: STR) : INT
            MAKELOCAL C = 10
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let block = block_stmt! {
        make_local_stmt!("C", int_lit_expr!(10))
    };

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [proc_param!("A", "INT"), proc_param!("B", "STR")],
            returns: INT,
            body: block
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_return_stmt_with_expr() {
    let code = r#"
        TO MYPROC() : INT
            RETURN 10
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            returns: INT,
            body: block_stmt! {
                ret_stmt! { int_lit_expr!(10) }
            }
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_return_stmt_without_expr() {
    let code = r#"
        TO MYPROC()
            HALT
        END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            returns: UNIT,
            body: block_stmt! {
                halt_stmt!()
            }
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_xcor() {
    let actual = TytleParser.parse("XCOR").unwrap();
    let expected = ast! { command_stmt!(XCOR) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_ycor() {
    let actual = TytleParser.parse("YCOR").unwrap();
    let expected = ast! { command_stmt!(YCOR) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_pen_up() {
    let actual = TytleParser.parse("PENUP").unwrap();
    let expected = ast! { command_stmt!(PENUP) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_pen_down() {
    let actual = TytleParser.parse("PENDOWN").unwrap();
    let expected = ast! { command_stmt!(PENDOWN) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_show_turtle() {
    let actual = TytleParser.parse("SHOWTURTLE").unwrap();
    let expected = ast! { command_stmt!(SHOWTURTLE) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_hide_turtle() {
    let actual = TytleParser.parse("HIDETURTLE").unwrap();
    let expected = ast! { command_stmt!(HIDETURTLE) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_pen_erase() {
    let actual = TytleParser.parse("PENERASE").unwrap();
    let expected = ast! { command_stmt!(PENERASE) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_clean() {
    let actual = TytleParser.parse("CLEAN").unwrap();
    let expected = ast! { command_stmt!(CLEAN) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_clear_screen() {
    let actual = TytleParser.parse("CLEARSCREEN").unwrap();
    let expected = ast! { command_stmt!(CLEARSCREEN) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_set_pen_color() {
    let actual = TytleParser.parse("SETPENCOLOR").unwrap();
    let expected = ast! { command_stmt!(SETPENCOLOR) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_set_background_color() {
    let actual = TytleParser.parse("SETBACKGROUND").unwrap();
    let expected = ast! { command_stmt!(SETBACKGROUND) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_wait() {
    let actual = TytleParser.parse("WAIT").unwrap();
    let expected = ast! { command_stmt!(WAIT) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_command_stop() {
    let actual = TytleParser.parse("STOP").unwrap();
    let expected = ast! { command_stmt!(STOP) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_error_proc_param_missing_colon() {
    let code = r#"
        TO MYPROC(X: INT,  Y INT) : BOOL
        END
    "#;

    let expected = ParseError::MissingColon;

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_variable_must_not_contain_lowercase_letters() {
    let code = "MAKE myvar=1";

    let expected = ParseError::InvalidIdentifierDeclaration(
        "All characters must be capital, digit or `_` (got `myvar`)".to_string(),
    );

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_variable_must_not_begin_with_a_digit() {
    let code = "MAKE 2MYVAR=1";

    let expected = ParseError::InvalidIdentifierDeclaration(
        "Variable name isn't allowed to begin with a digit (got `2MYVAR`)".to_string(),
    );

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_param_must_not_begin_with_a_digit() {
    let code = r#"
        TO MYPROC(2MYVAR: INT)
        END
    "#;

    let expected = ParseError::InvalidIdentifierDeclaration(
        "Variable name isn't allowed to begin with a digit (got `2MYVAR`)".to_string(),
    );

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_param_must_not_contain_lowercase_letters() {
    let code = r#"
        TO MYPROC(myvar: INT)
        END
    "#;

    let expected = ParseError::InvalidIdentifierDeclaration(
        "All characters must be capital, digit or `_` (got `myvar`)".to_string(),
    );

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_missing_colon_before_return_type() {
    let code = r#"
        TO MYPROC(MYVAR: INT) INT
        END
    "#;

    let expected = ParseError::MissingColon;

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_missing_return_type() {
    let code = r#"
        TO MYPROC(MYVAR: INT) :
        END
    "#;

    let expected = ParseError::MissingProcReturnType;

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_invalid_param_type() {
    let code = r#"
        TO MYPROC(MYVAR: INTEGER)
        END
    "#;

    let expected = ParseError::InvalidDataType("INTEGER".to_string());

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_invalid_return_type() {
    let code = r#"
        TO MYPROC(MYVAR: INT) : STRING
        END
    "#;

    let expected = ParseError::InvalidDataType("STRING".to_string());

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_proc_param_cannot_be_unit() {
    let code = r#"
            TO MYPROC(A: UNIT)
            END
        "#;

    let expected = ParseError::InvalidDataType("UNIT".to_string());

    assert_parse_err!(expected, code);
}

#[test]
fn parse_error_trap_is_a_reserved_keyword() {
    assert_reserved_word!("TRAP");
}

#[test]
fn parse_error_print_is_a_reserved_keyword() {
    assert_reserved_word!("PRINT");
}

#[test]
fn parse_error_true_is_a_reserved_keyword() {
    assert_reserved_word!("TRUE");
}

#[test]
fn parse_error_false_is_a_reserved_keyword() {
    assert_reserved_word!("FALSE");
}

#[test]
fn parse_error_if_is_a_reserved_keyword() {
    assert_reserved_word!("IF");
}

#[test]
fn parse_error_repeat_is_a_reserved_keyword() {
    assert_reserved_word!("REPEAT");
}

#[test]
fn parse_error_makeglobal_is_a_reserved_keyword() {
    assert_reserved_word!("MAKEGLOBAL");
}

#[test]
fn parse_error_makelocal_is_a_reserved_keyword() {
    assert_reserved_word!("MAKELOCAL");
}

#[test]
fn parse_error_make_is_a_reserved_keyword() {
    assert_reserved_word!("MAKE");
}

#[test]
fn parse_error_to_is_a_reserved_keyword() {
    assert_reserved_word!("TO");
}

#[test]
fn parse_error_end_is_a_reserved_keyword() {
    assert_reserved_word!("END");
}

#[test]
fn parse_error_return_is_a_reserved_keyword() {
    assert_reserved_word!("RETURN");
}

#[test]
fn parse_error_halt_is_a_reserved_keyword() {
    assert_reserved_word!("HALT");
}

#[test]
fn parse_error_wait_is_a_reserved_keyword() {
    assert_reserved_word!("WAIT");
}

#[test]
fn parse_error_setx_is_a_reserved_keyword() {
    assert_reserved_word!("SETX");
}

#[test]
fn parse_error_sety_is_a_reserved_keyword() {
    assert_reserved_word!("SETY");
}

#[test]
fn parse_error_xcor_is_a_reserved_keyword() {
    assert_reserved_word!("XCOR");
}

#[test]
fn parse_error_ycor_is_a_reserved_keyword() {
    assert_reserved_word!("YCOR");
}

#[test]
fn parse_error_showturtle_is_a_reserved_keyword() {
    assert_reserved_word!("SHOWTURTLE");
}

#[test]
fn parse_error_hideturtle_is_a_reserved_keyword() {
    assert_reserved_word!("HIDETURTLE");
}

#[test]
fn parse_error_penup_is_a_reserved_keyword() {
    assert_reserved_word!("PENUP");
}

#[test]
fn parse_error_pendown_is_a_reserved_keyword() {
    assert_reserved_word!("PENDOWN");
}

#[test]
fn parse_error_penerase_is_a_reserved_keyword() {
    assert_reserved_word!("PENERASE");
}

#[test]
fn parse_error_setpencolor_is_a_reserved_keyword() {
    assert_reserved_word!("SETPENCOLOR");
}

#[test]
fn parse_error_setbackground_is_a_reserved_keyword() {
    assert_reserved_word!("SETBACKGROUND");
}

#[test]
fn parse_error_clean_is_a_reserved_keyword() {
    assert_reserved_word!("CLEAN");
}

#[test]
fn parse_error_clearscreen_is_a_reserved_keyword() {
    assert_reserved_word!("CLEARSCREEN");
}

#[test]
fn parse_error_and_is_a_reserved_keyword() {
    assert_invalid_identifier!("AND");
}

#[test]
fn parse_error_or_is_a_reserved_keyword() {
    assert_invalid_identifier!("OR");
}

#[test]
fn parse_error_not_is_a_reserved_keyword() {
    assert_invalid_identifier!("NOT");
}
