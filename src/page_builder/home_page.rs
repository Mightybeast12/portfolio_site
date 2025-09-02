use yew::prelude::*;
use yew_router::prelude::*;

use crate::page_builder::alula_bible_page;
use crate::page_builder::auto_ingest_page;
use crate::page_builder::code_show_case_elements;
use crate::page_builder::inspekt_showcase_page;
use crate::page_builder::rust_site;
use crate::page_builder::stock_tracker_page;
use crate::utils::router::Route;

pub fn home_page() -> Html {
    html! {
        <div class="home-container">
            //Portfolio Page
            <div class="markdown-container">
                {rust_site::port_folio_site_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::RustSite} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Portfolio Page End
            //Inspekt Page
            <div class="markdown-container">
                {inspekt_showcase_page::inspekt_markdown_intro_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::InspektPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Inspekt Page End

            // StockTrackerPage
            <div class="markdown-container">
                {stock_tracker_page::create_stock_tracker_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::StockTrackerPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // StockTrackerPage End

            //AutoIngestPage
            <div class="markdown-container">
                {auto_ingest_page::alula_auto_ingest_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::AutoIngestPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // AutoIngestPage End

            // Alula BiblePage
            <div class="markdown-container">
                {alula_bible_page::create_alula_bible_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::AlulaBiblePage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"Check out"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Alula bible End
            {code_show_case_elements::rust_docker_container_showcase()}
        </div>
    }
}
