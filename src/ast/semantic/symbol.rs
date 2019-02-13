use crate::ast::semantic::{Procedure, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Var(Variable),
    Proc(Procedure),
}

impl Symbol {
    fn as_var(&self) -> &Variable {
        if let &Symbol::Var(ref var) = self {
            var
        } else {
            panic!("expected symbol `{}` to be a Variable", self.name());
        }
    }

    fn as_proc(&self) -> &Procedure {
        if let &Symbol::Proc(ref proc) = self {
            proc
        } else {
            panic!("expected symbol `{}` to be a Procedure", self.name());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SymbolKind {
    Var,
    Proc,
}

impl Symbol {
    pub fn name(&self) -> String {
        match self {
            Symbol::Var(ref var) => var.name.to_owned(),
            Symbol::Proc(ref proc) => proc.name.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_var_when_symbol_is_a_variable() {
        let var = Variable::build_global("A");
        let symbol = Symbol::Var(var);

        assert_eq!(*symbol.as_var(), Variable::build_global("A"));
    }

    #[test]
    #[should_panic(expected = "expected symbol `MYPROC` to be a Variable")]
    fn error_as_var_when_symbol_is_not_a_variable() {
        let proc = Procedure::new("MYPROC");
        let symbol = Symbol::Proc(proc);

        symbol.as_var();
    }

    #[test]
    fn as_proc_when_symbol_is_a_procedure() {
        let proc = Procedure::new("MYPROC");
        let symbol = Symbol::Proc(proc);

        assert_eq!(*symbol.as_proc(), Procedure::new("MYPROC"));
    }

    #[test]
    #[should_panic(expected = "expected symbol `A` to be a Procedure")]
    fn error_as_proc_when_symbol_is_not_a_variable() {
        let var = Variable::build_global("A");
        let symbol = Symbol::Var(var);

        symbol.as_proc();
    }
}
