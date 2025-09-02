//! Action button components for external links and navigation

use crate::constants::ui::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// Props for the external link button component
#[derive(Properties, PartialEq)]
pub struct ExternalButtonProps {
    pub href: String,
    pub image_url: String,
    pub alt_text: Option<String>,
    #[prop_or(BUTTON_4_CLASS.to_string())]
    pub class: String,
    #[prop_or(LOGO_SIZE.to_string())]
    pub image_size: String,
}

/// A button component for external links with an icon
#[function_component(ExternalButton)]
pub fn external_button(props: &ExternalButtonProps) -> Html {
    let alt_text = props
        .alt_text
        .clone()
        .unwrap_or_else(|| "External Link".to_string());

    html! {
        <a href={props.href.clone()} target="_blank" rel="noopener noreferrer">
            <button class={props.class.clone()} role="button">
                <img
                    src={props.image_url.clone()}
                    alt={alt_text}
                    style={format!("width: {}; height: {};", props.image_size, props.image_size)}
                />
            </button>
        </a>
    }
}

/// Props for the navigation button component
#[derive(Properties, PartialEq)]
pub struct NavigationButtonProps<R: Routable + 'static> {
    pub route: R,
    pub text: String,
    #[prop_or(CHECK_OUT_BUTTON_TEXT.to_string())]
    pub button_text: String,
    #[prop_or(BUTTON_20_CLASS.to_string())]
    pub button_class: String,
    #[prop_or(BUTTON_20_CONTAINER_CLASS.to_string())]
    pub container_class: String,
    #[prop_or(BUTTON_LINK_CLASS.to_string())]
    pub link_class: String,
}

/// A navigation button component with arrow styling
#[function_component]
pub fn NavigationButton<R: Routable + 'static>(props: &NavigationButtonProps<R>) -> Html {
    let button_text = props.button_text.clone();
    let button_class = props.button_class.clone();
    let container_class = props.container_class.clone();
    let link_class = props.link_class.clone();
    let route = props.route.clone();

    html! {
        <div class={container_class}>
            <Link<R> to={route} classes={Classes::from(link_class)}>
                <div class={button_class}>
                    <span class="button-20-text">{button_text}</span>
                    <span class="button-20-arrow"></span>
                </div>
            </Link<R>>
        </div>
    }
}
