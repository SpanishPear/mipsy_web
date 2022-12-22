use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IconButtonProps {
    pub onclick: Option<Callback<MouseEvent>>,
    pub class: Option<Classes>,
    pub children: Children,
}

#[styled_component(IconButton)]
pub(crate) fn icon_button(props: &IconButtonProps) -> Html {
    html! {
        <button class={classes!(css!(r#"
            width: 2.5rem;
            height: 3rem;
            background-color: transparent;
            border: none;
            &:hover {
                cursor: pointer;
                color: white;
            }
        "#), props.class.clone())} onclick={props.onclick.clone()}>
            { for props.children.iter() }
        </button>
    }
}

#[styled_component(StopIconOutline)]
pub(crate) fn stop_icon_outline() -> Html {
    html! {
        <svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M2 2v12h12V2H2zm10.75 10.75h-9.5v-9.5h9.5v9.5z"/>
        </svg>
    }
}

#[styled_component(StopIconFilled)]
pub(crate) fn stop_icon_filled() -> Html {
    html! {
        <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clip-rule="evenodd" />
        </svg>
    }
}

#[styled_component(RunIcon)]
pub(crate) fn run_icon() -> Html {
    html! {
        <svg x="0px" y="0px"
           viewBox="0 0 297 297" style="enable-background:new 0 0 297 297;">
            <path d="M148.5,0C66.486,0,0,66.486,0,148.5S66.486,297,148.5,297S297,230.514,297,148.5S230.514,0,148.5,0z M235.79,161.984
              l-78.501,45.323c-2.422,1.398-5.159,2.137-7.916,2.137c0,0-0.167,0-0.168,0c-8.752,0-16.039-7.12-16.039-15.872V186.4
              l-36.044,20.907c-2.422,1.398-5.242,2.137-7.999,2.137c-8.753,0-15.956-7.12-15.956-15.872v-90.645
              c0-8.752,7.287-15.872,16.04-15.872c2.757,0,5.411,0.738,7.833,2.137l36.128,20.907v-7.172c0-8.752,7.287-15.872,16.04-15.872
              c2.757,0,5.577,0.738,7.999,2.137l78.543,45.323c4.966,2.867,7.951,8.001,7.951,13.734S240.756,159.117,235.79,161.984z"/>
        </svg>
    }
}

#[styled_component(RunIconOutline)]
pub(crate) fn run_icon_outline() -> Html {
    html! {
        <svg viewBox="0 0 16 16" fill="currentColor">
          <path fill-rule="evenodd" clip-rule="evenodd" d="M4.25 3l1.166-.624 8 5.333v1.248l-8 5.334-1.166-.624V3zm1.5 1.401v7.864l5.898-3.932L5.75 4.401z"/>
        </svg>
    }
}

#[styled_component(StepBackIcon)]
pub(crate) fn step_back_icon() -> Html {
    html! {
        <svg style="width: 1.6rem" viewBox="0 0 16 16" version="1.1">
          <rect width="16" height="16" id="icon-bound" fill="none" />
          <path d="M2,15h2V1H2V15z M14.4,2.4L13,1L6,8l7,7l1.4-1.4L8.8,8L14.4,2.4z" />
        </svg>
    }
}

#[styled_component(StepForwardIcon)]
pub(crate) fn step_forward_icon() -> Html {
    html! {
        <svg viewBox="0 0 36 36" version="1.1"  preserveAspectRatio="xMidYMid meet">
            <path d="M7.08,6.52a1.68,1.68,0,0,0,0,2.4L16.51,18,7.12,27.08a1.7,1.7,0,0,0,2.36,2.44h0L21.4,18,9.48,6.47A1.69,1.69,0,0,0,7.08,6.52Z" class="clr-i-outline clr-i-outline-path-1"></path><path d="M26.49,5a1.7,1.7,0,0,0-1.7,1.7V29.3a1.7,1.7,0,0,0,3.4,0V6.7A1.7,1.7,0,0,0,26.49,5Z" class="clr-i-outline clr-i-outline-path-2"></path>
            <rect x="0" y="0" width="36" height="36" fill-opacity="0"/>
        </svg>
    }
}

#[styled_component(ResetIcon)]
pub(crate) fn reset_icon() -> Html {
    html! {
        <svg viewBox="0 0 16 16" fill="currentColor">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M12.75 8a4.5 4.5 0 0 1-8.61 1.834l-1.391.565A6.001 6.001 0 0 0 14.25 8 6 6 0 0 0 3.5 4.334V2.5H2v4l.75.75h3.5v-1.5H4.352A4.5 4.5 0 0 1 12.75 8z"/>
        </svg>
    }
}
