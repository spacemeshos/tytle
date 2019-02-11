use crate::ast::semantic::{Constant, Procedure, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Var(Variable),
    Proc(Procedure),
}

impl Symbol {
    pub fn name(&self) -> String {
        match self {
            Symbol::Var(ref var) => var.name.clone(),
            _ => unimplemented!(),
        }
    }
}
