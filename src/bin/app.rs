use mipsy_web::editor::component::Editor;
use stylist::yew::*;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello, world!"}</h1>
            <Editor />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
