//! Project showcase card components

use crate::components::buttons::NavigationButton;
use crate::constants::ui::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// Props for a project showcase card
#[derive(Properties, PartialEq)]
pub struct ProjectCardProps<R: Routable> {
    pub title: String,
    pub content: Html,
    pub route: R,
    #[prop_or(CHECK_OUT_BUTTON_TEXT.to_string())]
    pub button_text: String,
    #[prop_or(MARKDOWN_CONTAINER_CLASS.to_string())]
    pub container_class: String,
}

/// A reusable project card component with navigation button
#[function_component(ProjectCard)]
pub fn project_card<R: Routable + 'static>(props: &ProjectCardProps<R>) -> Html {
    html! {
        <div class={props.container_class.clone()}>
            {props.content.clone()}
            <NavigationButton<R>
                route={props.route.clone()}
                text={props.title.clone()}
                button_text={props.button_text.clone()}
            />
        </div>
    }
}

/// Props for a simple content card
#[derive(Properties, PartialEq)]
pub struct ContentCardProps {
    pub content: Html,
    #[prop_or(MARKDOWN_CONTAINER_CLASS.to_string())]
    pub container_class: String,
}

/// A simple content card without navigation
#[function_component(ContentCard)]
pub fn content_card(props: &ContentCardProps) -> Html {
    html! {
        <div class={props.container_class.clone()}>
            {props.content.clone()}
        </div>
    }
}
