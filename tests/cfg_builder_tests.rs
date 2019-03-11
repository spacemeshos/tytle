#[macro_use]
extern crate tytle;

use tytle::ast::semantic::*;
use tytle::ast::statement::*;
use tytle::ir::*;
use tytle::parser::{Parser, TytleParser};

macro_rules! prepare {
    ($code:expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();
        let generator = SymbolTableGenerator::new();
        let mut env = generator.generate(&mut ast).unwrap();
        let mut checker = AstTypeCheck::new(&mut env);

        let res = checker.check(&mut ast);
        assert!(res.is_ok());

        (ast, env)
    }};
}

#[test]
fn cfg_build_bool_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Bool(true), bool_ins!(true));
    assert_eq!(CfgInstruction::Bool(false), bool_ins!(false));
}

#[test]
fn cfg_build_int_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Int(10), int_ins!(10));
    assert_eq!(CfgInstruction::Int(20), int_ins!(20));
}

#[test]
fn cfg_build_str_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Str("Hello".to_string()), str_ins!("Hello"));
    assert_eq!(CfgInstruction::Str("World".to_string()), str_ins!("World"));
}

#[test]
fn cfg_build_add_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Add, add_ins!());
}

#[test]
fn cfg_build_mul_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Mul, mul_ins!());
}

#[test]
fn cfg_build_not_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Not, not_ins!());
}

#[test]
fn cfg_build_and_ins_macro_sanity() {
    assert_eq!(CfgInstruction::And, and_ins!());
}

#[test]
fn cfg_build_or_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Or, or_ins!());
}

#[test]
fn cfg_build_gt_ins_macro_sanity() {
    assert_eq!(CfgInstruction::GT, gt_ins!());
}

#[test]
fn cfg_build_lt_ins_macro_sanity() {
    assert_eq!(CfgInstruction::LT, lt_ins!());
}

#[test]
fn cfg_build_load_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Load(100), load_ins!(100));
    assert_eq!(CfgInstruction::Load(200), load_ins!(200));
}

#[test]
fn cfg_build_store_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Store(100), store_ins!(100));
    assert_eq!(CfgInstruction::Store(200), store_ins!(200));
}

#[test]
fn cfg_build_cmd_ins_macro_sanity() {
    assert_eq!(CfgInstruction::Command(Command::PenUp), cmd_ins!(PENUP));
    assert_eq!(CfgInstruction::Command(Command::PenDown), cmd_ins!(PENDOWN));
}

