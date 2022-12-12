use bounce::use_atom;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::components::decompiled_container::DecompiledContainer;
use crate::state::State;

use super::containers::{DataContainer, EditorContainer};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct ThreeTabSwitcherProps {
    pub editor_container: VChild<EditorContainer>,
    pub decompiled_container: VChild<DecompiledContainer>,
    pub data_container: VChild<DataContainer>,
}

#[styled_component(ThreeTabSwitcher)]
pub fn three_tab_switcher(
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

    let binary_exists = use_atom::<State>();
    let binary_exists = !matches!(*binary_exists, State::NoBinary);
    let is_disabled = |index| {
        if index != 0 {
            !binary_exists
        } else {
            false
        }
    };

    // Note - this cannot be a separate function
    // or the Style is dropped
    let button_classes = |index| {
        let style = Style::new(
            r#"
            flex: 1 1 0px;
            border-radius: 0px;
            border: 1px solid black;
            &:hover {
                cursor: pointer;
                background-color: #f5c6c6;
            }
            
        "#,
        );
        let is_disabled = is_disabled(index);

        let style = style.unwrap();
        let second_class = Style::new(if *displayed == index {
            "background-color: #f5c6c6;"
        } else {
            "background-color: #fee2e2;"
        })
        .unwrap();

        if is_disabled {
            let disabled_style = Style::new(
                r#"
                background-color: #e9ecef;
                color: #6c757d;
                &:hover {
                    cursor: not-allowed;
                    background-color: #e9ecef;
                }
            "#,
            );
            let disabled_style = disabled_style.unwrap();
            classes!(style, second_class, disabled_style)
        } else {
            classes!(style, second_class)
        }
    };

    html! {
        <div id="three-tab-switcher__container" class={css!(r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100%;
            justify-content: space-between;
            border-radius: 5px;
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
            <div id="three-tab-switcher__buttons" class={css!(r#"
                display: flex;
                flex-direction: row;
                width: 100%;
                height: 4%;
            "#)}>
                <button
                    class={button_classes(0)}
                    id="three-tab-switcher__editor"
                    onclick={click_callback(0)}
                    disabled={is_disabled(0)}
                >
                    {"editor"}
                </button>
                <button
                    id="three-tab-switcher__decompiled"
                    onclick={click_callback(1)}
                    class={button_classes(1)}
                    disabled={is_disabled(1)}

                >
                    {"decompiled"}
                </button>
                <button
                    id="three-tab-switcher__data"
                    onclick={click_callback(2)}
                    class={button_classes(2)}
                    disabled={is_disabled(2)}
                >
                    {"data"}
                </button>
            </div>
        </div>
    }
}
