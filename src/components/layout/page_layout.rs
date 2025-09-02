//! Page layout components

use crate::components::navigation::NavBar;
use yew::prelude::*;

/// Props for the main page layout
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub page_title: Option<String>,
    #[prop_or_default]
    pub show_navbar: bool,
}

/// Main layout component that wraps all pages
#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    // Set the page title if provided
    if let Some(title) = &props.page_title {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(title);
    }

    let show_navbar = props.show_navbar;
    let children = props.children.clone();

    html! {
        <div>
            if show_navbar {
                <NavBar />
            }
            <main>
                { for children.iter() }
            </main>
        </div>
    }
}

/// Props for a page container
#[derive(Properties, PartialEq)]
pub struct PageContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>,
}

/// A generic page container component
#[function_component(PageContainer)]
pub fn page_container(props: &PageContainerProps) -> Html {
    let class = props
        .class
        .as_deref()
        .unwrap_or("page-container")
        .to_string();
    let children = props.children.clone();

    html! {
        <div class={class}>
            { for children.iter() }
        </div>
    }
}

/// Props for the home page layout
#[derive(Properties, PartialEq)]
pub struct HomeLayoutProps {
    #[prop_or_default]
    pub children: Children,
}

/// Specialized layout component for the home page
#[function_component(HomeLayout)]
pub fn home_layout(props: &HomeLayoutProps) -> Html {
    let children = props.children.clone();

    html! {
        <div class="home-container">
            { for children.iter() }
        </div>
    }
}
