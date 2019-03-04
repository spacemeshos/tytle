use crate::ir::CfgInstruction;
use std::collections::HashMap;

pub type CfgNodeId = usize;

pub struct CfgEdge(CfgNodeId, CfgNodeId);

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

    pub fn append_stmt(&mut self, inst: CfgInstruction) {
        self.insts.push(inst);
    }
}

pub struct CfgGraph {
    pub next_id: CfgNodeId,
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

        let mut nodes = HashMap::new();
        nodes.insert(node.id, node);

        self.current_node_mut()
    }
}
