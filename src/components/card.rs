//! Modern card component

use crate::styles::components::card as styles;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub hoverable: bool,
    #[prop_or_default]
    pub elevated: bool,
    #[prop_or_default]
    pub compact: bool,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let mut card_classes = vec![styles::card_base()];

    if props.hoverable {
        card_classes.push(styles::card_hoverable());
    }

    if props.elevated {
        card_classes.push(styles::card_elevated());
    }

    if props.compact {
        card_classes.push(styles::card_compact());
    }

    html! {
        <div class={classes!(card_classes, props.class.clone())}>
            {for props.children.iter()}
        </div>
    }
}
