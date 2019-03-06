use crate::ir::CfgInstruction;
use std::collections::HashMap;

pub type CfgNodeId = usize;

#[derive(Debug, Clone)]
pub struct CfgEdge(CfgNodeId, CfgNodeId);

#[derive(Debug, Clone)]
pub struct CfgNode {
    pub id: CfgNodeId,
    pub insts: Vec<CfgInstruction>,
}

impl CfgNode {
    pub fn new(id: CfgNodeId) -> Self {
        Self {
            id,
            insts: Vec::new(),
        }
    }

    pub fn append_inst(&mut self, inst: CfgInstruction) {
        self.insts.push(inst);
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

    pub fn current_node_mut(&mut self) -> &mut CfgNode {
        let node_id = self.next_id - 1;
        self.nodes.get_mut(&node_id).unwrap()
    }

    pub fn new_node(&mut self) -> &mut CfgNode {
        let node = CfgNode::new(self.next_id);

        self.nodes.insert(node.id, node);

        self.next_id += 1;

        self.current_node_mut()
    }

    pub fn get_next_id(&self) -> CfgNodeId {
        self.next_id
    }
}
