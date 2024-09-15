use yew::prelude::*;
use crate::utils::card_elements;

pub fn rust_docker_container_showcase() -> Html {
    let title = "Rust Docker".to_string(); 
    let file = "static/cv_portfolio/Rust_Dockerfile ".to_string();
    card_elements::create_dynamic_styled_markdown(title, file) 
    
}
 
pub fn auto_ingest_docker_container_showcase() -> Html {
    let title = "Auto Ingest Dockerfile".to_string(); 
    let file = "static/auto_ingest/docker_build".to_string();
    card_elements::create_dynamic_styled_markdown(title, file) 
    
}
