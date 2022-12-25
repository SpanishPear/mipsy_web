use crate::{
    agent::{worker::MipsyWebWorker, FromWorker},
    components::{
        decompiled_container::DecompiledContainer,
        layout::ThreeTabSwitcher,
        layout::{
            containers::{AppContainer, DataContainer, EditorContainer, MenuContainer},
            ThreeColResizable,
        },
        runtime::{container::RuntimeContainer, register_tab_switcher::RegisterTabSwitcher},
    },
    editor::files::{FileList, FileListAction},
    setup_splits,
    state::{
        app::{State, StateAction},
        breakpoints::Breakpoints,
    },
    SplitContainer,
};
use bounce::{use_atom, use_atom_setter, use_slice, use_slice_dispatch};
use gloo_worker::{Spawnable, WorkerBridge};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    // on the first render, run the javascript
    // that enables panes to resize
    // store a handle for future use
    let split_container = use_atom::<SplitContainer>();
    let breakpoint_setter = use_atom_setter::<Breakpoints>();
    let files = use_slice::<FileList>();

    use_effect_with_deps(
        move |_| {
            log::debug!("running setup_splits");
            let container = SplitContainer {
                handle: setup_splits(),
            };
            split_container.set(container);

            log::debug!("setting initial main.s content");
            files.dispatch(FileListAction::Append(
                "main.s".into(),
                include_str!("../main.s").into(),
            ));

            files.dispatch(FileListAction::ToggleCompile(0));
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
                    FromWorker::Decompiled(res) => {
                        // we have decompiled and binary
                        // from succesful compilation
                        // so set the state to a new compiled state
                        state_dispatch(StateAction::InitialiseFromDecompiled(res));
                    }
                    FromWorker::Breakpoints(res) => breakpoint_setter(res),
                    FromWorker::Pong(_) => {}
                    _ => {}
                }
            })
            .spawn("/worker.js")
    });

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
                    <RuntimeContainer>
                        <RegisterTabSwitcher>
                            <div>{"used registers"}</div>
                            <div>{"all regs"}</div>
                        </RegisterTabSwitcher>
                        <div>{"bottom"}</div>
                    </RuntimeContainer>
            </ThreeColResizable>
        </AppContainer>
        </ContextProvider<WorkerBridge<MipsyWebWorker>>>
    }
}
