#![feature(test)]
#![feature(box_patterns)]

#[macro_use]
extern crate lazy_static;

extern crate cfg_if;
extern crate wasm_bindgen;

#[macro_use]
pub mod types;
pub mod defaults;
pub mod parser;
pub mod ast;
pub mod runtime;

// test
mod test_parser;
mod test_types;

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
pub fn parse_input(s: &str) -> JsValue {
    let result = parser::parse(s);
    JsValue::from_serde(&result).unwrap()
}