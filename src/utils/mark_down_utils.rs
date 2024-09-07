use pulldown_cmark::{html,Parser};
use yew::prelude::*;
pub fn create_markdown(markdown_input: &str) -> Html{
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}


pub fn create_code_styled_markdown(input: &str) -> Html {
    html! {
        <div style="display: flex; justify-content: center; align-items: center; height: 10%;">
            <div style="
                background-color: color: rgb(42, 57, 141);
                border: 2px solid #ddd; 
                border-radius: 5px; 
                padding: 20px;
                width: 80%;
                max-width: 800px;
                box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
                overflow-x: auto;
            ">
                <pre> {create_markdown(&input)}
                </pre>
            </div>
        </div>
    }
}

 