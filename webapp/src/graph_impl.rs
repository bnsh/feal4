// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be an object that can _actually_ do
 * feal4.
 */

use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::wasm_bindgen;
use gloo_console::log;

use yew::{
    html,
    Html
};

use crate::graph::Node;
use crate::computation_graph::{
    ComputationGraph,
    ComputationNode,
    Plaintext,
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8_11, Key12_15,
    Copy16, Copy32, Copy64,
    Left, Right,
    F,
    Xor32, Xor64,
    Swap,
    Ciphertext,
};

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
    compnodes: Vec<Rc<RefCell<dyn ComputationNode>>>,
}

impl GraphImpl {
    pub fn new(graph_data: &Vec<Node>) -> GraphImpl {
        let mut compnodes: Vec<Rc<RefCell<dyn ComputationNode>>> = vec![];
        for node in graph_data {
            let res : Rc<RefCell<dyn ComputationNode>> = match node.compgraph {
                ComputationGraph::Plaintext {} => Rc::new(RefCell::new(Plaintext{node: node.clone()})),
                ComputationGraph::Key0 {} => Rc::new(RefCell::new(Key0{node: node.clone()})),
                ComputationGraph::Key1 {} => Rc::new(RefCell::new(Key1{node: node.clone()})),
                ComputationGraph::Key2 {} => Rc::new(RefCell::new(Key2{node: node.clone()})),
                ComputationGraph::Key3 {} => Rc::new(RefCell::new(Key3{node: node.clone()})),
                ComputationGraph::Key4 {} => Rc::new(RefCell::new(Key4{node: node.clone()})),
                ComputationGraph::Key5 {} => Rc::new(RefCell::new(Key5{node: node.clone()})),
                ComputationGraph::Key6 {} => Rc::new(RefCell::new(Key6{node: node.clone()})),
                ComputationGraph::Key7 {} => Rc::new(RefCell::new(Key7{node: node.clone()})),
                ComputationGraph::Key8_11 {} => Rc::new(RefCell::new(Key8_11{node: node.clone()})),
                ComputationGraph::Key12_15 {} => Rc::new(RefCell::new(Key12_15{node: node.clone()})),
                ComputationGraph::Copy16 {src} => Rc::new(RefCell::new(Copy16{node: node.clone(), src: compnodes[src].clone()})),
                ComputationGraph::Copy32 {src} => Rc::new(RefCell::new(Copy32{node: node.clone(), src: compnodes[src].clone()})),
                ComputationGraph::Copy64 {src} => Rc::new(RefCell::new(Copy64{node: node.clone(), src: compnodes[src].clone()})),
                ComputationGraph::Left {src} => Rc::new(RefCell::new(Left{node: node.clone(), src: compnodes[src].clone()})),
                ComputationGraph::Right {src} => Rc::new(RefCell::new(Right{node: node.clone(), src: compnodes[src].clone()})),
                ComputationGraph::F {subkey, value} => Rc::new(RefCell::new(F{node: node.clone(), subkey: compnodes[subkey].clone(), value: compnodes[value].clone()})),
                ComputationGraph::Xor32 {a, b} => Rc::new(RefCell::new(Xor32{node: node.clone(), a: compnodes[a].clone(), b: compnodes[b].clone()})),
                ComputationGraph::Xor64 {a, b} => Rc::new(RefCell::new(Xor64{node: node.clone(), a: compnodes[a].clone(), b: compnodes[b].clone()})),
                ComputationGraph::Swap {left, right} => Rc::new(RefCell::new(Swap{node: node.clone(), left: compnodes[left].clone(), right: compnodes[right].clone()})),
                ComputationGraph::Ciphertext {src} => Rc::new(RefCell::new(Ciphertext{node: node.clone(), src: compnodes[src].clone()})),
            };
            compnodes.push(res);
        }
        GraphImpl{compnodes: compnodes,}
    }

    pub fn pass_differential(&self, differential: u64) {
        let plaintext1 = random_u64();
        let plaintext2 = plaintext1 ^ differential;

        log!(format!("plaintext1 is {}", plaintext1));
        log!(format!("plaintext2 is {}", plaintext2));
    }

    pub fn compute_size(&self) -> (f32, f32, f32, f32) {
        let min_x = self.compnodes.iter().map(|node| node.borrow().node().x).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or_default();
        let max_x = self.compnodes.iter().map(|node| node.borrow().node().x).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or_default();
        let min_y = self.compnodes.iter().map(|node| node.borrow().node().y).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or_default();
        let max_y = self.compnodes.iter().map(|node| node.borrow().node().y).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or_default();

        (min_x, max_x, min_y, max_y)
    }

    pub fn render(&self) -> Html {
        let (minx, maxx, miny, maxy) = self.compute_size();
        let scale = 1.0;
        let margin = 100.0;
        let width = maxx - minx; let width_s = format!("{}", ((width + 2.0 * margin) as i32));
        let height = maxy - miny; let height_s = format!("{}", ((height + 100.0 + 2.0 * margin) as i32));
        let transform_s = format!("translate({}, {}) scale({}, {})", -minx + margin, -miny + margin, scale, scale);

        let node_htmls: Vec<Html> = self.compnodes.iter().map(|node| node.borrow().render_node()).collect();
        let edge_htmls: Vec<Html> = self.compnodes.iter().map(|node| node.borrow().render_edges()).collect();

        html! {
                <svg width={width_s} height={height_s} style="margin-right: 10px;">
                    <defs>
                        <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="0" refY="3.5" orient="auto">
                            <polygon points="0 0, 10 3.5, 0 7" />
                        </marker>
                    </defs>
                    <g transform={transform_s}>
                        { for node_htmls.into_iter() }
                        { for edge_htmls.into_iter() }
                    </g>
                </svg>
        }
    }
}
