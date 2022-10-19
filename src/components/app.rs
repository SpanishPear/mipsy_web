use crate::agent::MipsyWebWorker;
use gloo_worker::Spawnable;
use js_sys::Promise;
use stylist::yew::styled_component;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let bridge = MipsyWebWorker::spawner()
        .callback(move |m| {
            // this runs in the main browser thread
            // and does not block the web worker
            log::info!("received message from worker: {:?}", m);
        })
        .spawn("/worker.js");

    spawn_local(async move {
        bridge.send(crate::agent::ToWorker::Ping);
        // We need to hold the bridge until the worker resolves.
        let promise = Promise::new(&mut |_, _| {});
        let a = JsFuture::from(promise).await;
        log::error!("{:?}", a);
    });

    html! {
        <AppContainer>
            //<ModeMenu/>
            //<EditorArea />
            // TODO: this will be renamed for sure
            //<RegistersAndTerm />
            <div> </div>
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
