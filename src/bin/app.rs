// due to a bug in stylist
#![allow(clippy::let_unit_value)]

use bounce::BounceRoot;

use mipsy_web::components::app::App;
use stylist::yew::*;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use yew::prelude::*;

#[styled_component(AppRoot)]
fn app() -> Html {
    html! {
        <BounceRoot>
            <App />
        </BounceRoot>
    }
}

fn main() {
    // if debug, use wasm_logger (it contains line numbers)
    // otherwise, use tracing_web
    if cfg!(debug_assertions) {
        wasm_logger::init(wasm_logger::Config::default());
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
            .with_writer(MakeConsoleWriter); // write events to the console

        let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .init(); // Install these as subscribers to tracing events
    }
    yew::start_app::<AppRoot>();
}
