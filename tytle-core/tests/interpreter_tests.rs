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
    intr.exec_code();

    assert_eq!((0, 10), host.xycors());
}

#[test]
pub fn interpreter_backward_int_lit_expr() {
    let code = r#"
        FORWARD 10
        BACKWARD 5
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 5), host.xycors());
}

#[test]
pub fn interpreter_ycor_minimum_is_zero() {
    let code = r#"
        FORWARD 10
        BACKWARD 20
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 0), host.xycors());
}

#[test]
pub fn interpreter_right_int_lit_expr() {
    let code = "RIGHT 10";

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((10, 0), host.xycors());
}

#[test]
pub fn interpreter_left_int_lit_expr() {
    let code = r#"
        RIGHT 10
        LEFT 5
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((5, 0), host.xycors());
}

#[test]
pub fn interpreter_xcor_minimum_is_zero() {
    let code = r#"
        RIGHT 10
        LEFT 20
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 0), host.xycors());
}

#[test]
pub fn interpreter_setx_int_lit_expr() {
    let code = "SETX 10";

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((10, 0), host.xycors());
}

#[test]
pub fn interpreter_sety_int_lit_expr() {
    let code = "SETY 10";

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 10), host.xycors());
}

#[test]
pub fn interpreter_forward_one_var_expr() {
    let code = r#"
       MAKEGLOBAL X = 1 + 2
       FORWARD X * X
    "#;

    setup_interpreter!(code, env, cfg, host, intr);

    intr.exec_code();

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
    intr.exec_code();

    assert_eq!((0, 30), host.xycors());
}

#[test]
pub fn interpreter_forward_const_repeat_const_times() {
    let code = r#"
        REPEAT 3 [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    // intr.exec_code();

    // assert_eq!((0, 15), host.xycors());
}

#[test]
pub fn interpreter_if_true_bool_lit_cond_expr() {
    let code = r#"
        IF 1 < 2 [FORWARD 5]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 5), host.xycors());
}

#[test]
pub fn interpreter_if_else_false_bool_lit_cond_expr() {
    let code = r#"
        IF 1 > 2 [FORWARD 5] [FORWARD 7]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!((0, 7), host.xycors());
}

#[test]
pub fn interpreter_xcor() {
    let code = r#"
        RIGHT 20
        XCOR
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["XCOR = 20".to_string()], host.get_log());
}

#[test]
pub fn interpreter_ycor() {
    let code = r#"
        FORWARD 30
        YCOR
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["YCOR = 30".to_string()], host.get_log());
}

#[test]
pub fn interpreter_pen_up() {
    let code = r#"
        PENUP
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["PENUP".to_string()], host.get_log());
}

#[test]
pub fn interpreter_pen_erase() {
    let code = r#"
        PENERASE
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["PENERASE".to_string()], host.get_log());
}

#[test]
pub fn interpreter_clear() {
    let code = r#"
         CLEAN
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["CLEAN".to_string()], host.get_log());
}

#[test]
pub fn interpreter_clear_screen() {
    let code = r#"
         CLEARSCREEN
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["CLEARSCREEN".to_string()], host.get_log());
}

#[test]
#[ignore]
pub fn interpreter_set_pen_color() {
    let code = r#"
         SETPENCOLOR [255 255 255]
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();
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
    intr.exec_code();

    assert_eq!(vec!["SHOWTURTLE".to_string()], host.get_log());
    assert!(host.get_turtle().is_visible());
}

#[test]
pub fn interpreter_hide_turtle() {
    let code = r#"
         HIDETURTLE
    "#;

    setup_interpreter!(code, env, cfg, host, intr);
    intr.exec_code();

    assert_eq!(vec!["HIDETURTLE".to_string()], host.get_log());
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
