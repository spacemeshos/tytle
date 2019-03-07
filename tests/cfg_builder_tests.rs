#[macro_use]
extern crate tytle;

// use tytle::ast::semantic::{AstTypeCheck, SymbolTableGenerator};
use tytle::ast::statement::{Command, Direction};
use tytle::ir::*;
// use tytle::parser::{Parser, TytleParser};
//
// macro_rules! prepare_ast {
//     ($code:expr) => {{
//         let mut ast = TytleParser.parse($code).unwrap();
//         let mut generator = SymbolTableGenerator::new();
//         let mut symbol_table = generator.generate(&mut ast).unwrap();
//         let mut checker = AstTypeCheck::new(&mut symbol_table);
//
//         let _ = checker.check(&mut ast);
//         ast
//     }};
// }

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
    assert_eq!(CfgInstruction::Direction(Direction::Left), direct_ins!(LEFT));
    assert_eq!(CfgInstruction::Direction(Direction::Right), direct_ins!(RIGHT));
}
