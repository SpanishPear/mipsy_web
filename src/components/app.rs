use crate::{
    agent::{worker::MipsyWebWorker, FromWorker},
    components::{
        decompiled_container::DecompiledContainer,
        layout::{containers::RuntimeContainer, ThreeTabSwitcher},
        layout::{
            containers::{AppContainer, DataContainer, EditorContainer, MenuContainer},
            ThreeColResizable,
        },
    },
    editor::{
        files::{FileList, FileListAction},
        MipsyCodeEditorLink,
    },
    setup_splits,
    state::app::{State, StateAction},
    SplitContainer,
};
use bounce::{use_atom, use_slice, use_slice_dispatch};
use gloo_worker::{Spawnable, WorkerBridge};
use js_sys::Promise;
use stylist::yew::styled_component;
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

    let state_dispatch = use_slice_dispatch::<State>();
    let bridge: UseStateHandle<WorkerBridge<MipsyWebWorker>> = use_state(|| {
        MipsyWebWorker::spawner()
            .callback(move |m| {
                // this runs in the main browser thread
                // and does not block the web worker
                log::debug!("received message from worker: {:?}", m);
                match m {
                    FromWorker::Decompiled(response) => {
                        // we have decompiled and binary
                        // from succesful compilation
                        // so set the state to a new compiled state
                        state_dispatch(StateAction::InitialiseFromDecompiled(response));
                    }
                    FromWorker::Pong(_) => {}
                    _ => {}
                }
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
            <ThreeColResizable>
                    <MenuContainer />
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
                    <RuntimeContainer />
            </ThreeColResizable>
        </AppContainer>
        </ContextProvider<WorkerBridge<MipsyWebWorker>>>
    }
}
