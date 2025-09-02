use crate::components::layout::Layout;
use crate::page_builder::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,

    #[at("/inspekt")]
    InspektPage,

    #[at("/contact")]
    ContactPage,

    #[at("/stock_tracker")]
    StockTrackerPage,

    #[at("/alula_bible")]
    AlulaBiblePage,

    #[at("/auto_ingest_page")]
    AutoIngestPage,

    #[at("/rust_site")]
    RustSite,

    #[at("/info")]
    Info,

    #[at("/landing")]
    LandingPage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => home_page::home_page(),
        Route::InspektPage => inspekt_showcase_page::inspekt_page(),
        Route::StockTrackerPage => stock_tracker_page::stock_tracker_page(),
        Route::AlulaBiblePage => alula_bible_page::alula_bible_page(),
        Route::AutoIngestPage => auto_ingest_page::auto_ingest_page(),
        Route::RustSite => rust_site::rust_site(),
        Route::ContactPage => contact_page::contact_page(),
        Route::LandingPage => landing_page::landing_page(),
        Route::Info => info_page::info_page(),
    }
}

#[function_component(RouterComponent)]
pub fn router_component() -> Html {
    html! {
        <BrowserRouter>
            <Layout show_navbar={true}>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}
