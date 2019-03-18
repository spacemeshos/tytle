use crate::ast::semantic::{Environment, SymbolId};
use crate::ast::statement::{Command, Direction};
use crate::ir::{CfgGraph, CfgInstruction, CfgNodeId};
use crate::vm::*;

pub struct Interpreter<'env, 'cfg> {
    memory: Memory,
    call_stack: CallStack,
    env: &'env Environment,
    cfg_graph: &'cfg CfgGraph,
    node_id: CfgNodeId,
    ip: usize,
}

impl<'env, 'cfg> Interpreter<'env, 'cfg> {
    pub fn new(cfg_graph: &'cfg CfgGraph, env: &'env Environment) -> Self {
        let start_id = cfg_graph.get_entry_node_id();

        let mut interpreter = Self {
            ip: 0,
            env,
            cfg_graph,
            memory: Memory::new(),
            call_stack: CallStack::new(),
            node_id: start_id,
        };

        // TODO: setup a wrapper to main procedure

        interpreter
    }

    pub fn exec_next(&mut self) -> bool {
        let node = self.cfg_graph.get_node(self.node_id);

        let inst = node.insts.get(self.ip);

        if inst.is_none() {
            return false;
        }

        let inst = inst.unwrap();

        match inst {
            CfgInstruction::Command(ref cmd) => self.exec_cmd(cmd),
            // CfgInstruction::Direction(ref direct) => self.exec_direct(direct),
            // CfgInstruction::Load(ref symbol_id) => self.exec_load(symbol_id),
            // CfgInstruction::Store(ref symbol_id) => self.exec_store(symbol_id),
            // CfgInstruction::Call(ref node_id) => self.exec_call(node_id),
            // CfgInstruction::Bool(v) => self.exec_bool(*v),
            // CfgInstruction::Int(v) => self.exec_int(*v),
            // CfgInstruction::Str(v) => unimplemented!(),
            // CfgInstruction::Return => self.exec_ret(),
            // CfgInstruction::Add => self.exec_add(),
            // CfgInstruction::Mul => self.exec_mul(),
            // CfgInstruction::Not => self.exec_not(),
            // CfgInstruction::And => self.exec_and(),
            // CfgInstruction::Or => self.exec_or(),
            // CfgInstruction::GT => self.exec_gt(),
            // CfgInstruction::LT => self.exec_lt(),
            _ => unimplemented!(),
        };

        true
    }

    fn exec_load(&mut self, symbol_id: &SymbolId) {}

    fn exec_store(&mut self, symbol_id: &SymbolId) {}

    fn exec_call(&mut self, node_id: &CfgNodeId) {
        //
    }

    fn exec_ret(&mut self) {
        //
    }

    fn exec_cmd(&mut self, cmd: &Command) {}

    fn exec_direct(&mut self, direct: &Direction) {
        //
    }

    fn exec_add(&mut self) {}

    fn exec_mul(&mut self) {}

    fn exec_not(&mut self) {}

    fn exec_and(&mut self) {}

    fn exec_or(&mut self) {}

    fn exec_gt(&mut self) {}

    fn exec_lt(&mut self) {}

    fn exec_bool(&mut self, value: bool) {}

    fn exec_int(&mut self, value: usize) {}
}
