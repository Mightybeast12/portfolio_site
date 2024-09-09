
use yew::prelude::*;
use yew_router::prelude::*;
use crate::utils::pages::*;
use super::layout::Layout; 

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/inspekt")]
    InspektPage,
    #[at("/contact")]
    ContactPage,
    #[at["/info"]]
    Info,

}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage />},
        Route::InspektPage => html! { <InspektPage /> },
        Route::ContactPage => html! { <ContactPage /> },
        Route::Info => html! { <Info /> },
    }
}

#[function_component(RouterComponent)]
pub fn router_component() -> Html {
    html! {
        <BrowserRouter>
            <Layout> 
                <Switch<Route> render={switch} />
                <Redirect<Route> to={Route::HomePage} />
            </Layout> 
        </BrowserRouter>
    }
}