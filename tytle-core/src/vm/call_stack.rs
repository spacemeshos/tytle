use crate::ir::CfgNodeId;

#[derive(Debug, Clone)]
pub enum CallStackItem {
    Int(isize),
    Bool(bool),
    Addr(CfgNodeId, usize),
    // StrRef
}

impl CallStackItem {
    pub fn is_int(&self) -> bool {
        match self {
            CallStackItem::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            CallStackItem::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_addr(&self) -> bool {
        match self {
            CallStackItem::Addr(..) => true,
            _ => false,
        }
    }

    pub fn to_int(&self) -> isize {
        match self {
            CallStackItem::Int(v) => *v,
            _ => panic!("expected an integer"),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            CallStackItem::Bool(v) => *v,
            _ => panic!("expected a bool"),
        }
    }

    pub fn to_addr(&self) -> (CfgNodeId, usize) {
        match self {
            CallStackItem::Addr(node_id, ip) => (*node_id, *ip),
            _ => panic!("expected an address"),
        }
    }
}

#[derive(Debug)]
pub struct CallStackFrame {
    items: Vec<CallStackItem>,
}

impl CallStackFrame {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, item: CallStackItem) {
        self.items.push(item);
    }

    pub fn load(&mut self, index: usize) -> &CallStackItem {
        self.items.get(index).unwrap()
    }

    pub fn store(&mut self, index: usize, item: CallStackItem) {
        std::mem::replace(&mut self.items[index], item);
    }

    pub fn peek(&self) -> &CallStackItem {
        self.items.last().unwrap()
    }

    pub fn pop(&mut self) -> CallStackItem {
        self.items.pop().unwrap()
    }
}

#[derive(Debug)]
pub struct CallStack {
    pub frames: Vec<CallStackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn load_item(&mut self, index: usize) -> &CallStackItem {
        let frame = self.current_frame_mut();
        frame.load(index)
    }

    pub fn store_item(&mut self, index: usize, item: CallStackItem) {
        let frame = self.current_frame_mut();
        frame.store(index, item);
    }

    pub fn push_item(&mut self, item: CallStackItem) {
        let frame = self.current_frame_mut();
        frame.push(item);
    }

    pub fn pop_item(&mut self) -> CallStackItem {
        let frame = self.current_frame_mut();
        frame.pop()
    }

    pub fn peek_item(&self) -> &CallStackItem {
        let frame = self.current_frame();
        frame.peek()
    }

    pub fn open_stackframe(&mut self) -> &mut CallStackFrame {
        let mut frame = CallStackFrame::new();
        self.frames.push(frame);

        self.current_frame_mut()
    }

    pub fn close_stackframe(&mut self) {
        self.frames.pop();
    }

    pub fn current_frame(&self) -> &CallStackFrame {
        self.frames.last().unwrap()
    }

    pub fn current_frame_mut(&mut self) -> &mut CallStackFrame {
        self.frames.last_mut().unwrap()
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn vm_callstack_sanity() {
        panic!()
    }
}
