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
#[ignore]
pub fn interpreter_forward_const_repeat_const_times() {}

#[test]
#[ignore]
pub fn interpreter_if_true_bool_lit_cond_expr() {}

#[test]
#[ignore]
pub fn interpreter_if_else_false_bool_lit_cond_expr() {}

#[test]
#[ignore]
pub fn interpreter_xcor() {}

#[test]
#[ignore]
pub fn interpreter_ycor() {}

#[test]
#[ignore]
pub fn interpreter_pen_up() {}

#[test]
#[ignore]
pub fn interpreter_pen_erase() {}

#[test]
#[ignore]
pub fn interpreter_clear() {}

pub fn interpreter_clear_screen() {}

#[test]
#[ignore]
pub fn interpreter_set_pen_color() {}

#[test]
#[ignore]
pub fn interpreter_set_bg_color() {}

#[test]
#[ignore]
pub fn interpreter_show_turtle() {}

#[test]
#[ignore]
pub fn interpreter_hide_turtle() {}

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
