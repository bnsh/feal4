// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use std::collections::HashMap;

use yew::html;
use yew::html::Html;
use yew::functional::function_component;

use reqwest;
use serde::{Deserialize, Serialize};

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

impl Component for GraphComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            graph: None,
            error: None,
            task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveGraph(Ok(graph)) => {
                self.graph = Some(graph);
                self.task = None;  // Drop the completed fetch task.
            }
            Msg::ReceiveGraph(Err(error)) => {
                self.error = Some(error);
                self.task = None;  // Drop the completed fetch task.
            }
        }
        true
    }

    fn view(&self) -> Html {
        if let Some(error) = &self.error {
            return html! { <div>{error}</div> };
        }
        if let Some(graph) = &self.graph {
            // Render your graph data.
            return html! { <div>{"Graph data received!"}</div> };  // Simplified for brevity.
        }
        html! { <div>{"Loading..."}</div> }
    }

}

#[function_component]
fn App() -> Html {
    html! {
        <div><p>{"Hello!"}</p></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
