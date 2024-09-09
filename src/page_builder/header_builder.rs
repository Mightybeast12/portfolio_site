use yew::prelude::*;
use crate::utils::router::Route; 
use yew_router::prelude::*; 


fn firat_header() -> Html {
    html! {
        <nav class="navbar">
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
                    <li><Link<Route> to={Route::ContactPage}>{"Contact"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Info}>{"Info"}</Link<Route>></li>
                </ul>
                <span class="navbar-logo">{"Firat Honca"}</span>
            </div>
        </nav>
    }
}
pub fn build_header() -> Html {
    firat_header()
}
