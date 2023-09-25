// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use std::collections::HashMap;

use gloo_console::log;
use gloo_net::http::Request;

use yew::html;
use yew::html::Html;
use yew::functional::{use_state, use_effect_with_deps};
use yew::functional::function_component;

use serde::{Deserialize, Serialize};

use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
struct Edge {
    src: i32,
    dst: i32,
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    color: String,
    x: f32,
    y: f32,
    radius: f32,
    size: f32,
    #[serde(default)]
    value: i64,
    bitsize: i32,
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graph {
    edges: Vec<Edge>,
    nodes: HashMap<i32, Node>,
}

fn compute_size(graph: &Graph) -> (f32, f32, f32, f32) {
    let min_x = graph.nodes.values()
        .map(|node| node.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_default();
    let max_x = graph.nodes.values()
        .map(|node| node.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_default();
    let min_y = graph.nodes.values()
        .map(|node| node.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_default();
    let max_y = graph.nodes.values()
        .map(|node| node.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_default();

    (min_x, max_x, min_y, max_y)
}

fn hexstr(value: i64, bitsize: i32) -> String {
    let hex_str = format!("{:x}", value);
    let required_chars = bitsize / 4;
    let padded_str = format!("{:0>width$}", hex_str, width = required_chars as usize);
    format!("0x{}", padded_str)
}

fn grab_svg_nodes(nodes: &HashMap<i32, Node>) -> Vec<Html> {
    let svg_nodes : Vec<Html> = nodes.iter().map(|(_nodeid, node)| {
        let (realx, realy) = (node.x, node.y);
        let cx_str = format!("{}", (realx as i32));
        let cy_str = format!("{}", (realy as i32));
        let r_str = format!("{}", node.radius);
        let fill_str = format!("{}", node.color);
        let label_str = format!("{}", node.label);
        let value = hexstr(node.value, node.bitsize);

        html! {
            <>
                <circle cx={cx_str.clone()} cy={cy_str.clone()} r={r_str} fill={fill_str} />
                <text x={cx_str.clone()} y={cy_str.clone()} font-family="Arial" font-size="10" fill="black" text-anchor="middle" dy=".3em">{label_str}</text>
                <rect x={format!("{}", realx-56.0)} y={format!("{}", realy+21.)} width="112" height="50" rx="10" ry="10" fill="white" stroke="black"/>
                <text x={format!("{}", realx)} y={format!("{}", realy+46.)} font-family="Arial" font-size="10" fill="black" text-anchor="middle" dy=".3em">{value}</text>
            </>
        }
    }).collect();

    svg_nodes
}

fn grab_svg_edges(graph: &Graph) -> Vec<Html> {
    let svg_edges : Vec<Html> = graph.edges.iter().map(|edge| {
        let src = graph.nodes.get(&edge.src).unwrap();
        let dst = graph.nodes.get(&edge.dst).unwrap();

        let markersz = 20.0;

        let theta = f32::atan2(dst.y - src.y, dst.x - src.x);
        let (srcx, srcy) = (src.x + 1.0 * src.radius * theta.cos(), src.y + 1.0 * src.radius * theta.sin());
        let (dstx, dsty) = (dst.x - 1.0 * (dst.radius+markersz) * theta.cos(), dst.y - 1.0 * (dst.radius+markersz) * theta.sin());

        let d = format!("M {},{} L {},{}", srcx, srcy, dstx, dsty);

        html! {
            <path d={d} fill="none" stroke="black" stroke-width="2" marker-end="url(#arrowhead)" />
        }
    }).collect();

    svg_edges
}

fn handle(graph: &Graph) -> Html {
    let (minx, maxx, miny, maxy) = compute_size(graph);
    log!(JsValue::from_str(&serde_json::to_string(&vec![minx, maxx, miny, maxy]).unwrap()));
    let scale = 1.0;
    let margin = 100.0;
    let width = maxx - minx; let width_s = format!("{}", ((width + 2.0 * margin) as i32));
    let height = maxy - miny; let height_s = format!("{}", ((height + 100.0 + 2.0 * margin) as i32));
    /*
So, what do we want to do?
First. translate minx, miny -> margin * width, margin * height: minx - minx + margin * width, miny - miny + margin * height
Second, scale by scale, scale
     */
    let transform_s = format!("translate({}, {}) scale({}, {})", -minx + margin, -miny + margin, scale, scale);
    let svg_nodes = grab_svg_nodes(&graph.nodes);
    let svg_edges = grab_svg_edges(&graph);

    html! {
        <div style="display: flex; align-items: flex-start; position: relative">

            <svg width={width_s} height={height_s} style="margin-right: 10px;">
                <defs>
                    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="0" refY="3.5" orient="auto">
                        <polygon points="0 0, 10 3.5, 0 7" />
                    </marker>
                </defs>
                <g transform={transform_s}>
                    { for svg_edges.into_iter() } // First draw all the edges.
                    { for svg_nodes.into_iter() }
                </g>
            </svg>
            <div style="position: sticky; top: 0;">
                <input type="text" placeholder="Enter text" />
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let graph = use_state(|| None::<Graph>);

    {
        let graph = graph.clone();
        use_effect_with_deps(move |_| {
            let graph = graph.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_graph: Option<Graph> = Request::get("/graph.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                graph.set(fetched_graph);
            });
            || ()
        }, ());
    }

    if let Some(graph_data) = &*graph {
        handle(graph_data)
    } else {
        html! {
            <div>{ "Loading..." }</div>
        }
    }

}

fn main() {
    yew::Renderer::<App>::new().render();
}
