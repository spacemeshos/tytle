#[macro_use]
extern crate tytle;

#[macro_use]
extern crate maplit;

use tytle::ast::semantic::*;
use tytle::ast::statement::*;
use tytle::ir::*;
use tytle::parser::{Parser, TytleParser};

#[test]
fn compile_cfg_graph_bool_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Bool(true), bool_ins!(true));
    assert_eq!(CfgInstruction::Bool(false), bool_ins!(false));
}

#[test]
fn compile_cfg_graph_int_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Int(10), int_ins!(10));
    assert_eq!(CfgInstruction::Int(20), int_ins!(20));
}

#[test]
fn compile_cfg_graph_str_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Str("Hello".to_string()), str_ins!("Hello"));
    assert_eq!(CfgInstruction::Str("World".to_string()), str_ins!("World"));
}

#[test]
fn compile_cfg_graph_add_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Add, add_ins!());
}

#[test]
fn compile_cfg_graph_mul_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Mul, mul_ins!());
}

#[test]
fn compile_cfg_graph_not_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Not, not_ins!());
}

#[test]
fn compile_cfg_graph_and_ins_macro_sanity() {
    assert_eq!(CfgInstruction::And, and_ins!());
}

#[test]
fn compile_cfg_graph_or_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Or, or_ins!());
}

#[test]
fn compile_cfg_graph_gt_ins_macro_sanity() {
    assert_eq!(CfgInstruction::GT, gt_ins!());
}

#[test]
fn compile_cfg_graph_lt_ins_macro_sanity() {
    assert_eq!(CfgInstruction::LT, lt_ins!());
}

#[test]
fn compile_cfg_graph_load_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Load(100), load_ins!(100));
    assert_eq!(CfgInstruction::Load(200), load_ins!(200));
}

#[test]
fn compile_cfg_graph_store_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Store(100), store_ins!(100));
    assert_eq!(CfgInstruction::Store(200), store_ins!(200));
}

#[test]
fn compile_cfg_graph_cmd_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Command(Command::PenUp), cmd_ins!(PENUP));
    assert_eq!(CfgInstruction::Command(Command::PenDown), cmd_ins!(PENDOWN));
}

#[test]
fn compile_cfg_graph_return_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Return, ret_ins!());
}

#[test]
fn compile_cfg_graph_call_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Call(10), call_ins!(10));
}

#[test]
fn compile_cfg_graph_ret_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Return, ret_ins!());
}

#[test]
fn compile_cfg_graph_direct_ins_macro_sanity() {
    assert_eq!(
        CfgInstruction::Direction(Direction::Left),
        direct_ins!(LEFT)
    );
    assert_eq!(
        CfgInstruction::Direction(Direction::Right),
        direct_ins!(RIGHT)
    );
}

