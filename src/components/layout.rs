use crate::components::app::{EditorContainer, MenuContainer, RuntimeContainer};
use stylist::yew::styled_component;
use yew::{prelude::*, virtual_dom::VChild};

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
                display: flex;
                flex-direction: row;
                max-width: 98vw;
            "#)}>
                <div class={css!(r#"
                    border: 1px solid black;
                "#)} id="left">
                    {props.menu_container.clone()}
                </div>

                <div class={css!(r#"
                "#)} id="middle">
                    {props.editor_container.clone()}
                </div>

                <div class={css!(r#"
                    border: 1px solid black;
                    min-width: 100px;
                "#)} id="right">
                    {props.runtime_container.clone()}
                </div>
            </div>
        </div>
    }
}
