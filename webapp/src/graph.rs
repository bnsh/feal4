// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use yew::{
    function_component,
    html,
    Html,
    Properties,
    use_effect_with_deps,
    use_state
};

use gloo_console::log;
use gloo_net::http::Request;

use serde::{Deserialize, Serialize};

use crate::computation_graph;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Node {
    id: i32,
    color: String,
    x: f32,
    y: f32,
    radius: f32,
    size: f32,
    bitsize: i32,
    #[serde(flatten)]
    compgraph: computation_graph::ComputationGraph,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub differential: i64
}

#[function_component(Graph)]
pub fn app(props: &Props) -> Html {
    let graph = use_state(|| None::<Vec<Node>>);

    {
        let graph = graph.clone();
        use_effect_with_deps(move |_| {
            let graph = graph.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_graph: Option<Vec<Node>> = Request::get("/graph.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                graph.set(fetched_graph.clone());
                log!(format!("Loaded {} nodes from graph.json.", fetched_graph.unwrap().len()));
            });
            || ()
        }, ());
    }

    if let Some(graph_data) = &*graph {
        html! {
            <div>{props.differential} {graph_data.len()}</div>
        }
    }
    else {
        html! {
            <div>{"Loading..."}</div>
        }
    }
}