#[test]
fn compile_cfg_graph_node_insts_macro_sanity() {
    let actual = cfg_graph! {
        node!(1,
            int_ins!(10),
            int_ins!(20),
            add_ins!()
        )
    };

    let mut expected = CfgGraph::new();
    let node = expected.current_node_mut();

    node.append_inst(CfgInstruction::Int(10));
    node.append_inst(CfgInstruction::Int(20));
    node.append_inst(CfgInstruction::Add);

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_edge_insts_macro_sanity() {
    let actual = cfg_graph! {
        node!(1, int_ins!(10)),
        node!(2, int_ins!(20)),
        edge_true_jmp!(1, 2)
    };

    let mut expected = CfgGraph::new();
    let node0 = expected.current_node_mut();
    node0.append_inst(CfgInstruction::Int(10));

    expected.new_node();
    let node1 = expected.current_node_mut();
    node1.append_inst(CfgInstruction::Int(20));

    expected.add_edge(1, 2, CfgJumpType::WhenTrue);

    assert_eq!(expected, actual);
}

#[test]
#[ignore]
fn compile_cfg_graph_empty_program() {
    let actual = compile_cfg_graph!("");
    let expected = cfg_graph! {};

    assert_eq!(expected, actual)
}

#[test]
fn compile_cfg_graph_make_global_assign_int_expr() {
    let code = r#"
        MAKEGLOBAL A = (1 + 2) * 5
    "#;

    let actual = compile_cfg_graph!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(1),
            int_ins!(2),
            add_ins!(),
            int_ins!(5),
            mul_ins!(),
            store_ins!(1),
            eoc_ins!()
        )
    };

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_if_stmt_without_else_block() {
    let code = r#"
        MAKEGLOBAL A = 10

        IF 1 < 2 [
            MAKE A = 20
        ]
        MAKEGLOBAL B = A + 1
    "#;

    let actual = compile_cfg_graph!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(10),
            store_ins!(1),
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(2,
            int_ins!(20),
            store_ins!(1)
        ),
        node!(3,
            load_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(2),
            eoc_ins!()
        ),
        edge_true_jmp!(1, 2),
        edge_always_jmp!(2, 3),
        edge_fallback_jmp!(1, 3)
    };

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_if_stmt_with_else_block() {
    let code = r#"
        MAKEGLOBAL A = 10

        IF 1 < 2 [MAKE A = 20] [MAKE A = 30]

        MAKEGLOBAL B = A + 1
    "#;

    let actual = compile_cfg_graph!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(10),
            store_ins!(1),
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(2,
            int_ins!(20),
            store_ins!(1)
        ),
        node!(3,
            int_ins!(30),
            store_ins!(1)
        ),
        node!(4,
            load_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(2),
            eoc_ins!()
        ),
        edge_true_jmp!(1, 2),
        edge_fallback_jmp!(1, 3),
        edge_always_jmp!(2, 4),
        edge_always_jmp!(3, 4)
    };

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_nested_if_stmts() {
    let code = r#"
        MAKEGLOBAL A = 10
        MAKEGLOBAL B = 20

        IF 1 < 2 [
            IF 1 + A < B [
                FORWARD 100
                RIGHT 90
            ]
        ]

        MAKEGLOBAL C = A * B
    "#;

    let actual = compile_cfg_graph!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(10),
            store_ins!(1),   // A = 10
            int_ins!(20),
            store_ins!(2),   // B = 20
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(2,
              int_ins!(1),
              load_ins!(1),
              add_ins!(),    // 1 + A
              load_ins!(2),  // B
              lt_ins!()      // 1 + A < B
        ),
        node!(3,
              int_ins!(100),
              direct_ins!(FORWARD),
              int_ins!(90),
              direct_ins!(RIGHT)
        ),
        node!(4,
              load_ins!(1),
              load_ins!(2),
              mul_ins!(),     // A * B
              store_ins!(3),  // C = A * B
              eoc_ins!()
        ),
        edge_true_jmp!(1, 2),
        edge_fallback_jmp!(1, 4),
        edge_true_jmp!(2, 3),
        edge_fallback_jmp!(2, 4),
        edge_always_jmp!(3, 4)
    };

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_repeat_stmt() {
    let code = r#"
        REPEAT 1 + 1 [
            FORWARD 10
        ]

        MAKEGLOBAL A = 10
    "#;

    let actual = compile_cfg_graph!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(0),
            store_ins!(2),   // TMPVAR_A = 0
            int_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(3),  // TMPVAR_B = 1 + 1
            load_ins!(2),
            load_ins!(3),
            lt_ins!()       // TMPVAR_A < TMPVAR_B
        ),
        node!(2,
            int_ins!(10),
            direct_ins!(FORWARD), // FORWARD 10
            load_ins!(2),
            int_ins!(1),
            add_ins!(),    // TMPVAR_A + 1
            store_ins!(2), // TMPVAR_A = TMPVAR_A + 1
            load_ins!(2),
            load_ins!(3),
            lt_ins!()      // TMPVAR_A < TMPVAR_B
        ),
        node!(3,
            int_ins!(10),
            store_ins!(1),
            eoc_ins!()
        ),
        edge_true_jmp!(2, 2),
        edge_fallback_jmp!(2, 3),
        edge_true_jmp!(1, 2),
        edge_fallback_jmp!(1, 3)
    };

    assert_eq!(expected, actual);
}

