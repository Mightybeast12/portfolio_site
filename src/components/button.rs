//! Modern button component

use crate::styles::components::button as styles;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub disabled: bool,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_style = match props.variant {
        ButtonVariant::Primary => styles::button_primary(),
        ButtonVariant::Secondary => styles::button_secondary(),
        ButtonVariant::Ghost => styles::button_ghost(),
    };

    html! {
        <button
            class={classes!(
                styles::button_base(),
                variant_style,
                props.class.clone()
            )}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            {for props.children.iter()}
        </button>
    }
}
