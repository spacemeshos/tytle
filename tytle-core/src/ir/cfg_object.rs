use crate::ir::{CfgGraph, CfgNodeId};
use std::collections::HashMap;

pub struct CfgObject {
    pub graph: CfgGraph,
    pub jmp_table: HashMap<CfgNodeId, u64>,
}
