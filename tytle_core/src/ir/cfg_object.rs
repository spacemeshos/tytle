use crate::ast::semantic::SymbolId;
use crate::ir::{CfgGraph, CfgNodeId};
use std::collections::HashMap;

pub struct CfgObject {
    pub graph: CfgGraph,
    pub jmp_table: HashMap<CfgNodeId, SymbolId>,
}
