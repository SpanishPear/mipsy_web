use crate::agent::MipsyWebWorker;
use crate::{components::layout::ResizableLayout, editor::component::Editor};
use gloo_worker::Spawnable;
use js_sys::Promise;
use stylist::css;
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
        //TODO: use channels to send messages/await
        // responses from the worker
        log::error!("{:?}", a);
    });

    html! {
        <AppContainer>
            <ResizableLayout
                menu_container={{ html_nested! {
                    <MenuContainer />
                }}}
                editor_container={{ html_nested!{
                    <EditorContainer />
                }}}
                runtime_container={{html_nested!{
                    <RuntimeContainer />
                }}}
            >
            </ResizableLayout>
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
            min-width: 100vw;
            min-height: 100vh;
            height: 100%;
            width: 100%;
            background-color: pink;
        "#)}>
            {
                for props.children.iter()
            }
        </div>
    }
}

#[styled_component(MenuContainer)]
pub fn menu_container() -> Html {
    html! {
        <div class={css!(r#"
            background-color: red;
            width: 100%;
            height: 100%;
        "#)}>
            {"menu"}
        </div>
    }
}

#[styled_component(EditorContainer)]
pub fn editor_container() -> Html {
    let styles: String = "width: 100%; height: 100%; max-height: 90vh;".into();
    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
        "#)}>
            <Editor {styles}/>
        </div>
    }
}

#[styled_component(RuntimeContainer)]
pub fn runtime_container() -> Html {
    html! {
        <div class={css!(r#"
            background-color: green;
        "#)}>
            {"runtime"}
        </div>
    }
}
