use crate::ast::semantic::{Environment, SymbolId, SymbolKind};
use crate::ast::statement::{Command, Direction};
use crate::ir::{CfgGraph, CfgInstruction, CfgNodeId, CfgObject};
use crate::vm::*;

pub struct Interpreter<'env, 'cfg, 'host> {
    pub ip: usize,
    pub node_id: CfgNodeId,
    pub memory: Memory,
    pub call_stack: CallStack,
    env: &'env Environment,
    cfg: &'cfg CfgObject,
    host: &'host mut Host,
}

impl<'env, 'cfg, 'host> Interpreter<'env, 'cfg, 'host> {
    pub fn new(cfg: &'cfg CfgObject, env: &'env Environment, host: &'host mut Host) -> Self {
        // node with `id = 0` is reserved for the `main wrapper`
        // while node having `id = 1` is reserved for the `main`
        let main_node_id = cfg.graph.get_entry_node_id();

        let mut intr = Self {
            ip: 0,
            env,
            host,
            cfg,
            memory: Memory::new(),
            call_stack: CallStack::new(),
            node_id: main_node_id,
        };

        intr.init_memory();
        intr.init_callstack();

        intr
    }

    pub fn exec_code(&mut self) {
        loop {
            let completed = self.exec_next();

            if completed {
                return;
            }
        }
    }

    pub fn exec_next(&mut self) -> bool {
        let node = self.cfg.graph.get_node(self.node_id);

        let inst = node.insts.get(self.ip);

        if inst.is_none() {
            return true;
        }

        let inst = inst.unwrap();
        let mut is_jmp = false;

        match inst {
            CfgInstruction::Call(ref node_id) => {
                is_jmp = true;
                self.exec_call(*node_id);
            }
            CfgInstruction::Command(ref cmd) => self.exec_cmd(cmd),
            CfgInstruction::Direction(ref direct) => self.exec_direct(direct),
            CfgInstruction::Bool(v) => self.exec_bool(*v),
            CfgInstruction::Int(v) => self.exec_int(*v),
            CfgInstruction::Return => self.exec_ret(),
            CfgInstruction::Not => self.exec_not(),
            CfgInstruction::Add | CfgInstruction::Mul => self.exec_int_binary(inst.clone()),
            CfgInstruction::Or | CfgInstruction::And | CfgInstruction::GT | CfgInstruction::LT => {
                self.exec_bool_binary(inst.clone())
            }
            CfgInstruction::Load(var_id) => self.exec_load(*var_id),
            CfgInstruction::Store(var_id) => self.exec_store(*var_id),
            CfgInstruction::Str(v) => unimplemented!(),
        };

        if is_jmp == false {
            self.ip += 1;
        }

        false
    }

    fn exec_load(&mut self, var_id: SymbolId) {
        let var = self.env.symbol_table.get_var_by_id(var_id).unwrap();

        if var.global {
            // for global variables the rule is: `addr` <=> `global index`
            let addr = Address(var.index.unwrap());

            let value = self.memory.get_global(addr).unwrap();

            match value {
                MemoryValue::Int(v) => self.exec_int(*v),
                MemoryValue::Bool(v) => self.exec_bool(*v),
                MemoryValue::Str(v) => unimplemented!(),
            };
        } else {
            unimplemented!()
        }
    }

    fn exec_store(&mut self, var_id: SymbolId) {
        let var = self.env.symbol_table.get_var_by_id(var_id).unwrap();
        let index = var.index.unwrap();

        if var.global {
            let stack_value = self.call_stack.pop_item();

            let value = match stack_value {
                CallStackItem::Int(v) => MemoryValue::Int(v),
                CallStackItem::Bool(v) => MemoryValue::Bool(v),
                _ => unimplemented!(),
            };

            let addr = Address(index);
            self.memory.set_global(addr, value);
        } else {
            unimplemented!()
        }
    }

