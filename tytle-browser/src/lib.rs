use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn forward(count: i32);
// fn backward(count: i32);
// fn left(count: i32);
// fn right(count: i32);
}

#[wasm_bindgen]
pub fn execute(code: &str) {
    let formatted_code = format!("Code: {}", code);

    println!("{}", formatted_code);
}
