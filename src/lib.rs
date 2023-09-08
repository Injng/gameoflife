use yew::prelude::*;

mod universe;
mod node;

use universe::Universe;

#[macro_use]
extern crate lazy_static;

#[function_component]
pub fn App() -> Html {
    html! {
        <Universe />
    }
}

