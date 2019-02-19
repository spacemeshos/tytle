use crate::ast::expression::ExpressionType;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub global: bool,
    pub name: String,
    pub var_type: Option<ExpressionType>,
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
            var_type: None,
        }
    }

    pub fn set_resolved_type(&mut self, rt: ExpressionType) {
        match self.var_type {
            None => self.var_type = Some(rt),
            Some(ref current_rt) if *current_rt == rt => {}
            _ => panic!("Type mismatch for variable `{}`", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_to_a_variable_the_same_primitive_type_twice() {
        let mut var = Variable::build_global("A");

        var.set_resolved_type(ExpressionType::Int);
        var.set_resolved_type(ExpressionType::Int);

        assert_eq!(Some(ExpressionType::Int), var.var_type);
    }

    #[test]
    #[should_panic(expected = "Type mismatch for variable `A`")]
    fn error_when_variable_type_mismatch() {
        let mut var = Variable::build_global("A");

        var.set_resolved_type(ExpressionType::Int);
        var.set_resolved_type(ExpressionType::Str);
    }
}
