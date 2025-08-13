use crate::styles::layout::navbar as styles;
use crate::theme::ThemeToggle;
use crate::utils::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn navbar_component() -> Html {
    html! {
        <nav class={styles::navbar_main()}>
            <div class={styles::navbar_container()}>
                <div class={styles::navbar_brand()}>
                    <Link<Route> to={Route::HomePage}>
                        <img src="static/logo_transparent.png" alt="Firat Honca"/>
                    </Link<Route>>
                    <Link<Route> to={Route::HomePage} classes={styles::navbar_brand_text()}>
                        {"Firat Honca"}
                    </Link<Route>>
                </div>

                <ul class={classes!(styles::navbar_menu(), styles::navbar_menu_mobile())}>
                    <li class={styles::navbar_item()}>
                        <Link<Route> to={Route::HomePage}>{"Home"}</Link<Route>>
                    </li>
                    <li class={styles::navbar_item()}>
                        <Link<Route> to={Route::InspektPage}>{"Inspekt"}</Link<Route>>
                    </li>
                    <li class={styles::navbar_item()}>
                        <Link<Route> to={Route::LandingPage}>{"Landing"}</Link<Route>>
                    </li>
                    <li class={styles::navbar_item()}>
                        <Link<Route> to={Route::Info}>{"Info"}</Link<Route>>
                    </li>
                </ul>

                <div class={styles::navbar_actions()}>
                    <ThemeToggle />
                </div>

                <div class={styles::navbar_hamburger()}>
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>
        </nav>
    }
}
