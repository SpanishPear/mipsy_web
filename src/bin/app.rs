// due to a bug in stylist
#![allow(clippy::let_unit_value)]

use bounce::BounceRoot;
use mipsy_web::{
    setup_splits,
    components::app::App
};
use stylist::yew::*;
use yew::prelude::*;

#[styled_component(AppRoot)]
fn app() -> Html {

    use_effect(|| {
        setup_splits(); 
        
        || ()
    });

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
