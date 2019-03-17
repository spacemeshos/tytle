use crate::vm::position::Position;

pub trait Screen {
    fn draw_turtle(&mut self, position: Position);
}

struct DebugScreen;

impl Screen for DebugScreen {
    fn draw_turtle(&mut self, position: Position) {
        println!("Moved turtle to {:?}", position);
    }
}
