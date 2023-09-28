// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be an object that can _actually_ do
 * feal4.
 */

use wasm_bindgen::prelude::*;
use gloo_console::log;

use crate::graph::Node;

#[wasm_bindgen]
extern "C" {
    pub type Math;

    #[wasm_bindgen(static_method_of = Math, js_name = random)]
    pub fn randfloat() -> f32;
}

pub fn random_u16() -> u16 {
    (Math::randfloat() * (u16::MAX as f32)) as u16
}

pub fn random_u64() -> u64 {
    let mut result: u64 = 0;
    for _ in 0..8 {
        let byte: u8 = (Math::randfloat() * (u8::MAX as f32)) as u8;
        result = (result << 8) | (byte as u64);
    }
    result
}

pub struct GraphImpl {
}

impl GraphImpl {
    pub fn new(graph_data: &Vec<Node>) -> GraphImpl {
        for _node in graph_data {
        }
        GraphImpl{
        }
    }

    pub fn pass_differential(&self, differential: u64) {
        let plaintext1 = random_u64();
        let plaintext2 = plaintext1 ^ differential;

        log!(format!("plaintext1 is {}", plaintext1));
        log!(format!("plaintext2 is {}", plaintext2));
    }
}
