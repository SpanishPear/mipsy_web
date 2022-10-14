use stylist::yew::*;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello, world!"}</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
