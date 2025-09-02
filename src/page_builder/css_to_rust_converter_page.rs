use yew::prelude::*;

pub fn css_to_rust_converter_page() -> Html {
    html! {
        <div class="home-container">
            <div class="markdown-container">
                {css_to_rust_converter_markdown()}
                <div class="button-20-container">
                    <a href="https://github.com/Mightybeast12/css-to-rust-converter" target="_blank" class="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"VIEW ON GITHUB"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

pub fn css_to_rust_converter_intro() -> Html {
    html! {
        <div>
            <h1>{"CSS-to-Rust Converter"}</h1>
            <p>{"A comprehensive tool for converting CSS styles to Rust stylist format for Yew applications."}</p>
            <ul class="feature-list">
                <li>{"Smart CSS parsing with media queries and pseudo-selectors"}</li>
                <li>{"Automatic value mapping to theme variables"}</li>
                <li>{"Component grouping and variant detection"}</li>
                <li>{"CLI interface with progress bars and analysis tools"}</li>
            </ul>
        </div>
    }
}

pub fn css_to_rust_converter_markdown() -> Html {
    html! {
        <div>
            <h1>{"CSS-to-Rust Converter"}</h1>

            <p>{"A comprehensive tool for converting CSS styles to Rust stylist format for Yew applications."}</p>

            <h2>{"Features"}</h2>
            <ul>
                <li>{"🎯 Smart CSS Parsing - Complex CSS with media queries and pseudo-selectors"}</li>
                <li>{"🔄 Value Mapping - Automatic mapping to theme variables"}</li>
                <li>{"📦 Component Grouping - Organized into Rust modules"}</li>
                <li>{"🎨 Variant Detection - Extracts style variants automatically"}</li>
                <li>{"🛠️ CLI Interface - Rich command-line with progress bars"}</li>
                <li>{"⚙️ Configurable - Custom mappings via JSON"}</li>
                <li>{"📊 Analysis Tools - CSS analysis and validation"}</li>
            </ul>

            <h2>{"Installation & Usage"}</h2>
            <p>{"Install and use the converter with simple commands:"}</p>
            <ul>
                <li>{"pip install -r requirements.txt"}</li>
                <li>{"python -m css_to_rust convert input.css -o output.rs"}</li>
                <li>{"python -m css_to_rust convert input.css -o ./styles/ --component"}</li>
            </ul>

            <h2>{"Technology Stack"}</h2>
            <ul>
                <li>{"Language: Python 3.8+"}</li>
                <li>{"CSS Parsing: Advanced AST parsing"}</li>
                <li>{"Output: Rust stylist format"}</li>
                <li>{"CLI: Rich terminal interface"}</li>
                <li>{"Configuration: JSON-based mappings"}</li>
            </ul>

            <h2>{"Key Benefits"}</h2>
            <ul>
                <li>{"🔄 Automatic CSS to Rust stylist conversion"}</li>
                <li>{"🎨 Theme variable mapping for design systems"}</li>
                <li>{"📦 Component-based organization"}</li>
                <li>{"🛠️ Batch processing capabilities"}</li>
                <li>{"⚙️ Custom design token support"}</li>
            </ul>

            <h2>{"Perfect for Yew Development"}</h2>
            <p>{"Seamlessly integrate converted styles into your Yew components. This tool bridges the gap between traditional CSS development and modern Rust web applications."}</p>

            <h2>{"Framework Support"}</h2>
            <p>{"Automatically detects and converts styles from:"}</p>
            <ul>
                <li>{"Bootstrap components and utilities"}</li>
                <li>{"Tailwind CSS classes"}</li>
                <li>{"Custom CSS frameworks"}</li>
                <li>{"Design system tokens"}</li>
            </ul>
        </div>
    }
}
