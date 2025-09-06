use pulldown_cmark::{html,Parser};
use yew::prelude::*;
pub fn create_markdown(markdown_input: &str) -> Html{
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}


pub fn create_code_styled_markdown(input:String) -> Html {
    html! {
        <div class="card-code-1">
            <div class="top">
                <div class="circle red"></div>
                <div class="circle yellow"></div>
                <div class="circle green"></div>
            </div>
            <div class="header">
                <h2 id="title2">{"Code Block"}</h2>
            </div>
            <div class="code-container">
                <pre id="code">{input}</pre>
            </div>
        </div>
    }
}

 