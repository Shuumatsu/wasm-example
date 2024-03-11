mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/workers.js")]
extern "C" {
    fn simple_example() -> String;
}

#[wasm_bindgen]
pub fn greet() {
    simple_example();
    alert("Hello, myproject!");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test() {
        simple_example();
    }
}