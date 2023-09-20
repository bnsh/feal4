// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use std::collections::HashMap;

use gloo_net::http::Request;

use yew::html;
use yew::html::Html;
use yew::functional::{use_state, use_effect_with_deps};
use yew::functional::function_component;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Edge {
    src: i32,
    dst: i32,
    label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    color: String,
    x: f32,
    y: f32,
    size: f32,
    label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graph {
    edges: Vec<Edge>,
    nodes: HashMap<String, Node>,
}

#[function_component]
fn App() -> Html {
    let graph = use_state(|| None::<Graph>);

    {
        let graph = graph.clone();
        use_effect_with_deps(move |_| {
            let graph = graph.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_graph: Graph = Request::get("/graph.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                graph.set(Some(fetched_graph));
            });
            || ()
        }, ());
    }

    if let Some(graph_data) = &*graph {
        html! {
            <div>
                { format!("There are {} edges", graph_data.edges.len()) }
            </div>
        }
    } else {
        html! {
            <div>{ "Loading..." }</div>
        }
    }

}

fn main() {
    yew::Renderer::<App>::new().render();
}
