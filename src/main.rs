use yew::prelude::*;
use pulldown_cmark::{Parser, Options, html};

#[function_component(MarkdownViewer)]
fn markdown_viewer() -> Html {
    let markdown_input = r#"
# Hello, Yew!


[Link to Rust](https://www.rust-lang.org/)
    "#;

    // Parse the markdown to HTML
    let mut html_output = String::new();
    let parser = Parser::new_ext(markdown_input, Options::empty());
    html::push_html(&mut html_output, parser);
     

    html! {
        <div dangerously_set_inner_html={html_output} />
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Markdown Viewer Header" }</h1>
            <MarkdownViewer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
