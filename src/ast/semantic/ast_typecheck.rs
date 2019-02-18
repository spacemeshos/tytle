use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

pub struct AstTypeCheck<'a> {
    sym_table: &'a SymbolTable,
}

impl<'a> AstTypeCheck<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self { sym_table }
    }

    pub fn check(&mut self) {
        //
    }
}

impl<'a> AstWalker<'a> for AstTypeCheck<'a> {}
