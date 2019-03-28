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
}
