use crate::ast::semantic::Variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Var(Variable),
}

impl Symbol {
    pub fn name(&self) -> String {
        match self {
            Symbol::Var(ref var) => var.name.clone(),
            _ => unimplemented!()
        }
    }
}
