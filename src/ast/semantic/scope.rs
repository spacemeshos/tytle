use crate::ast::semantic::Variable;
use std::collections::HashMap;

pub struct Scope {
    variables: HashMap<String, Variable>
}
