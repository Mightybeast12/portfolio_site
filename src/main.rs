use yew::prelude::*;
use yew_router::prelude::*;
 
use cv_portfolio_site::*; 
use page_builder::{home_page, inspekt_showcase_page}; 

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/inspekt")]
    InspektPage,
    #[at("/contact")]
    ContactPage,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div>
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::HomePage}>{ "Home" }</Link<Route>></li>
                        <li><Link<Route> to={Route::InspektPage}>{ "Inspekt" }</Link<Route>></li>
                    </ul>
                </nav>
                <Switch<Route> >
                    <Route<Route, home_page> path={Route::HomePage} />
                    <Route<Route, inspekt_showcase_page> path={Route::InspektPage} />
                    // You can add a route for the ContactPage as well
                </Switch<Route>>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
