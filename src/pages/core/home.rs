use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::projects;
use crate::routing::Route;
use crate::shared::ui::code_showcase;

pub fn home_page() -> Html {
    html! {
        <div class="home-container">
            // New Repository Showcases - At the top

            // Portfolio Site Infrastructure
            <div class="markdown-container">
                {projects::portfolio_site::portfolio_site_intro()}
                <div class="button-20-container">
                    <Link<Route> to={Route::PortfolioSitePage} classes="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"CHECK OUT"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </Link<Route>>
                </div>
            </div>
            // Portfolio Site End

            // Firo Logger
            <div class="markdown-container">
                {projects::firo_logger::firo_logger_intro()}
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
                {projects::css_to_rust_converter::css_to_rust_converter_intro()}
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
                {projects::game_score_tracker::game_score_tracker_intro()}
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
                {projects::gitlab_terraform::gitlab_terraform_intro()}
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
                {projects::rust_site::port_folio_site_markdown()}
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
                {projects::inspekt::inspekt_markdown_intro_markdown()}
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
                {projects::stock_tracker::create_stock_tracker_markdown()}
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
                {projects::auto_ingest::alula_auto_ingest_markdown()}
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
                {projects::alula_bible::create_alula_bible_markdown()}
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

            {code_showcase::rust_docker_container_showcase()}
        </div>
    }
}
