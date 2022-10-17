use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <AppContainer>
            <ModeMenu/>
            <EditorArea />
            // TODO: this will be renamed for sure
            <RegistersAndTerm />
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
