use crate::ir::{CfgEdge, CfgInstruction, CfgJumpType, CfgNodeId};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct CfgNode {
    pub id: CfgNodeId,
    pub insts: Vec<CfgInstruction>,
    pub incoming: HashSet<CfgEdge>,
    pub outgoing: HashSet<CfgEdge>,
}

impl CfgNode {
    pub fn new(id: CfgNodeId) -> Self {
        Self {
            id,
            insts: Vec::new(),
            incoming: Default::default(),
            outgoing: Default::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.insts.is_empty()
    }

    pub fn has_outgoing_edges(&self) -> bool {
        self.outgoing.len() > 0
    }

    pub fn is_orphan(&self) -> bool {
        self.incoming.len() == 0 && self.outgoing.len() == 0
    }

    pub fn ends_with_return(&self) -> bool {
        if self.insts.is_empty() {
            return false;
        }

        let last_inst: &CfgInstruction = self.insts.last().unwrap();

        if let CfgInstruction::Return = last_inst {
            return true;
        } else {
            return false;
        }
    }

    pub fn append_inst(&mut self, inst: CfgInstruction) {
        self.insts.push(inst);
    }

    pub fn add_outgoing_edge(&mut self, dst_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.outgoing.insert(CfgEdge {
            node_id: dst_node_id,
            jmp_type,
        });
    }

    pub fn add_incoming_edge(&mut self, src_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.incoming.insert(CfgEdge {
            node_id: src_node_id,
            jmp_type,
        });
    }
}
