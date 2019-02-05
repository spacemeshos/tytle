// #[macro_use]
// extern crate logos;
//
// use logos::ast::expression::*;
// use logos::ast::macros;
// use logos::ast::statement::*;
//
// use logos::ast::semantic::ProgramWalker;
//
// #[test]
// #[ignore]
// // #[should_panic]
// fn expr_undefined_var() {
//     let ast = ast! {
//         make_stmt!("A", var_lit_expr!("B"))
//     };
//
//     dbg!(ast);
// }
//
// #[test]
// #[ignore]
// #[should_panic]
// fn expr_undefined_proc() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn expr_add_type_mismatch() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn expr_mul_type_mismatch() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn if_cond_non_bool_literal() {
//     let ast = ast! {
//         if_stmt! {
//             cond: int_lit_expr!(1),
//             when_true: empty_block!()
//         }
//     };
// }
//
// #[test]
// #[ignore]
// #[should_panic]
// fn if_cond_expr_type_mistmach() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn if_cond_expr_undefined_var() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn make_stmt_global_var_already_exists() {}
//
// #[test]
// #[ignore]
// #[ignore]
// fn proc_name_must_start_with_non_digit() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn proc_duplicate_declaration() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn direct_stmt_expr_must_resolve_to_int() {}
//
// #[test]
// #[ignore]
// #[should_panic]
// fn repeat_stmt_count_expr_must_resolve_to_int() {}
