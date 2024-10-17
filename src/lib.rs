use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fngreet(name:&str) -> String {
    format!("Hello,{}",name)
}
