use crate::ast::semantic::PrimitiveType;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub global: bool,
    pub name: String,
    pub reference: Option<u64>,
    pub resolved_type: Option<PrimitiveType>,
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

    pub fn set_resolved_type(&mut self, rt: PrimitiveType) {
        match self.resolved_type {
            None => self.resolved_type = Some(rt),
            Some(ref current_rt) if *current_rt == rt => {}
            _ => panic!(format!("Type mismatch for variable `{}`", self.name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting_to_var_the_same_variable_type_twice() {
        let mut var = Variable::build_global("A");

        var.set_resolved_type(PrimitiveType::Int);
        var.set_resolved_type(PrimitiveType::Int);

        assert_eq!(Some(PrimitiveType::Int), var.resolved_type);
    }

    #[test]
    #[should_panic(expected = "Type mismatch for variable `A`")]
    fn raises_when_variable_type_mismatch() {
        let mut var = Variable::build_global("A");

        var.set_resolved_type(PrimitiveType::Int);
        var.set_resolved_type(PrimitiveType::Str);
    }
}
