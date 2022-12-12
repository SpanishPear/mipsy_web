use crate::components::sidebar::SideBar;
use crate::editor::component::Editor;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppContainerProps {
    pub children: Children,
}

#[styled_component(AppContainer)]
pub fn app_container(props: &AppContainerProps) -> Html {
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

#[styled_component(DataContainer)]
pub fn data() -> Html {
    html! {
         <>{"data"}</>
    }
}
