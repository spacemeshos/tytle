use std::collections::HashMap;

pub type VarRef = usize;

pub enum Var {
    Global(GlobalVar),
    Local(LocalVar),
}

pub struct GlobalVar {
    pub offset: usize,
    pub data: VarData,
}

pub struct LocalVar {
    pub index: usize,
    pub data: VarData,
}

pub enum VarData {
    Int,
    Bool,
    Str,
}

pub struct VarsRefs {
    pub next_ref: VarRef,
    pub vars: HashMap<VarRef, Var>,
}

impl VarsRefs {
    pub fn new() -> Self {
        Self {
            next_ref: 0,
            vars: HashMap::new(),
        }
    }

    pub fn store_var(&mut self, var: Var) -> VarRef {
        self.vars.insert(self.next_ref, var);

        self.next_ref += 1;
        self.next_ref - 1
    }

    pub fn lookup_var(&self, var_ref: VarRef) -> Var {
        unimplemented!()
    }
}
