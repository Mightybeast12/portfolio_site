use yew::prelude::*;
use yew_router::{prelude::*};

use cv_portfolio_site::*;
use page_builder::{home_page, inspekt_showcase_page,
                   header_builder};

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    HomePage,
    #[at("/inspekt")]
    InspektPage,
    #[at("/contact")]
    ContactPage,
}

#[function_component(InspektPage)]
fn display_inspekt_page() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::HomePage));
    html! {
        <div>
            {inspekt_showcase_page::inspekt_page()} 
            <button {onclick}>{ "Go to Home" }</button>
        </div>
    }
}
#[function_component(HomePage)] 
fn displa_home_page() -> Html {
    home_page::create_home_page() 
} 

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage />},
        Route::InspektPage => html! { <InspektPage /> },
        _ => html! { <h1>{ "Page Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div>
                <div> {header_builder::build_header()} </div> 
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::HomePage}>{ "Home" }</Link<Route>></li>
                        <li><Link<Route> to={Route::InspektPage}>{ "Inspekt" }</Link<Route>></li>
                    </ul>
                </nav>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
