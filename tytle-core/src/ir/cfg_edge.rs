use crate::ir::{CfgJumpType, CfgNodeId};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CfgEdge {
    pub node_id: CfgNodeId,
    pub jmp_type: CfgJumpType,
}
