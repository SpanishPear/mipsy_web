use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(StopIconOutline)]
pub(crate) fn stop_icon_outline() -> Html {
    html! {
        <svg class={css!(r#"
            width: 1.3rem;
            height: 1.3rem;
        "#)} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
          <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
        </svg>
    }
}

#[styled_component(StopIconFilled)]
pub(crate) fn stop_icon_filled() -> Html {
    html! {
        <svg class={css!(r#"
            width: 1.3rem;
            height: 1.3rem;
        "#)} viewBox="0 0 20 20" fill="currentColor">
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

#[styled_component(StepBackIcon)]
pub(crate) fn step_back_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
    }
}

#[styled_component(StepForwardIcon)]
pub(crate) fn step_forward_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
    }
}
