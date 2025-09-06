//! Navigation bar components

use crate::constants::social::FULL_NAME;
use crate::routing::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Props for navigation bar component
#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    #[prop_or("static/logo_transparent.png".to_string())]
    pub logo_src: String,
    #[prop_or(FULL_NAME.to_string())]
    pub logo_alt: String,
}

/// Main navigation bar component with responsive hamburger menu
#[function_component(NavBar)]
pub fn navbar(props: &NavBarProps) -> Html {
    html! {
        <nav class="navbar-container">
            <div class="nav-container">
                <input class="checkbox" type="checkbox" id="menu-toggle" />
                <div class="hamburger-lines">
                    <span class="line line1"></span>
                    <span class="line line2"></span>
                    <span class="line line3"></span>
                </div>
                <ul class="menu-items">
                    <li><Link<Route> to={Route::HomePage}>{"Home"}</Link<Route>></li>
                    <li><Link<Route> to={Route::InspektPage}>{"Inspekt"}</Link<Route>></li>
                    <li><Link<Route> to={Route::LandingPage}>{"Landing"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Info}>{"Info"}</Link<Route>></li>
                </ul>
                <span class="navbar-img">
                    <Link<Route> to={Route::HomePage}>
                        <img src={props.logo_src.clone()} alt={props.logo_alt.clone()}/>
                    </Link<Route>>
                </span>
            </div>
        </nav>
    }
}

/// Navigation menu item props
#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    pub route: Route,
    pub label: String,
}

/// Individual navigation menu item component
#[function_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {
    html! {
        <li>
            <Link<Route> to={props.route.clone()}>{&props.label}</Link<Route>>
        </li>
    }
}
