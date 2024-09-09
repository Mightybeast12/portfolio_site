use yew::prelude::*;
use crate::utils::card_elements;

pub fn rust_docker_container_showcase() -> Html {
    let title = "Rust Docker".to_string(); 
    let file = "static/Docker Build".to_string();
    card_elements::create_dynamic_styled_markdown(title, file) 
    
}
