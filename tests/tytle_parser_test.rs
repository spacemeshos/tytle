#[macro_use]
extern crate tytle;

use tytle::ast::{expression::*, statement::*};
use tytle::parser::{ParseError, Parser, TytleParser};

#[test]
fn nop_stmt() {
    let actual = TytleParser.parse("").unwrap();
    let expected = ast! { eof!() };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward() {
    let actual = TytleParser.parse("FORWARD 20").unwrap();
    let expected = ast! { direct_lit_expr!(FORWARD, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_backward() {
    let actual = TytleParser.parse("BACKWARD 20").unwrap();
    let expected = ast! { direct_lit_expr!(BACKWARD, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_left() {
    let actual = TytleParser.parse("LEFT 20").unwrap();
    let expected = ast! { direct_lit_expr!(LEFT, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_right() {
    let actual = TytleParser.parse("RIGHT 20").unwrap();
    let expected = ast! { direct_lit_expr!(RIGHT, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_setx() {
    let actual = TytleParser.parse("SETX 20").unwrap();
    let expected = ast! { direct_lit_expr!(SETX, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_sety() {
    let actual = TytleParser.parse("SETY 20").unwrap();
    let expected = ast! { direct_lit_expr!(SETY, 20) };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_no_empty_lines() {
    let code = r#"
    FORWARD 10
    RIGHT 20
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        direct_lit_expr!(FORWARD, 10),
        direct_lit_expr!(RIGHT, 20)
    };

    assert_eq!(actual, expected);
}

#[test]
fn direction_forward_and_then_backward_with_empty_lines() {
    let code = r#"

    FORWARD 10

    RIGHT 20

    "#;

    let actual = TytleParser.parse(code).unwrap();

    let expected = ast! {
        direct_lit_expr!(FORWARD, 10),
        direct_lit_expr!(RIGHT, 20)
    };

    assert_eq!(actual, expected);
}

#[test]
fn expr_integer_surrounded_by_parentheses() {
    let actual = TytleParser.parse("FORWARD (10)").unwrap();
    let expected = ast! { direct_lit_expr!(FORWARD, 10) };

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_with_spaces() {
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

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_integers_without_spaces() {
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

    assert_eq!(actual, expected);
}

#[test]
fn expr_add_and_mul_integers() {
    let actual = TytleParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

    let clause1 = binary_expr!("*", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));
    let clause2 = binary_expr!("*", boxed_int_lit_expr!(3), boxed_int_lit_expr!(4));
    let expr = binary_expr!("+", Box::new(clause1), Box::new(clause2));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mul_integers_without_spaces() {
    let actual = TytleParser.parse("FORWARD 1 * 2").unwrap();

    let expr = binary_expr!("*", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(actual, expected);
}

#[test]
fn expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
    let actual = TytleParser.parse("FORWARD (1*1 + 2) * (3*3 + 4)").unwrap();

    let ones_mul = binary_expr!("*", boxed_int_lit_expr!(1), boxed_int_lit_expr!(1));
    let three_mul = binary_expr!("*", boxed_int_lit_expr!(3), boxed_int_lit_expr!(3));

    let add_1_2 = binary_expr!("+", Box::new(ones_mul), boxed_int_lit_expr!(2));
    let add_3_4 = binary_expr!("+", Box::new(three_mul), boxed_int_lit_expr!(4));

    let expr = binary_expr!("*", Box::new(add_1_2), Box::new(add_3_4));

    let expected = ast! { direct_stmt!(FORWARD, expr) };

    assert_eq!(actual, expected);
}

#[test]
fn expr_proc_call() {
    let actual = TytleParser
        .parse("FORWARD FOO(10, :X + 1, BAR(2, 3))")
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

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKE \"MYVAR = 2").unwrap();

    let expected = ast! {
        make_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_a_string() {
    let actual = TytleParser.parse("MAKE \"MYVAR = \"Hello").unwrap();

    let expected = ast! {
        make_stmt!("MYVAR", str_lit_expr!("Hello"))
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_expr() {
    let actual = TytleParser.parse("MAKE \"MYVAR = 1 + 2").unwrap();

    let expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let expected = ast! {
        make_stmt!("MYVAR", expr)
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_variable_assign_an_expr_containing_another_var() {
    let actual = TytleParser.parse("MAKE \"A = :B + 2").unwrap();

    let expr = binary_expr!("+", boxed_var_lit_expr!("B"), boxed_int_lit_expr!(2));

    let expected = ast! {
        make_stmt!("A", expr)
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_global_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKEGLOBAL \"MYVAR = 2").unwrap();

    let expected = ast! {
        make_global_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(actual, expected);
}

#[test]
fn make_local_variable_assign_an_integer() {
    let actual = TytleParser.parse("MAKELOCAL \"MYVAR = 2").unwrap();

    let expected = ast! {
        make_local_stmt!("MYVAR", int_lit_expr!(2))
    };

    assert_eq!(actual, expected);
}

#[test]
fn if_stmt_without_else() {
    let code = r#"
    IF 1 + 2 [
        MAKE "A = 3
        MAKE "B = 4
    ]
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let cond_expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

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
fn if_stmt_with_else() {
    let code = r#"
    IF 1 + 2 [MAKE "A = 1] [MAKE "B = 2]
    "#;
    let actual = TytleParser.parse(code).unwrap();

    let cond_expr = binary_expr!("+", boxed_int_lit_expr!(1), boxed_int_lit_expr!(2));

    let if_stmt = if_stmt! {
        cond: cond_expr,
        when_true: block_stmt! { make_stmt!("A", int_lit_expr!(1))  },
        when_false: block_stmt! { make_stmt!("B", int_lit_expr!(2)) }
    };

    let expected = ast! { if_stmt };

    assert_eq!(expected, actual);
}

#[test]
fn repeat_stmt() {
    let code = r#"
    REPEAT 1 + 2 [
        MAKE "A = 3
        MAKE "B = 4
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
fn procedure_stmt_without_params() {
    let code = r#"
    TO MYPROC
        MAKE "A = 3
        MAKE "B = 4
    END
    "#;
    let actual = TytleParser.parse(code).unwrap();

    let block = block_stmt! {
        make_stmt!("A", int_lit_expr!(3)),
        make_stmt!("B", int_lit_expr!(4))
    };

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [],
            body: block
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn procedure_stmt_with_params() {
    let code = r#"
    TO MYPROC :A :B
        MAKE "C = 10
    END
    "#;

    let actual = TytleParser.parse(code).unwrap();

    let block = block_stmt! {
        make_stmt!("C", int_lit_expr!(10))
    };

    let expected = ast! {
        proc_stmt! {
            name: "MYPROC",
            params: [proc_param!("A"), proc_param!("B")],
            body: block
        }
    };

    assert_eq!(expected, actual);
}

#[test]
fn command_xcor() {
    let actual = TytleParser.parse("XCOR").unwrap();
    let expected = ast! { command_stmt!(XCOR) };

    assert_eq!(expected, actual);
}

#[test]
fn command_ycor() {
    let actual = TytleParser.parse("YCOR").unwrap();
    let expected = ast! { command_stmt!(YCOR) };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_up() {
    let actual = TytleParser.parse("PENUP").unwrap();
    let expected = ast! { command_stmt!(PENUP) };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_down() {
    let actual = TytleParser.parse("PENDOWN").unwrap();
    let expected = ast! { command_stmt!(PENDOWN) };

    assert_eq!(expected, actual);
}

#[test]
fn command_show_turtle() {
    let actual = TytleParser.parse("SHOWTURTLE").unwrap();
    let expected = ast! { command_stmt!(SHOWTURTLE) };

    assert_eq!(expected, actual);
}

#[test]
fn command_hide_turtle() {
    let actual = TytleParser.parse("HIDETURTLE").unwrap();
    let expected = ast! { command_stmt!(HIDETURTLE) };

    assert_eq!(expected, actual);
}

#[test]
fn command_pen_erase() {
    let actual = TytleParser.parse("PENERASE").unwrap();
    let expected = ast! { command_stmt!(PENERASE) };

    assert_eq!(expected, actual);
}

#[test]
fn command_clean() {
    let actual = TytleParser.parse("CLEAN").unwrap();
    let expected = ast! { command_stmt!(CLEAN) };

    assert_eq!(expected, actual);
}

#[test]
fn command_clear_screen() {
    let actual = TytleParser.parse("CLEARSCREEN").unwrap();
    let expected = ast! { command_stmt!(CLEARSCREEN) };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_pen_color() {
    let actual = TytleParser.parse("SETPENCOLOR").unwrap();
    let expected = ast! { command_stmt!(SETPENCOLOR) };

    assert_eq!(expected, actual);
}

#[test]
fn command_set_background_color() {
    let actual = TytleParser.parse("SETBACKGROUND").unwrap();
    let expected = ast! { command_stmt!(SETBACKGROUND) };

    assert_eq!(expected, actual);
}

#[test]
fn command_wait() {
    let actual = TytleParser.parse("WAIT").unwrap();
    let expected = ast! { command_stmt!(WAIT) };

    assert_eq!(expected, actual);
}

#[test]
fn command_stop() {
    let actual = TytleParser.parse("STOP").unwrap();
    let expected = ast! { command_stmt!(STOP) };

    assert_eq!(expected, actual);
}

#[test]
fn parse_error_invalid_proc_param() {
    let code = r#"
    TO MYPROC :X Y
    END
    "#;

    let expected = Err(ParseError::InvalidProcParam {
        param: "Y".to_string(),
    });
    let actual = TytleParser.parse(code);

    assert_eq!(expected, actual);
}

#[test]
fn parse_error_make_variable_must_be_prefixed_with_quotation_marks() {
    let expected = Err(ParseError::Custom {
        message: "Invalid `MAKE` expression: A. Variable should be prefixed with `\"`".to_string(),
    });

    let actual = TytleParser.parse("MAKE A=1");

    assert_eq!(expected, actual);
}
