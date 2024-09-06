use yew::prelude::*;
use crate::utils::{mark_down_utils, file_reader};

pub fn create_code_markdown(file: &str) -> Html {
    let markdown = r#"
```docker     
# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.67

#Set Web assembly
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
            { mark_down_utils::create_code_styled_markdown(&markdown) }
        </div>
    }
}
