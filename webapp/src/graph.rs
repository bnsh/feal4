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
use crate::graph_impl::GraphImpl;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    id: u32,
    color: String,
    x: f32,
    y: f32,
    radius: f32,
    size: f32,
    bitsize: u32,
    #[serde(flatten)]
    compgraph: computation_graph::ComputationGraph,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub differential: u64
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
        let graph_impl = GraphImpl::new(graph_data);
        graph_impl.pass_differential(props.differential);
        html! {
            <div>{"differential="}{props.differential}<br />{"node_count="}{graph_data.len()}<br /></div>
        }
    }
    else {
        html! {
            <div>{"Loading..."}</div>
        }
    }
}
