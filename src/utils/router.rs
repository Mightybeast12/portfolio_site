
use yew::prelude::*;
use yew_router::{prelude::*};
use crate::utils::pages::{HomePage,InspektPage};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/inspekt")]
    InspektPage,
    #[at("/contact")]
    ContactPage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage />},
        Route::InspektPage => html! { <InspektPage /> },
        _ => html! { <h1>{ "Page Not Found" }</h1> },
    }
}

#[function_component(RouterComponent)]
pub fn router_component() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}