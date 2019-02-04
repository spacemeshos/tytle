#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub global: bool,
    pub name: String,
    pub reference: Option<u64>,
    pub resolved_type: Option<VariableType>,
}

impl Variable {
    pub fn build_global(name: &str) -> Self {
        Self::build(name, true)
    }

    pub fn build_local(name: &str) -> Self {
        Self::build(name, false)
    }

    pub fn build(name: &str, global: bool) -> Self {
        Self {
            global,
            name: name.to_string(),
            reference: None,
            resolved_type: None,
        }
    }

    pub fn set_reference(&mut self, reference: u64) {
        self.reference = Some(reference);
    }

    pub fn set_resolved_type(&mut self, rt: VariableType) {
        self.resolved_type = Some(rt);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableType {
    Int,
    Str,
}
