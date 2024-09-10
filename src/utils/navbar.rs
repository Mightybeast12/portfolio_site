use yew::prelude::*;
use crate::utils::router::Route; 
use yew_router::prelude::*; 

#[function_component(NavBar)]
pub fn navbar_component() -> Html {
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
                    <img src="static/logo_transparent.png" alt="Firat Honca"/>
                </span>
            </div>
        </nav>
    }
}