    fn exec_call(&mut self, callee_id: CfgNodeId) {
        let old_frame = self.call_stack.current_frame_mut();

        let proc_id = self.cfg.jmp_table[&callee_id];
        let proc = self.env.symbol_table.get_proc_by_id(proc_id).unwrap();

        let mut params = Vec::new();
        let nparams = proc.params_types.len();

        // allocate procedure params by copying the call args from the current (old) stack-frame
        (1..=nparams).into_iter().for_each(|_| {
            let param = old_frame.pop();

            params.push(param);
        });

        // pushing the return address to the top of the old stack-frame
        let ret_addr = CallStackItem::Addr(self.node_id, self.ip);
        old_frame.push(ret_addr);

        // callee allocates a new callstack frame
        let new_frame = self.call_stack.open_stackframe();
        for param in params {
            new_frame.push(param);
        }

        // pointing the next instruction, to the first instruction of the destination CFG node
        self.node_id = callee_id;
        self.ip = 0;
    }

    fn exec_ret(&mut self) {
        // TODO: handle procs returning `Unit`

        let ret_item = self.call_stack.pop_item();

        // unwinding the procedure callstack frame
        self.call_stack.close_stackframe();

        let ret_addr = self.call_stack.pop_item();
        let (ret_node_id, ret_ip) = ret_addr.to_addr();

        // pointing one instruction after the `call`
        self.node_id = ret_node_id;
        self.ip = ret_ip;
    }

    fn exec_cmd(&mut self, cmd: &Command) {
        self.host.exec_cmd(cmd);
    }

    fn exec_direct(&mut self, direct: &Direction) {
        let count = self.call_stack.pop_item().to_int();

        self.host.exec_direct(direct, count)
    }

    fn exec_int_binary(&mut self, op: CfgInstruction) {
        let a = self.call_stack.pop_item();
        let b = self.call_stack.pop_item();

        assert!(a.is_int() && b.is_int());

        let a = a.to_int();
        let b = b.to_int();

        match op {
            CfgInstruction::Add => self.exec_int(a + b),
            CfgInstruction::Mul => self.exec_int(a * b),
            _ => panic!("invalid binary-op: `{:?}`", op),
        }
    }

    fn exec_not(&mut self) {
        let a = self.call_stack.pop_item();

        assert!(a.is_bool());

        let b = !a.to_bool();

        self.exec_bool(b);
    }

    fn exec_bool_binary(&mut self, op: CfgInstruction) {
        let a = self.call_stack.pop_item();
        let b = self.call_stack.pop_item();

        assert!(a.is_bool() && b.is_bool());

        let a = a.to_bool();
        let b = b.to_bool();

        match op {
            CfgInstruction::And => self.exec_bool(a && b),
            CfgInstruction::Or => self.exec_bool(a || b),
            CfgInstruction::GT => self.exec_bool(a > b),
            CfgInstruction::LT => self.exec_bool(a < b),
            _ => panic!("invalid binary-op: `{:?}`", op),
        }
    }

    fn exec_bool(&mut self, v: bool) {
        self.call_stack.push_item(CallStackItem::Bool(v));
    }

    fn exec_int(&mut self, v: isize) {
        self.call_stack.push_item(CallStackItem::Int(v));
    }

    fn init_memory(&mut self) {
        self.memory.init_globals(self.env);
    }

    fn init_callstack(&mut self) {
        // we create a `__main__` wrapper stack-frame a.k.a `__main_wrapper__`
        // and we call `__main__` within this stack-frame context
        //
        //
        //     Call-Stack
        //     ===========
        //
        // |                  |
        // |      ....        |
        // |      ....        |
        // |      ....        |
        // |                  |
        // |    __main__      |
        // |  (node_id = 1)   |
        // |------------------|
        // |-------------------
        // |                  |
        // | __main_wrapper__ |
        // |  (node_id = 0)   |
        // |------------------|

        assert!(self.call_stack.is_empty());
        self.call_stack.open_stackframe();
    }
}
