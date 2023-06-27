use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: usize, y: usize) -> usize {
    x + y
}