#[test]
fn compile_cfg_graph_proc_with_no_external_calls() {
    let code = r#"
        TO MYPROC(): INT
            MAKELOCAL A = 10
            MAKELOCAL B = 20

            RETURN A + B
        END

        MAKEGLOBAL C = 30
    "#;

    let actual = compile_cfg_obj!(code);

    let expected_graph = cfg_graph! {
        node!(1,
            int_ins!(30),
            store_ins!(2), // C = 30
            eoc_ins!()
        ),
        node!(2,
            int_ins!(10),
            store_ins!(3),  // A = 10
            int_ins!(20),
            store_ins!(4),  // B = 20
            load_ins!(3),
            load_ins!(4),
            add_ins!(),     // A + B
            ret_ins!()
        )
    };

    // node `2` represents Procedure with `id = 1`
    let expected_jmp_table = hashmap! { 2 => 1 };

    assert_eq!(expected_graph, actual.graph);
    assert_eq!(expected_jmp_table, actual.jmp_table);
}

#[test]
fn compile_cfg_graph_proc_with_external_calls() {
    let code = r#"
        MYPROC(1, TRUE)

        TO MYPROC(A: INT, B: BOOL): INT
            RETURN 10
        END

        MYPROC(2, FALSE)
    "#;

    let actual = compile_cfg_obj!(code);

    let expected_graph = cfg_graph! {
        node!(1,
            int_ins!(1),
            bool_ins!(true),
            call_ins!(2),  // MYPROC(1, TRUE)
            int_ins!(2),
            bool_ins!(false),
            call_ins!(2),  // MYPROC(2, FALSE)
            eoc_ins!()
        ),
        node!(2,
            int_ins!(10),
            ret_ins!()    // RETURN 10
        )
    };

    let expected_jmp_table = hashmap! { 2 => 1 };

    assert_eq!(expected_graph, actual.graph);
    assert_eq!(expected_jmp_table, actual.jmp_table);
}

#[test]
fn compile_cfg_graph_recursive_procedure() {
    let code = r#"
        TO RECUR_PROC(I: INT, N: INT, ACC: INT): INT
            IF I < N [RETURN RECUR_PROC(I + 1, N, ACC * (I + 1))] [RETURN ACC]
        END
        RECUR_PROC(0, 5, 1)
    "#;

    let actual = compile_cfg_obj!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(0),
            int_ins!(5),
            int_ins!(1),
            call_ins!(2),
            eoc_ins!()
        ),
        node!(2,
            load_ins!(2), // I
            load_ins!(3), // N
            lt_ins!()     // IF I < N
        ),
        node!(3,
            load_ins!(2),
            int_ins!(1),
            add_ins!(),   // I + 1  => ARG #1
            load_ins!(3), // N      => ARG #2
            load_ins!(4), // ACC
            load_ins!(2), // I
            int_ins!(1),
            add_ins!(),   // I + 1
            mul_ins!(),   // ACC * (I + 1)  => ARG #3
            call_ins!(2), // RECUR_PROC(I + 1, N, ACC * (I + 1))
            ret_ins!()    // RETURN RECUR_PROC(I + 1, N, ACC * (I + 1))
        ),
        node!(4,
              load_ins!(4), // ACC
              ret_ins!()    // RETURN ACC
        ),
        node!(5,
            ret_ins!()
        ),
        edge_true_jmp!(2, 3),
        edge_fallback_jmp!(2, 4)
    };

    assert_eq!(expected, actual.graph);
}

#[test]
fn compile_cfg_graph_mutually_exclusive_procedures() {
    let code = r#"
        TO F(A: INT): INT
            10 + G(1)
        END

        TO G(B: INT): INT
            20 * F(2)
        END

        F(1)
    "#;

    let actual = compile_cfg_obj!(code);

    let expected = cfg_graph! {
        node!(1,
            int_ins!(1),
            call_ins!(2),
            eoc_ins!()
        ),
        node!(2,
            int_ins!(10),
            int_ins!(1),
            call_ins!(3),  // G(1)
            add_ins!(),    // 10 + G(1)
            ret_ins!()
        ),
        node!(3,
            int_ins!(20),
            int_ins!(2),
            call_ins!(2),  // F(2)
            mul_ins!(),    // 20 * F(2)
            ret_ins!()
        )
    };

    assert_eq!(expected, actual.graph);
}
