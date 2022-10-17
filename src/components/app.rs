use stylist::yew::styled_component;
use yew::prelude::*;
use yew_agent::{use_bridge, UseBridgeHandle};

use crate::agent::{ToWorker, Worker};

#[styled_component(App)]
pub fn app() -> Html {
    let bridge: UseBridgeHandle<Worker> = use_bridge(move |response| {
        log::info!("{:?}", response);
    });

    use_effect(move || {
        bridge.send(ToWorker::Ping);
        || ()
    });
    html! {
        <AppContainer>
            <div>

            </div>
        </AppContainer>
    }
}

#[derive(Properties, PartialEq)]
struct AppContainerProps {
    children: Children,
}

#[styled_component(AppContainer)]
fn app_container(props: &AppContainerProps) -> Html {
    html! {
        <div class={css!(r#"
            width: 100vw;
            height: 100vh;
            background-color: pink;
        "#)}>
            {
                for props.children.iter()
            }
        </div>
    }
}
