
use yew::prelude::*;
 
use crate::utils::image_utils::image_carousel_builder;
use crate::utils::mark_down_utils;
 
pub fn port_folio_site_markdown() -> Html {
     
let markdown = r#"
# Rust-Based Website

This website is built using **Rust** and the **Yew** framework to showcase my work on various projects. The site is hosted on **Google Cloud Run**

## Key Features:
Built with **Yew's routing** and **web-worker** capabilities.
Packaged in a **Docker container** for easy deployment.
Deployed on **Google Cloud Run** for scalability.
Automatically built and pushed to **Google Cloud Artifact Registry** using a custom **bash script**.

"#;

    mark_down_utils::create_markdown(&markdown) 
 
}

 
fn portfolio_showcase_images() -> Html {
let images = vec![
    "rust_site/cloud_run_home_page.png".to_string(),
    "rust_site/auto_deploy_cloud.png".to_string(),
    "rust_site/button_styles.png".to_string(),
    "rust_site/artificat_docker.png".to_string(),
    "rust_site/creating_elements.png".to_string(),
    "rust_site/routing.png".to_string(),
];

    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}
 
pub fn rust_site() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            {port_folio_site_markdown()} 
            {portfolio_showcase_images()}
        </div>
    }
}
