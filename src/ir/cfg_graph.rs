use crate::ir::CfgInstruction;
use std::collections::HashMap;

pub type CfgNodeId = usize;

#[derive(Debug, Copy, Clone)]
pub enum CfgJumpType {
    WhenTrue,
    WhenFalse,
    Always,
    Fallback,
}

#[derive(Debug, Clone)]
pub struct CfgEdge {
    pub node_id: CfgNodeId,
    pub jmp_type: CfgJumpType,
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    pub id: CfgNodeId,
    pub insts: Vec<CfgInstruction>,
    pub incoming: Vec<CfgEdge>,
    pub outgoing: Vec<CfgEdge>,
}

impl CfgNode {
    pub fn new(id: CfgNodeId) -> Self {
        Self {
            id,
            insts: Vec::new(),
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    pub fn append_inst(&mut self, inst: CfgInstruction) {
        self.insts.push(inst);
    }

    pub fn add_outgoing_edge(&mut self, dst_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.outgoing.push(CfgEdge {
            node_id: dst_node_id,
            jmp_type,
        });
    }

    pub fn add_incoming_edge(&mut self, src_node_id: CfgNodeId, jmp_type: CfgJumpType) {
        self.incoming.push(CfgEdge {
            node_id: src_node_id,
            jmp_type,
        });
    }
}

#[derive(Debug, Clone)]
pub struct CfgGraph {
    next_id: CfgNodeId,
    pub nodes: HashMap<CfgNodeId, CfgNode>,
}

impl CfgGraph {
    pub fn new() -> Self {
        let mut graph = Self {
            nodes: HashMap::new(),
            next_id: 0,
        };

        graph.new_node();

        graph
    }

    pub fn get_node_mut(&mut self, node_id: CfgNodeId) -> &mut CfgNode {
        self.nodes.get_mut(&node_id).unwrap()
    }

    pub fn current_node_mut(&mut self) -> &mut CfgNode {
        self.get_node_mut(self.next_id - 1)
    }

    pub fn new_node(&mut self) -> CfgNodeId {
        let node = CfgNode::new(self.next_id);

        self.nodes.insert(node.id, node);

        self.next_id += 1;

        self.get_current_id()
    }

    pub fn get_current_id(&self) -> CfgNodeId {
        self.get_next_id() - 1
    }

    pub fn get_next_id(&self) -> CfgNodeId {
        self.next_id
    }
}
