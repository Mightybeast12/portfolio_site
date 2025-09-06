use crate::shared::ui::cards;
use yew::prelude::*;

pub fn rust_docker_container_showcase() -> Html {
    let title = "Rust Docker".to_string();
    let file = "static/cv_portfolio/Rust_Dockerfile ".to_string();
    cards::create_dynamic_styled_markdown(title, file)
}

pub fn auto_ingest_docker_container_showcase() -> Html {
    let title = "Auto Ingest Dockerfile".to_string();
    let file = "static/auto_ingest/docker_build".to_string();
    cards::create_dynamic_styled_markdown(title, file)
}
