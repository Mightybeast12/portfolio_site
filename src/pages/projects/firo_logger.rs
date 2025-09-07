use yew::prelude::*;

pub fn firo_logger_page() -> Html {
    html! {
        <div class="home-container">
            <div class="markdown-container">
                {firo_logger_markdown()}
                <div class="button-20-container">
                    <a href="https://github.com/Mightybeast12/firo_logger" target="_blank" class="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"VIEW ON GITHUB"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </a>
                </div>
                <div class="button-20-container">
                    <a href="https://crates.io/crates/firo_logger" target="_blank" class="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"VIEW ON CRATES.IO"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

pub fn firo_logger_intro() -> Html {
    html! {
        <div>
            <h1>{"firo_logger"}</h1>
            <p>{"A high-performance, feature-rich logger for Rust applications with colored output, structured logging, file rotation, and async logging."}</p>
            <ul class="feature-list">
                <li>{"High-performance async logging (1M+ logs/second)"}</li>
                <li>{"Structured JSON logging with metadata support"}</li>
                <li>{"Automatic file rotation and lifecycle management"}</li>
                <li>{"Published on crates.io with comprehensive documentation"}</li>
            </ul>
        </div>
    }
}

pub fn firo_logger_markdown() -> Html {
    html! {
        <div>
            <h1>{"firo_logger"}</h1>

            <p>{"A high-performance, feature-rich logger for Rust applications with colored output, structured logging, file rotation, async logging, and advanced configuration."}</p>

            <h2>{"Features"}</h2>
            <ul>
                <li>{"✨ Colored console output with customizable colors"}</li>
                <li>{"📊 Structured logging with JSON format support"}</li>
                <li>{"📁 File logging with automatic rotation (size-based and time-based)"}</li>
                <li>{"⚡ Async logging for high-performance applications"}</li>
                <li>{"🎯 Level filtering with module-specific filters"}</li>
                <li>{"🔒 Thread-safe with minimal overhead"}</li>
                <li>{"📍 Caller information (file, line, module)"}</li>
                <li>{"🏷️ Custom metadata support"}</li>
                <li>{"🌍 Environment configuration support"}</li>
                <li>{"🏗️ Builder pattern for easy configuration"}</li>
            </ul>

            <h2>{"Quick Start"}</h2>
            <div class="card-code-1">
                <div class="top-card-code-1">
                    <div class="circle-card-code-1 red"></div>
                    <div class="circle-card-code-1 yellow"></div>
                    <div class="circle-card-code-1 green"></div>
                </div>
                <div class="header-card-code-1">
                    <h3 id="card-code-1-title">{"Quick Start Example"}</h3>
                </div>
                <div class="cardcode-1-code-container">
                    <pre id="card-code-1-code-block">{r#"use firo_logger::{init_default, log_info, log_error, log_success};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger with default settings
    init_default()?;

    // Log some messages
    log_info!("Application started");
    log_success!("Configuration loaded successfully");
    log_error!("Failed to connect to database: {}", "Connection timeout");

    Ok(())
}"#}</pre>
                </div>
            </div>

            <h2>{"Advanced Configuration"}</h2>
            <div class="card-code-1">
                <div class="top-card-code-1">
                    <div class="circle-card-code-1 red"></div>
                    <div class="circle-card-code-1 yellow"></div>
                    <div class="circle-card-code-1 green"></div>
                </div>
                <div class="header-card-code-1">
                    <h3 id="card-code-1-title">{"Advanced Configuration"}</h3>
                </div>
                <div class="cardcode-1-code-container">
                    <pre id="card-code-1-code-block">{r#"use firo_logger::{LoggerConfig, LogLevel, OutputFormat, init};

let config = LoggerConfig::builder()
    .level(LogLevel::Debug)
    .format(OutputFormat::Json)
    .console(true)
    .colors(true)
    .file("app.log")
    .rotate_by_size(10 * 1024 * 1024, 5) // 10MB, keep 5 files
    .async_logging(1000)
    .include_caller(true)
    .include_thread(true)
    .metadata("app", "my-app")
    .build();

init(config)?"#}</pre>
                </div>
            </div>

            <h2>{"Performance Highlights"}</h2>
            <ul>
                <li>{"🚀 1M+ logs/second in async mode"}</li>
                <li>{"⚡ Sub-microsecond latency for filtered-out messages"}</li>
                <li>{"💾 Zero allocation for many operations"}</li>
                <li>{"🔧 Lock-free paths optimized for concurrent access"}</li>
            </ul>

            <h2>{"Available on Crates.io"}</h2>
            <p>{"Install with: "}<code>{"cargo add firo_logger"}</code></p>
        </div>
    }
}
