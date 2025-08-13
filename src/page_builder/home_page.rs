use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Button, ButtonVariant, Card};
use crate::page_builder::alula_bible_page;
use crate::page_builder::auto_ingest_page;
use crate::page_builder::code_show_case_elements;
use crate::page_builder::inspekt_showcase_page;
use crate::page_builder::rust_site;
use crate::page_builder::stock_tracker_page;
use crate::styles::pages::home as styles;
use crate::utils::mark_down_utils;
use crate::utils::router::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class={styles::home_page_container()}>
            <div class={styles::home_page_header()}>
                <h1>{"Portfolio Showcase"}</h1>
                <p>{"Explore my projects and technical implementations"}</p>
            </div>

            // Portfolio Site Project
            <Card class={styles::home_page_project_card()} hoverable={true}>
                {rust_site::port_folio_site_markdown()}
                <div class={styles::home_page_card_actions()}>
                    <Link<Route> to={Route::RustSite}>
                        <Button variant={ButtonVariant::Primary}>
                            {"CHECK OUT"}
                        </Button>
                    </Link<Route>>
                </div>
            </Card>

            // Inspekt Project
            <Card class={styles::home_page_project_card()} hoverable={true}>
                {inspekt_showcase_page::inspekt_markdown_intro_markdown()}
                <div class={styles::home_page_card_actions()}>
                    <Link<Route> to={Route::InspektPage}>
                        <Button variant={ButtonVariant::Primary}>
                            {"CHECK OUT"}
                        </Button>
                    </Link<Route>>
                </div>
            </Card>

            // Stock Tracker Project
            <Card class={styles::home_page_project_card()} hoverable={true}>
                {stock_tracker_page::create_stock_tracker_markdown()}
                <div class={styles::home_page_card_actions()}>
                    <Link<Route> to={Route::StockTrackerPage}>
                        <Button variant={ButtonVariant::Primary}>
                            {"CHECK OUT"}
                        </Button>
                    </Link<Route>>
                </div>
            </Card>

            // Auto Ingest Project
            <Card class={styles::home_page_project_card()} hoverable={true}>
                {auto_ingest_page::alula_auto_ingest_markdown()}
                <div class={styles::home_page_card_actions()}>
                    <Link<Route> to={Route::AutoIngestPage}>
                        <Button variant={ButtonVariant::Primary}>
                            {"CHECK OUT"}
                        </Button>
                    </Link<Route>>
                </div>
            </Card>

            // Alula Bible Project
            <Card class={styles::home_page_project_card()} hoverable={true}>
                {alula_bible_page::create_alula_bible_markdown()}
                <div class={styles::home_page_card_actions()}>
                    <Link<Route> to={Route::AlulaBiblePage}>
                        <Button variant={ButtonVariant::Primary}>
                            {"Check out"}
                        </Button>
                    </Link<Route>>
                </div>
            </Card>

            // Code Showcase
            <div class={styles::home_page_project_card()}>
                {code_show_case_elements::rust_docker_container_showcase()}
            </div>
        </div>
    }
}

// Keep the function for backward compatibility
pub fn home_page() -> Html {
    html! { <HomePage /> }
}
