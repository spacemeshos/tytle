extern crate tytle;

use tytle::ast::semantic::*;
use tytle::ir::*;
use tytle::parser::{Parser, TytleParser};
use tytle::vm::*;

#[macro_export]
macro_rules! setup_interpreter {
    ($code: expr, $env: ident, $cfg: ident, $host: ident, $intr: ident) => {
        let mut ast = TytleParser.parse($code).unwrap();
        let generator = SymbolTableGenerator::new();

        let mut $env = generator.generate(&mut ast).unwrap();
        let mut checker = AstTypeCheck::new(&mut $env);

        let res = checker.check(&mut ast);
        assert!(res.is_ok());

        let builder = CfgBuilder::new(&mut $env);
        let $cfg = builder.build(&ast);

        let mut $host = DummyHost::new();
        let mut $intr = Interpreter::new(&$cfg, &$env, &mut $host);
    };
}

#[test]
pub fn interpreter_forward_int_lit_expr() {
    let code = "FORWARD 10";

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 10), host.xycors());
}

#[test]
pub fn interpreter_backward_int_lit_expr() {
    let code = r#"
        FORWARD 10
        BACKWARD 5
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 5), host.xycors());
}

#[test]
pub fn interpreter_ycor_minimum_is_zero() {
    let code = r#"
        FORWARD 10
        BACKWARD 20
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 0), host.xycors());
}

#[test]
pub fn interpreter_right_int_lit_expr() {
    let code = "RIGHT 10";

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((10, 0), host.xycors());
}

#[test]
pub fn interpreter_left_int_lit_expr() {
    let code = r#"
        RIGHT 10
        LEFT 5
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((5, 0), host.xycors());
}

#[test]
pub fn interpreter_xcor_minimum_is_zero() {
    let code = r#"
        RIGHT 10
        LEFT 20
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 0), host.xycors());
}

#[test]
pub fn interpreter_setx_int_lit_expr() {
    let code = "SETX 10";

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((10, 0), host.xycors());
}

#[test]
pub fn interpreter_sety_int_lit_expr() {
    let code = "SETY 10";

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 10), host.xycors());
}

#[test]
pub fn interpreter_print_const_expr() {
    let code = "PRINT 10";

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["10"], host.get_log());
}

#[test]
pub fn interpreter_print_var_expr() {
    let code = r#"
       MAKEGLOBAL X = 10
       PRINT X * X
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["100"], host.get_log());
}

#[test]
pub fn interpreter_forward_one_var_expr() {
    let code = r#"
       MAKEGLOBAL X = 1 + 2
       FORWARD X * X
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 9), host.xycors());
}

#[test]
pub fn interpreter_forward_two_vars_expr() {
    let code = r#"
       MAKEGLOBAL X = 10
       MAKEGLOBAL Y = 20
       FORWARD X + Y
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 30), host.xycors());
}

#[test]
pub fn interpreter_forward_const_repeat_const_times() {
    let code = r#"
        REPEAT 3 [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 15), host.xycors());
}

#[test]
pub fn interpreter_repeat_one_var_expr() {
    let code = r#"
        MAKEGLOBAL X = 2
        REPEAT (X + 1 + 1) [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 20), host.xycors());
}

#[test]
pub fn interpreter_if_true_bool_lit_cond_expr() {
    let code = r#"
        IF 1 < 2 [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 5), host.xycors());
}

#[test]
pub fn interpreter_if_else_false_bool_lit_cond_expr() {
    let code = r#"
        IF 1 > 2 [FORWARD 5] [FORWARD 7]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 7), host.xycors());
}

#[test]
pub fn interpreter_if_cond_var_expr() {
    let code = r#"
        MAKEGLOBAL X = 1
        MAKEGLOBAL Y = X + 1
        IF X < Y [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 5), host.xycors());
}

#[test]
pub fn interpreter_proc_call_with_no_params_and_locals_and_no_return_value() {
    let code = r#"
        TO MYPROC()
            FORWARD 10
            FORWARD 10
            FORWARD 10
        END
        MYPROC()
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 30), host.xycors());
}

#[test]
pub fn interpreter_print_inside_proc() {
    let code = r#"
        TO MYPROC(X: INT)
            PRINT X + 1
        END

        MYPROC(100)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["101"], host.get_log());
}

