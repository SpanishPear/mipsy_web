use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RuntimeContainerProps {
    pub children: Children,
}

#[styled_component(RuntimeContainer)]
pub fn resizable_layout(props: &RuntimeContainerProps) -> Html {
    html! {
        <div class={css!(r#"
            height: 100%;
            width: 100%;
            display: grid;
        "#)}>
            <div class={css!(r#"
                max-width: 98vw;
            "#)}>
                { for props.children.iter().enumerate().map(|(index, item)| {
                    match index {
                        0 => {
                            html!{
                            <div class={css!(r#"
                               border: 1px solid black;
                           "#)} id="runtime_top">
                               {item}
                           </div>
                            }
                        }
                        1 => {
                            html! {
                               <div  class={css!(r#"
                                   border: 1px solid black;
                               "#)} id="runtime_bottom">
                                   {item}
                               </div>
                            }
                        }
                        _ => unreachable!(),

                    }
                })}
            </div>
        </div>
    }
}
