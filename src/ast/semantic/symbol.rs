use crate::ast::semantic::{Procedure, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Var(Variable),
    Proc(Procedure),
}

impl Symbol {
    pub fn name(&self) -> String {
        match self {
            Symbol::Var(ref var) => var.name.to_owned(),
            Symbol::Proc(ref proc) => proc.name.to_owned(),
        }
    }
}