#[test]
pub fn interpreter_proc_call_with_no_params_and_locals_but_with_a_return_value() {
    let code = r#"
        TO MYPROC(): INT
            RETURN 10
        END

        FORWARD MYPROC()
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 10), host.xycors());
}

#[test]
pub fn interpreter_proc_call_with_params_and_no_additional_locals_and_no_return_value() {
    let code = r#"
        TO GO_FORWARD(X: INT)
            FORWARD X + 10
        END

        GO_FORWARD(10)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 20), host.xycors());
}

#[test]
pub fn interpreter_proc_call_with_params_and_no_additional_locals_but_with_return_value() {
    let code = r#"
        TO ADD10(X: INT): INT
            RETURN X + 10
        END

        FORWARD ADD10(15)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 25), host.xycors());
}

#[test]
pub fn interpreter_proc_call_with_params_and_additional_locals_and_return_value() {
    let code = r#"
        TO DO_CALC(X: INT): INT
            MAKELOCAL Y = 20
            MAKELOCAL Z = 40

            RETURN X + Y + Z
        END

        FORWARD DO_CALC(10)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 70), host.xycors());
}

#[test]
pub fn interpreter_calculating_factorial_recursively() {
    let code = r#"
        TO FACTORIAL(I: INT, N: INT): INT
            IF I + 1 > N [RETURN N][RETURN I * FACTORIAL(I + 1, N)]
        END
        FORWARD FACTORIAL(1, 6)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!((0, 720), host.xycors());
}

#[test]
pub fn interpreter_mutually_recursive_procedures() {
    let code = r#"
        TO F(A: INT): INT
            PRINT A

            IF A > 10 [
                RETURN A
            ][
                RETURN G(A + 2)
            ]
        END

        TO G(B: INT): INT
            PRINT B
            RETURN F(2 * B + 3)
        END

        F(0)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    let expected = vec!["0", "2", "7", "9", "21"];

    assert_eq!(expected, host.get_log());
}

#[test]
pub fn interpreter_stack_overflow() {
    let code = r#"
        TO OVERFLOW(I: INT): INT
            RETURN OVERFLOW(I + 1)
        END
        OVERFLOW(0)
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let res = intr.exec_code();

    assert_eq!(Err(InterpreterException::StackOverflow), res);
}

#[test]
pub fn interpreter_xcor() {
    let code = r#"
        RIGHT 20
        XCOR
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["XCOR = 20"], host.get_log());
}

#[test]
pub fn interpreter_ycor() {
    let code = r#"
        FORWARD 30
        YCOR
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["YCOR = 30"], host.get_log());
}

#[test]
pub fn interpreter_pen_up() {
    let code = r#"
        PENUP
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["PENUP"], host.get_log());
}

#[test]
pub fn interpreter_pen_erase() {
    let code = r#"
        PENERASE
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["PENERASE"], host.get_log());
}

#[test]
pub fn interpreter_clear() {
    let code = r#"
         CLEAN
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["CLEAN"], host.get_log());
}

#[test]
pub fn interpreter_clear_screen() {
    let code = r#"
         CLEARSCREEN
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["CLEARSCREEN"], host.get_log());
}

#[test]
#[ignore]
pub fn interpreter_set_pen_color() {
    let code = r#"
         SETPENCOLOR [255 255 255]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();
}

#[test]
#[ignore]
pub fn interpreter_set_bg_color() {}

#[test]
pub fn interpreter_show_turtle() {
    let code = r#"
         SHOWTURTLE
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["SHOWTURTLE"], host.get_log());
    assert!(host.get_turtle().is_visible());
}

#[test]
pub fn interpreter_hide_turtle() {
    let code = r#"
         HIDETURTLE
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    let _ = intr.exec_code();

    assert_eq!(vec!["HIDETURTLE"], host.get_log());
    assert_eq!(false, host.get_turtle().is_visible());
}

#[test]
#[ignore]
pub fn interpreter_wait_const_expr() {}

#[test]
#[ignore]
pub fn interpreter_wait_var_expr() {}

#[test]
#[ignore]
pub fn interpreter_stop_within_main_proc() {}

#[test]
#[ignore]
pub fn interpreter_stop_within_sub_proc() {}
