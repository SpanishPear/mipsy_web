// due to a bug in stylist
#![allow(clippy::let_unit_value)]

use bounce::BounceRoot;
use mipsy_web::{components::app::App, setup_splits};
use stylist::yew::*;
use yew::prelude::*;

#[styled_component(AppRoot)]
fn app() -> Html {
    // on the first render, run the javascript
    // that enables panes to resize
    use_effect_with_deps(
        |_| {
            setup_splits();

            || ()
        },
        (),
    );

    html! {
        <BounceRoot>
            <App />
        </BounceRoot>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<AppRoot>();
}
