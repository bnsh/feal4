// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 * Remember all the values (src, subkey, value, etc. are node _indices_ not actual _values_!
 */

use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use yew::{
    Html,
    html
};

use crate::graph::Node;
use crate::feal::f;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "label")]
pub enum ComputationGraph {
    #[serde(rename = "plaintext")]
    Plaintext {},

    #[serde(rename = "key0")]
    Key0 {},

    #[serde(rename = "key1")]
    Key1 {},

    #[serde(rename = "key2")]
    Key2 {},

    #[serde(rename = "key3")]
    Key3 {},

    #[serde(rename = "key4")]
    Key4 {},

    #[serde(rename = "key5")]
    Key5 {},

    #[serde(rename = "key6")]
    Key6 {},

    #[serde(rename = "key7")]
    Key7 {},

    #[serde(rename = "key8_11")]
    Key8_11 {},

    #[serde(rename = "key12_15")]
    Key12_15 {},

    #[serde(rename = "copy16")]
    Copy16 {src: usize},

    #[serde(rename = "copy32")]
    Copy32 {src: usize},

    #[serde(rename = "copy64")]
    Copy64 {src: usize},

    #[serde(rename = "left")]
    Left {src: usize},

    #[serde(rename = "right")]
    Right {src: usize},

    #[serde(rename = "F")]
    F {subkey: usize, value: usize},

    #[serde(rename = "xor32")]
    Xor32 {a: usize, b: usize},

    #[serde(rename = "xor64")]
    Xor64 {a: usize, b: usize},

    #[serde(rename = "swap")]
    Swap {left: usize, right: usize},

    #[serde(rename = "ciphertext")]
    Ciphertext {src: usize}
}

pub trait ComputationNode {
    fn label(&self) -> &str;
    fn node(&self) -> &Node;
    fn eval(&self) -> (u64, u64);
    fn render_node(&self) -> Html;
    fn render_edges(&self) -> Html;
}

fn hexstr(value: u64, bitsize: u32) -> String {
    let hex_str = format!("{:x}", value);
    let required_chars = bitsize / 4;
    let padded_str = format!("{:0>width$}", hex_str, width = required_chars as usize);
    format!("0x{}", padded_str)
}

fn generic_render_node(compnode: &dyn ComputationNode) -> Html {
    let (realx, realy) = (compnode.node().x, compnode.node().y);
    let cx_str = format!("{}", (realx as i32));
    let cy_str = format!("{}", (realy as i32));
    let r_str = format!("{}", compnode.node().radius);
    let fill_str = format!("{}", compnode.node().color);
    let label_str = format!("{}", compnode.label());
    let (path1, path2) = compnode.eval();
    let y1 = realy + 36.0; // or some adjusted value for the first line
    let y2 = realy + 46.0; // for the second line
    let y3 = realy + 56.0; // for the third line
    let differential = path1 ^ path2;
    html!(
        <>
            <circle cx={cx_str.clone()} cy={cy_str.clone()} r={r_str} fill={fill_str} />
            <text x={cx_str.clone()} y={cy_str.clone()} font-family="Arial" font-size="10" fill="black" text-anchor="middle" dy=".3em">{label_str}</text>
            <rect x={format!("{}", realx-56.0)} y={format!("{}", realy+21.)} width="112" height="50" rx="10" ry="10" fill="white" stroke="black"/>
            <text x={format!("{}", realx)} y={format!("{}", y1)} font-family="Arial" font-size="10" fill="black" text-anchor="middle" dy=".3em">{hexstr(path1, compnode.node().bitsize)}</text>
            <text x={format!("{}", realx)} y={format!("{}", y2)} font-family="Arial" font-size="10" fill="black" text-anchor="middle" dy=".3em">{hexstr(path2, compnode.node().bitsize)}</text>
            <text x={format!("{}", realx)} y={format!("{}", y3)} font-family="Arial" font-size="10" fill="red" text-anchor="middle" dy=".3em">{hexstr(differential, compnode.node().bitsize)}</text>
        </>
    )
}

fn generic_render_edge(dst: &dyn ComputationNode, src: &dyn ComputationNode) -> Html {
    let markersz = 20.0;

    let theta = f32::atan2(dst.node().y - src.node().y, dst.node().x - src.node().x);
    let (srcx, srcy) = (src.node().x + 1.0 * src.node().radius * theta.cos(), src.node().y + 1.0 * src.node().radius * theta.sin());
    let (dstx, dsty) = (dst.node().x - 1.0 * (dst.node().radius+markersz) * theta.cos(), dst.node().y - 1.0 * (dst.node().radius+markersz) * theta.sin());

    let d = format!("M {},{} L {},{}", srcx, srcy, dstx, dsty);

    html! {
        <path d={d} fill="none" stroke="black" stroke-width="2" marker-end="url(#arrowhead)" />
    }
}

fn generic_render_edges(dst: &dyn ComputationNode, srcs: Vec<Rc<RefCell<dyn ComputationNode>>>) -> Html {
    let edges: Vec<Html> = srcs.iter().map(|src| generic_render_edge(dst, &*src.borrow())).collect();
    html! {
        <>
            { for edges.into_iter() }
        </>
    }
}

