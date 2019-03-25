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
        // TODO
    }

    fn exec_trap(&mut self, node: usize, ip: usize) {
        // TODO
    }

    fn exec_print(&mut self, value: isize) {
        // TODO
    }
}