#[test]
fn cfg_build_direct_ins_macro_sanity() {
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
fn cfg_build_node_insts_macro_sanity() {
    let actual = cfg_graph! {
        node!(0,
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
fn cfg_build_edge_insts_macro_sanity() {
    let actual = cfg_graph! {
        node!(0, int_ins!(10)),
        node!(1, int_ins!(20)),
        edge_true_jmp!(0, 1)
    };

    let mut expected = CfgGraph::new();
    let node0 = expected.current_node_mut();
    node0.append_inst(CfgInstruction::Int(10));

    expected.new_node();
    let node1 = expected.current_node_mut();
    node1.append_inst(CfgInstruction::Int(20));

    expected.add_edge(0, 1, CfgJumpType::WhenTrue);

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_make_global_assign_int_expr() {
    let code = r#"
        MAKEGLOBAL A = (1 + 2) * 5
    "#;

    let expected = cfg_graph! {
        node!(0,
            int_ins!(1),
            int_ins!(2),
            add_ins!(),
            int_ins!(5),
            mul_ins!(),
            store_ins!(1)
        )
    };

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_if_stmt_without_else_block() {
    let code = r#"
        MAKEGLOBAL A = 10

        IF 1 < 2 [
            MAKE A = 20
        ]
        MAKEGLOBAL B = A + 1
    "#;

    let expected = cfg_graph! {
        node!(0,
            int_ins!(10),
            store_ins!(1),
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(1,
            int_ins!(20),
            store_ins!(1)
        ),
        node!(2,
            load_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(2)
        ),
        edge_true_jmp!(0, 1),
        edge_always_jmp!(1, 2),
        edge_fallback_jmp!(0, 2)
    };

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_if_stmt_with_else_block() {
    let code = r#"
        MAKEGLOBAL A = 10

        IF 1 < 2 [MAKE A = 20] [MAKE A = 30]

        MAKEGLOBAL B = A + 1
    "#;

    let expected = cfg_graph! {
        node!(0,
            int_ins!(10),
            store_ins!(1),
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(1,
            int_ins!(20),
            store_ins!(1)
        ),
        node!(2,
            int_ins!(30),
            store_ins!(1)
        ),
        node!(3,
            load_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(2)
        ),
        edge_true_jmp!(0, 1),
        edge_fallback_jmp!(0, 2),
        edge_always_jmp!(1, 3),
        edge_always_jmp!(2, 3)
    };

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_nested_if_stmts() {
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

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    let expected = cfg_graph! {
        node!(0,
            int_ins!(10),
            store_ins!(1),   // A = 10
            int_ins!(20),
            store_ins!(2),   // B = 20
            int_ins!(1),
            int_ins!(2),
            lt_ins!()
        ),
        node!(1,
              int_ins!(1),
              load_ins!(1),
              add_ins!(),    // 1 + A
              load_ins!(2),  // B
              lt_ins!()      // 1 + A < B
        ),
        node!(2,
              int_ins!(100),
              direct_ins!(FORWARD),
              int_ins!(90),
              direct_ins!(RIGHT)
        ),
        node!(3,
              load_ins!(1),
              load_ins!(2),
              mul_ins!(),     // A * B
              store_ins!(3)   // C = A * B
        ),
        edge_true_jmp!(0, 1),
        edge_fallback_jmp!(0, 3),
        edge_true_jmp!(1, 2),
        edge_fallback_jmp!(1, 3),
        edge_always_jmp!(2, 3)
    };

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_repeat_stmt() {
    let code = r#"
        REPEAT 1 + 1 [
            FORWARD 10
        ]

        MAKEGLOBAL A = 10
    "#;

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    let expected = cfg_graph! {
        node!(0,
            int_ins!(0),
            store_ins!(2),   // TMPVAR_A = 0
            int_ins!(1),
            int_ins!(1),
            add_ins!(),
            store_ins!(3),  // TMPVAR_B = 1 + 1
            load_ins!(2),
            load_ins!(3),
            gt_ins!()       // TMPVAR_A < TMPVAR_B
        ),
        node!(1,
            int_ins!(10),
            direct_ins!(FORWARD), // FORWARD 10
            load_ins!(2),
            int_ins!(1),
            add_ins!(),    // TMPVAR_A + 1
            store_ins!(2), // TMPVAR_A = TMPVAR_A + 1
            load_ins!(2),
            load_ins!(3),
            gt_ins!()      // TMPVAR_A < TMPVAR_B
        ),
        node!(2,
            int_ins!(10),
            store_ins!(1)
        ),
        edge_true_jmp!(1, 1),
        edge_fallback_jmp!(1, 2),
        edge_true_jmp!(0, 1),
        edge_fallback_jmp!(0, 2)
    };

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_proc_with_no_external_calls() {
    let code = r#"
        TO MYPROC(): INT
            MAKELOCAL A = 10
            MAKELOCAL B = 20

            RETURN A + B
        END

        MAKEGLOBAL C = 30
    "#;

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    let expected = cfg_graph! {
        node!(0,
            int_ins!(30),
            store_ins!(2) // C = 30
        ),
        node!(1,
            int_ins!(10),
            store_ins!(3),  // A = 10
            int_ins!(20),
            store_ins!(4),  // B = 20
            load_ins!(3),
            load_ins!(4),
            add_ins!(),     // A + B
            return_ins!()
        )
    };

    assert_eq!(expected, actual);
}

#[test]
fn cfg_build_proc_with_external_calls() {
    let code = r#"
        MYPROC(1, TRUE)

        TO MYPROC(A: INT, B: BOOL): INT
            RETURN 10
        END

        MYPROC(2, FALSE)
    "#;

    let (ast, mut env) = prepare!(code);
    let builder = CfgBuilder::new(&mut env);
    let actual = builder.build(&ast);

    let expected = cfg_graph! {
        node!(0,
            int_ins!(1),
            bool_ins!(true),
            call_ins!(1),  // MYPROC(1, TRUE)
            int_ins!(2),
            bool_ins!(false),
            call_ins!(1)   // MYPROC(2, FALSE)
        ),
        node!(1,
            int_ins!(10),
            return_ins!() // RETURN 10
        )
    };

    assert_eq!(expected, actual);
}
