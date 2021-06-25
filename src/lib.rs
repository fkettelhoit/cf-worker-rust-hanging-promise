extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn handle(obj: ObjectWithAsyncMethod) -> Result<String, JsValue> {
    obj.run().await?;
    Ok("Hello, Async!".to_string())
}

#[wasm_bindgen]
extern "C" {
    pub type ObjectWithAsyncMethod;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn run(this: &ObjectWithAsyncMethod) -> Result<JsValue, JsValue>;
}
