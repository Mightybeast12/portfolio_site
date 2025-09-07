use crate::components::layout::Layout;
use crate::pages::{core, projects};
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

    // New repository showcase pages
    #[at("/portfolio_site")]
    PortfolioSitePage,

    #[at("/firo_logger")]
    FiroLoggerPage,

    #[at("/css_to_rust_converter")]
    CssToRustConverterPage,

    #[at("/game_score_tracker")]
    GameScoreTrackerPage,

    #[at("/gitlab_terraform")]
    GitlabTerraformPage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => core::home::home_page(),
        Route::InspektPage => projects::inspekt::inspekt_page(),
        Route::StockTrackerPage => projects::stock_tracker::stock_tracker_page(),
        Route::AlulaBiblePage => projects::alula_bible::alula_bible_page(),
        Route::AutoIngestPage => projects::auto_ingest::auto_ingest_page(),
        Route::RustSite => projects::rust_site::rust_site(),
        Route::ContactPage => core::contact::contact_page(),
        Route::Info => core::info::info_page(),
        Route::PortfolioSitePage => projects::portfolio_site::portfolio_site_page(),
        Route::FiroLoggerPage => projects::firo_logger::firo_logger_page(),
        Route::CssToRustConverterPage => {
            projects::css_to_rust_converter::css_to_rust_converter_page()
        }
        Route::GameScoreTrackerPage => projects::game_score_tracker::game_score_tracker_page(),
        Route::GitlabTerraformPage => projects::gitlab_terraform::gitlab_terraform_page(),
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
