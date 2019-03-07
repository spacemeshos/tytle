use crate::ir::CfgInstruction;
use std::collections::HashMap;

pub type CfgNodeId = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CfgJumpType {
    WhenTrue,
    Always,
    Fallback,
}

#[derive(Debug, Clone)]
pub enum CfgElement {
    Node(CfgNode),
    Edge(CfgNodeId, CfgNodeId, CfgJumpType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgEdge {
    pub node_id: CfgNodeId,
    pub jmp_type: CfgJumpType,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

    // used for testing when manually building a graph
    pub fn add_element(&mut self, elem: CfgElement) {
        match elem {
            CfgElement::Node(node) => self.add_node(node),
            CfgElement::Edge(src_id, dst_id, jmp_type) => self.add_edge(src_id, dst_id, jmp_type),
        }
    }

    // used for testing when manually building a graph
    pub fn add_node(&mut self, node: CfgNode) {
        let node_id = node.id;

        self.nodes.insert(node_id, node);

        if self.get_current_id() < node_id {
            self.next_id = node_id + 1;
        }
    }

    // used for testing when manually building a graph
    pub fn add_edge(&mut self, src_id: CfgNodeId, dst_id: CfgNodeId, jmp_type: CfgJumpType) {
        let mut src_node = self.get_node_mut(src_id);
        src_node.add_outgoing_edge(dst_id, jmp_type);

        let mut dst_node = self.get_node_mut(dst_id);
        dst_node.add_incoming_edge(src_id, jmp_type);
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
