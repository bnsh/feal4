// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use std::collections::HashMap;

use gloo_net::http::Request;

use yew::html;
use yew::html::Html;
use yew::functional::{use_state, UseStateHandle, use_effect_with_deps};
use yew::functional::function_component;
use yew::Callback;

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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
    size: f32,
    label: String,
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
            let graph_clone = graph.clone();
            let task: FetchTask = FetchService::fetch(
                Request::get("/graph.json").body(yew::format::Nothing).unwrap(),
                Callback::from(move |response: yew::services::fetch::Response<Json<Result<Graph, anyhow::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            graph_clone.set(Some(body));
                        }
                    }
                }),
            ).expect("Failed to start request");
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
