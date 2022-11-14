use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct TabProps {
    pub uri: String,
    pub selected: Option<String>,
}

#[styled_component(Tab)]
pub fn tab(TabProps { uri, selected }: &TabProps) -> Html {
    // TODO: move uri and display_name to a separate struct
    let display_name = uri.split('/').last().unwrap_or_default();

    html! {
        <StyledTab selected={*selected == Some(uri.to_string())}>
            <span>{display_name}</span>
        </StyledTab>
    }
}

#[derive(Properties, PartialEq)]
pub struct StyledTabProps {
    pub selected: bool,
    pub children: Children,
}

#[styled_component(StyledTab)]
pub fn styled_tab(StyledTabProps { selected, children }: &StyledTabProps) -> Html {
    html! {
        <li
            style={ if *selected { "color: #fff; background-color: #1e1e1e; border-bottom: none;" } else { "" } }
            class={css!(r#"
                padding: 5px 10px;
                min-width: 100px;
                text-align: center;
                border-bottom: 1px solid #000;
                cursor: pointer;
                background-color: #fff;
                user-select: none;
                display: flex;
                align-items: center;
                background-color: #2d2d2d;
                color: #666666; 
                font-family: 'Roboto', sans-serif;
                justify-content: space-between;
            "#)}
        >

            {   for children.iter() }
            <span
                class={css!(r#"
                    float: right;
                    cursor: pointer;
                    user-select: none;
                    padding: 2px 4px;
                    border-radius: 25%;
                    color: #fff;
                    &:hover {
                        background-color: darkgray;
                    }
                "#)}
            >
                {"✕"}
            </span>
        </li>
    }
}