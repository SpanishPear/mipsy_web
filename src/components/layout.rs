use crate::components::app::{EditorContainer, MenuContainer, RuntimeContainer};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Properties, PartialEq)]
pub struct ResizableLayoutProps {
    pub menu_container: VChild<MenuContainer>,
    pub editor_container: VChild<EditorContainer>,
    pub runtime_container: VChild<RuntimeContainer>,
}

#[styled_component(ResizableLayout)]
pub fn resizable_layout(props: &ResizableLayoutProps) -> Html {
    html! {
        <div class={css!(r#"
            height: 100%;
            width: 100%;
            padding: 30px;
            display: grid;
        "#)}>
            <div class={css!(r#"
                flex: 1;
                display: flex;
                flex-direction: row;
            "#)}>
                <div class={css!(r#"
                    border: 1px solid black;
                    min-width: 100px;
                "#)}>
                    {props.menu_container.clone()}
                </div>

                <div class={css!(r#"
                    border: 1px solid black;
                    flex-grow: 1;
                "#)}>
                    {props.editor_container.clone()}
                </div>

                <div class={css!(r#"
                    border: 1px solid black;
                    min-width: 100px;
                "#)}>
                    {props.runtime_container.clone()}
                </div>
            </div>
        </div>
    }
}
