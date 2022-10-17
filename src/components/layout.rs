use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ThreeColProps {
    //TODO: pass in three children
}

#[styled_component(ThreeCol)]
pub fn three_col(props: &ThreeColProps) {}
