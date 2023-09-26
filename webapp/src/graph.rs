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
    compgraph: ComputationGraph,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "label")]
enum ComputationGraph {
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

    #[serde(rename = "left")]
    Left {src: i32},

    #[serde(rename = "right")]
    Right {src: i32},

    #[serde(rename = "F")]
    F {subkey: i32, value: i32},

    #[serde(rename = "copy")]
    Copy {src: i32},

    #[serde(rename = "swap")]
    Swap {left: i32, right: i32},

    #[serde(rename = "xor")]
    Xor {a: i32, b: i32},

    #[serde(rename = "ciphertext")]
    Ciphertext {src: i32}
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
