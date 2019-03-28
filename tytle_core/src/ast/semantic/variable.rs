use crate::ast::expression::ExpressionType;
use crate::ast::semantic::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub id: SymbolId,
    pub global: bool,
    pub param: bool,
    pub name: String,
    pub var_type: Option<ExpressionType>,

    // for global variables - `index` will be the index within the global variables
    // for local  variables - `index` will be the local index within the enclosing procedure
    pub index: Option<usize>,
}

impl Variable {
    pub fn build_global(name: &str, id: SymbolId) -> Self {
        Self::build(name, true, false, id)
    }

    pub fn build_local(name: &str, id: SymbolId) -> Self {
        Self::build(name, false, false, id)
    }

    pub fn build_param(name: &str, id: SymbolId) -> Self {
        Self::build(name, false, true, id)
    }

    pub fn build(name: &str, global: bool, param: bool, id: SymbolId) -> Self {
        Self {
            id,
            global,
            param,
            name: name.to_string(),
            var_type: None,
            index: None,
        }
    }

    pub fn is_param(&self) -> bool {
        self.param == true
    }

    pub fn set_resolved_type(&mut self, resolved_type: ExpressionType) {
        match self.var_type {
            None => self.var_type = Some(resolved_type),
            Some(ref current_rt) if *current_rt == resolved_type => {}
            _ => panic!("Type mismatch for variable `{}`", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_to_a_variable_the_same_primitive_type_twice() {
        let mut var = Variable::build_global("A", SymbolId(1));

        var.set_resolved_type(ExpressionType::Int);
        var.set_resolved_type(ExpressionType::Int);

        assert_eq!(Some(ExpressionType::Int), var.var_type);
    }

    #[test]
    #[should_panic(expected = "Type mismatch for variable `A`")]
    fn error_when_variable_type_mismatch() {
        let mut var = Variable::build_global("A", SymbolId(1));

        var.set_resolved_type(ExpressionType::Int);
        var.set_resolved_type(ExpressionType::Str);
    }
}
