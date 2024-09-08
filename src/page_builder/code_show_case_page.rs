use yew::prelude::*;
use crate::utils::card_elements;

pub fn rust_docker_container_showcase() -> Html {
    let title = "Rust Docker"; 
    let markdown = r#"
# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.67

#Set Web assembly w
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install --path .

# Run the web service on container startup.
CMD ["trunk serve"]
    "#; 
    
    
    html! {
        <div>
            {card_elements::create_code_styled_markdown(title.to_string(),markdown.to_string())}
        </div>
    }
}
