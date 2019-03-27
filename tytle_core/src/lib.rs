#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod vm;

pub mod prelude {
    pub use crate::ast::expression::*;
    pub use crate::ast::semantic::*;
    pub use crate::ast::statement::*;
    pub use crate::ast::*;
    pub use crate::ir::*;
    pub use crate::lexer::*;
    pub use crate::parser::*;
    pub use crate::vm::*;
}
