//! Loading spinner component

use crate::styles::components::loading as styles;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub size: LoadingSize,
}

#[derive(Clone, PartialEq)]
pub enum LoadingSize {
    Small,
    Medium,
    Large,
}

impl Default for LoadingSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let size_style = match props.size {
        LoadingSize::Small => styles::loading_small(),
        LoadingSize::Medium => styles::loading_medium(),
        LoadingSize::Large => styles::loading_large(),
    };

    html! {
        <div class={classes!(
            styles::loading_base(),
            size_style,
            props.class.clone()
        )}></div>
    }
}
