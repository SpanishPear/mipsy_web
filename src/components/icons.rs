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
