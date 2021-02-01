use wasm_bindgen::prelude::*;
mod alignment;
use alignment::Alignment;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn culc_star(js_objects: &JsValue) -> HashMap<String,Vec<i128>> {
    console_error_panic_hook::set_once();
    let align: Alignment = js_objects.into_serde().unwrap();
    let val: HashMap<String,Vec<i128>> = align.culc_star().collect();
    val
}
