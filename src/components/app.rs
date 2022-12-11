use crate::{
    agent::worker::MipsyWebWorker,
    components::{
        layout::ResizableLayout,
        sidebar::SideBar,
        three_tab_switcher::{DataContainer, DecompiledContainer, ThreeTabSwitcher},
    },
    editor::{
        component::Editor,
        files::{FileList, FileListAction},
        MipsyCodeEditorLink,
    },
    setup_splits, SplitContainer,
};
use bounce::{use_atom, use_slice};
use gloo_worker::{Spawnable, WorkerBridge};
use js_sys::Promise;
use stylist::{css, yew::styled_component};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let code_editor_link = use_atom::<MipsyCodeEditorLink>();
    // on the first render, run the javascript
    // that enables panes to resize
    // store a handle for future use
    let split_container = use_atom::<SplitContainer>();
    use_effect_with_deps(
        move |_| {
            log::debug!("running setup_splits");
            let container = SplitContainer {
                handle: setup_splits(),
            };
            split_container.set(container);

            || ()
        },
        (),
    );

    let bridge: UseStateHandle<WorkerBridge<MipsyWebWorker>> = use_state(|| {
        MipsyWebWorker::spawner()
            .callback(move |m| {
                // this runs in the main browser thread
                // and does not block the web worker
                log::info!("received message from worker: {:?}", m);
            })
            .spawn("/worker.js")
    });

    {
        let bridge = bridge.clone();
        spawn_local(async move {
            bridge.send(crate::agent::ToWorker::Ping);
            // We need to hold the bridge until the worker resolves.
            let promise = Promise::new(&mut |_, _| {});
            let a = JsFuture::from(promise).await;
            //TODO: use channels to send messages/await
            // responses from the worker
            log::error!("{:?}", a);
        });
    }

    let files = use_slice::<FileList>();
    use_effect_with_deps(
        move |_| {
            log::debug!("setting initial main.s content");
            code_editor_link.link.with_editor(|editor| {
                let model = editor.get_model().expect("editor has no model");

                model.set_value(include_str!("../main.s"));

                files.dispatch(FileListAction::Append(
                    "main.s".into(),
                    include_str!("../main.s").into(),
                ));
                files.dispatch(FileListAction::ToggleCompile(0));
            });

            || ()
        },
        (),
    );

    html! {
        <ContextProvider<WorkerBridge<MipsyWebWorker>> context={(*bridge).clone()}>
        <AppContainer>
            <ResizableLayout
                menu_container={{ html_nested! {
                    <MenuContainer />
                }}}
                three_tab_switcher={{ html_nested!{
                    <ThreeTabSwitcher
                        editor_container={{ html_nested!{
                            <EditorContainer />
                        }}}
                        decompiled_container={{ html_nested!{
                            <DecompiledContainer />
                        }}}
                        data_container={{ html_nested!{
                            <DataContainer />
                        }}}
                    />
                }}}
                runtime_container={{html_nested!{
                    <RuntimeContainer />
                }}}
            >
            </ResizableLayout>
        </AppContainer>
        </ContextProvider<WorkerBridge<MipsyWebWorker>>>
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
            background-color: #fee2e2;
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
            width: 100%;
            height: 100%;
        "#)}>
           <SideBar />
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
            min-width: 100%;
            min-height: 100%;
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
