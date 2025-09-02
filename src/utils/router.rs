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
        Route::HomePage => home_page::home_page(),
        Route::InspektPage => inspekt_showcase_page::inspekt_page(),
        Route::StockTrackerPage => stock_tracker_page::stock_tracker_page(),
        Route::AlulaBiblePage => alula_bible_page::alula_bible_page(),
        Route::AutoIngestPage => auto_ingest_page::auto_ingest_page(),
        Route::RustSite => rust_site::rust_site(),
        Route::ContactPage => contact_page::contact_page(),
        Route::LandingPage => landing_page::landing_page(),
        Route::Info => info_page::info_page(),
        Route::PortfolioSitePage => portfolio_site_page::portfolio_site_page(),
        Route::FiroLoggerPage => firo_logger_page::firo_logger_page(),
        Route::CssToRustConverterPage => css_to_rust_converter_page::css_to_rust_converter_page(),
        Route::GameScoreTrackerPage => game_score_tracker_page::game_score_tracker_page(),
        Route::GitlabTerraformPage => gitlab_terraform_page::gitlab_terraform_page(),
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
