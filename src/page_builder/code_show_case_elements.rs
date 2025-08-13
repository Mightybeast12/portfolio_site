//! Code showcase elements updated to use modern CodeBlock component

use crate::components::CodeBlock;
use yew::prelude::*;

pub fn rust_docker_container_showcase() -> Html {
    let title = "Rust Docker".to_string();
    let file_path = "static/cv_portfolio/Rust_Dockerfile ";

    // For now, we'll use a placeholder. This could be enhanced to read the file content
    let dockerfile_content = r#"# Multi-stage Dockerfile optimized for Rust/WASM builds
FROM rust:slim-bookworm AS builder
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY . .
RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]"#;

    html! {
        <CodeBlock
            title={Some(title)}
            code={dockerfile_content.to_string()}
            language={Some("dockerfile".to_string())}
            show_line_numbers={true}
            max_height={Some("400px".to_string())}
        />
    }
}

pub fn auto_ingest_docker_container_showcase() -> Html {
    let title = "Auto Ingest Dockerfile".to_string();

    // Placeholder content - this could be enhanced to read from the actual file
    let dockerfile_content = r#"FROM python:3.9-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
EXPOSE 8000
CMD ["python", "app.py"]"#;

    html! {
        <CodeBlock
            title={Some(title)}
            code={dockerfile_content.to_string()}
            language={Some("dockerfile".to_string())}
            show_line_numbers={true}
            max_height={Some("300px".to_string())}
        />
    }
}
