use yew::prelude::*;
use yew_router::prelude::*;

use crate::page_builder::alula_bible_page;
use crate::page_builder::auto_ingest_page;
use crate::page_builder::code_show_case_elements;
use crate::page_builder::css_to_rust_converter_page;
use crate::page_builder::firo_logger_page;
use crate::page_builder::game_score_tracker_page;
use crate::page_builder::gitlab_terraform_page;
use crate::page_builder::inspekt_showcase_page;
use crate::page_builder::portfolio_site_page;
use crate::page_builder::rust_site;
use crate::page_builder::stock_tracker_page;
use crate::utils::router::Route;

pub fn home_page() -> Html {
    html! {
        <div class="home-container">
            // New Repository Showcases - At the top

            // Portfolio Site Infrastructure
            <div class="markdown-container">
                {portfolio_site_page::portfolio_site_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::PortfolioSitePage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"Route CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Portfolio Site End

            // Firo Logger
            <div class="markdown-container">
                {firo_logger_page::firo_logger_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::FiroLoggerPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Firo Logger End

            // CSS to Rust Converter
            <div class="markdown-container">
                {css_to_rust_converter_page::css_to_rust_converter_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::CssToRustConverterPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // CSS to Rust Converter End

            // Game Score Tracker
            <div class="markdown-container">
                {game_score_tracker_page::game_score_tracker_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::GameScoreTrackerPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Game Score Tracker End

            // GitLab Terraform Infrastructure
            <div class="markdown-container">
                {gitlab_terraform_page::gitlab_terraform_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::GitlabTerraformPage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // GitLab Terraform End

            // Original Projects Below

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
