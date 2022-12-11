use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use super::app::EditorContainer;

#[styled_component(DecompiledContainer)]
pub fn decompiled() -> Html {
    html! {
         <>{"decompiled"}</>
    }
}

#[styled_component(DataContainer)]
pub fn data() -> Html {
    html! {
         <>{"data"}</>
    }
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ThreeTabSwitcherProps {
    pub editor_container: VChild<EditorContainer>,
    pub decompiled_container: VChild<DecompiledContainer>,
    pub data_container: VChild<DataContainer>,
}

#[styled_component(ThreeTabSwitcher)]
pub fn three_tab_swithcer(
    ThreeTabSwitcherProps {
        editor_container,
        decompiled_container,
        data_container,
    }: &ThreeTabSwitcherProps,
) -> Html {
    // a container with three possible display options
    // editor, decompiled, data
    // there should be three tab buttons at the buttom
    // that switch the display

    let displayed = use_state(|| 0);

    let click_callback = |index| {
        let displayed = displayed.clone();
        Callback::from(move |_| {
            displayed.set(index);
        })
    };

    html! {
        <div id="three-tab-switcher__container" class={css!(r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100%;
            justify-content: space-between;
        "#)}>
            <div id="three-tab-switcher__current_display" class={css!(r#"
                height: 90%;
                width: 100%;
            "#)}>
                {match *displayed {
                    0 => html!{{editor_container.clone()}},
                    1 => html!{{decompiled_container.clone()}},
                    2 => html!{{data_container.clone()}},
                    _ => unreachable!("invalid index"),
                }}
            </div>
            <div class="three-tab-switcher__buttons">
                <button id="three-tab-switcher__editor" onclick={click_callback(0)}>{"editor"}</button>
                <button id="three-tab-switcher__decompiled" onclick={click_callback(1)}>{"decompiled"}</button>
                <button id="three-tab-switcher__data" onclick={click_callback(2)}>{"data"}</button>
            </div>
        </div>
    }
}