pub struct Plaintext {
    pub node: Node,
    pub differential: u64,
    pub value: u64,
}

impl ComputationNode for Plaintext {
    fn label(&self) -> &str {"plaintext"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.value, self.value ^ self.differential)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key0 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key0 {
    fn label(&self) -> &str {"key0"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key1 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key1 {
    fn label(&self) -> &str {"key1"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key2 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key2 {
    fn label(&self) -> &str {"key2"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key3 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key3 {
    fn label(&self) -> &str {"key3"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key4 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key4 {
    fn label(&self) -> &str {"key4"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key5 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key5 {
    fn label(&self) -> &str {"key5"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key6 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key6 {
    fn label(&self) -> &str {"key6"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key7 {
    pub node: Node,
    pub key: u16,
}

impl ComputationNode for Key7 {
    fn label(&self) -> &str {"key7"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key as u64, self.key as u64)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key8_11 {
    pub node: Node,
    pub key: u64,
}

impl ComputationNode for Key8_11 {
    fn label(&self) -> &str {"key8-11"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key, self.key)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Key12_15 {
    pub node: Node,
    pub key: u64,
}

impl ComputationNode for Key12_15 {
    fn label(&self) -> &str {"key12-15"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        (self.key, self.key)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![])}
}

pub struct Copy16 {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Copy16 {
    fn label(&self) -> &str {"copy16"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (src1, src2) = self.src.borrow().eval();
        (src1 & 0x00ffff, src2 & 0x00ffff)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}

pub struct Copy32 {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Copy32 {
    fn label(&self) -> &str {"copy32"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (src1, src2) = self.src.borrow().eval();
        (src1 & 0x00ffffffff, src2 & 0x00ffffffff)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}

pub struct Copy64 {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Copy64 {
    fn label(&self) -> &str {"copy64"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        self.src.borrow().eval()
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}

pub struct Left {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Left {
    fn label(&self) -> &str {"left"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (src1, src2) = self.src.borrow().eval();
        ((src1 >> 32) & 0x00ffffffff, (src2 >> 32) & 0x00ffffffff)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}

pub struct Right {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Right {
    fn label(&self) -> &str {"right"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (src1, src2) = self.src.borrow().eval();
        (src1 & 0x00ffffffff, src2 & 0x00ffffffff)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}

pub struct F{
    pub node: Node,
    pub subkey: Rc<RefCell<dyn ComputationNode>>,
    pub value: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for F {
    fn label(&self) -> &str {"F"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (subkey1, subkey2) = self.subkey.borrow().eval();
        let (value1, value2) = self.value.borrow().eval();
        let subkey1 = subkey1 as u16;
        let subkey2 = subkey2 as u16;
        let value1 = value1 as u32;
        let value2 = value2 as u32;

        let output1 = f(subkey1, value1);
        let output2 = f(subkey2, value2);

        (output1.into(), output2.into())
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.subkey.clone(), self.value.clone()])}
}

pub struct Xor32 {
    pub node: Node,
    pub a: Rc<RefCell<dyn ComputationNode>>,
    pub b: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Xor32 {
    fn label(&self) -> &str {"xor32"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (a1, a2) = self.a.borrow().eval();
        let (b1, b2) = self.b.borrow().eval();
        ((a1 ^ b1) & 0x00ffffffff, (a2 ^ b2) & 0x00ffffffff)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.a.clone(), self.b.clone()])}
}

pub struct Xor64 {
    pub node: Node,
    pub a: Rc<RefCell<dyn ComputationNode>>,
    pub b: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Xor64 {
    fn label(&self) -> &str {"xor64"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (a1, a2) = self.a.borrow().eval();
        let (b1, b2) = self.b.borrow().eval();
        (a1 ^ b1, a2 ^ b2)
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.a.clone(), self.b.clone()])}
}

pub struct Swap {
    pub node: Node,
    pub left: Rc<RefCell<dyn ComputationNode>>,
    pub right: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Swap {
    fn label(&self) -> &str {"swap"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        let (left1, left2) = self.left.borrow().eval();
        let (right1, right2) = self.right.borrow().eval();

        let left1 = left1 & 0x00ffffffff;
        let left2 = left2 & 0x00ffffffff;
        let right1 = right1 & 0x00ffffffff;
        let right2 = right2 & 0x00ffffffff;

        (((right1 << 32) | left1), ((right2 << 32) | left2))
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.left.clone(), self.right.clone()])}
}

pub struct Ciphertext {
    pub node: Node,
    pub src: Rc<RefCell<dyn ComputationNode>>,
}

impl ComputationNode for Ciphertext {
    fn label(&self) -> &str {"ciphertext"}
    fn node(&self) -> &Node {&self.node}
    fn eval(&self) -> (u64, u64) {
        self.src.borrow().eval()
    }
    fn render_node(&self) -> Html {generic_render_node(self)}
    fn render_edges(&self) -> Html {generic_render_edges(self, vec![self.src.clone()])}
}
