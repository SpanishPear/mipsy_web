use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(MenuBar)]
pub fn menubar() -> Html {
    html! {
        <p>{"Hello World"}</p>
    }
}
