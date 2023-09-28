// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use yew::{
    function_component,
    html,
    Html,
    KeyboardEvent,
    NodeRef,
    use_state
};
use web_sys::HtmlInputElement;

// use gloo_console::log;
// use wasm_bindgen::JsValue;

pub mod graph;
pub mod graph_impl;
pub mod computation_graph;


#[function_component(App)]
fn app() -> Html {
    let differential = use_state(|| 0u64);
    let error = use_state(|| None::<String>);
    let input_ref: NodeRef = NodeRef::default();

    let handle_click = {
        let differential = differential.clone();
        let error = error.clone();
        let input_ref = input_ref.clone();
        move || {
            let input_element: HtmlInputElement = input_ref.cast().unwrap();
            let value = &input_element.value();

            match u64::from_str_radix(value, 16) {
                Ok(num) => {
                    differential.set(num);
                    error.set(None);
                },
                Err(_) => {
                    error.set(Some("Invalid hexadecimal input".to_string()));
                }
            }
        }
    };

    let onkeydown = {
        let handle_click = handle_click.clone();
        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                handle_click();
            }
        }
    };


    html! {
        <div style="display: flex; align-items: flex-start; position: relative">
            <graph::Graph differential={*differential} />
            <div style="position: sticky; top: 0;">
                <label>{"Differential 0x:"}</label>
                <input ref={input_ref.clone()} type="text" placeholder="Enter text" onkeydown={onkeydown} />
                {
                    if let Some(error_msg) = &*error {
                        html! { <div class="error">{error_msg}</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
