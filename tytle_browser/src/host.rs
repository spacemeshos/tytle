use tytle::ast::statement::{Command, Direction};
use tytle::vm::Host;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/tytle_host.js")]
extern "C" {
    type TytleHost;

    #[wasm_bindgen(constructor)]
    fn new() -> TytleHost;

    #[wasm_bindgen(method)]
    fn forward(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn backward(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn left(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn right(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn setx(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn sety(this: &TytleHost, count: i32);

    #[wasm_bindgen(method)]
    fn show_turtle(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn hide_turtle(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn pen_up(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn pen_down(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn pen_erase(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn clean(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn clear_screen(this: &TytleHost);

    #[wasm_bindgen(method)]
    fn print(this: &TytleHost, msg: &str);
}

pub struct BrowserHost {
    browser: TytleHost,
}

impl BrowserHost {
    pub fn new() -> Self {
        Self {
            browser: TytleHost::new(),
        }
    }
}

impl Host for BrowserHost {
    fn exec_direct(&mut self, direct: &Direction, count: isize) {
        let count = count as i32;

        match direct {
            Direction::Forward => self.browser.forward(count),
            Direction::Backward => self.browser.backward(count),
            Direction::Left => self.browser.left(count),
            Direction::Right => self.browser.right(count),
            Direction::SetX => self.browser.setx(count),
            Direction::SetY => self.browser.sety(count),
        }
    }

    fn exec_cmd(&mut self, cmd: &Command) {
        match cmd {
            Command::ShowTurtle => self.browser.show_turtle(),
            Command::HideTurtle => self.browser.hide_turtle(),
            Command::PenUp => self.browser.pen_up(),
            Command::PenDown => self.browser.pen_down(),
            Command::PenErase => self.browser.pen_erase(),
            Command::Clean => self.browser.clean(),
            Command::ClearScreen => self.browser.clear_screen(),
            _ => {
                // TOO
            }
        }
    }

    fn exec_trap(&mut self, node: usize, ip: usize) {
        // TODO
    }

    fn exec_print(&mut self, value: isize) {
        let msg = format!("[PRINT] {}", value);
        self.browser.print(&msg);
    }
}
