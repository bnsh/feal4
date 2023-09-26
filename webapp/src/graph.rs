// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 */

use yew::{
    function_component,
    html,
    Html,
    Properties
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub differential: i64
}

#[function_component(Graph)]
pub fn app(props: &Props) -> Html {
    html! {
        <div>
            {format!("{}", props.differential)}
        </div>
    }
}
