use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResizableLayoutProps {
    pub children: Children,
}

#[styled_component(ThreeColResizable)]
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
                { for props.children.iter().enumerate().map(|(index, item)| {
                    match index {
                        0 => {
                            html!{
                            <div class={css!(r#"
                               border: 1px solid black;
                           "#)} id="left">
                               {item}
                           </div>
                            }
                        }
                        1 => {
                            html! {
                               <div id="middle">
                                   {item}
                               </div>
                            }
                        }
                        2 => {
                            html! {
                               <div  class={css!(r#"
                                   border: 1px solid black;
                                   min-width: 100px;
                               "#)} id="right">
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
