//! Card utilities updated to use modern components

use crate::components::{Card, CodeBlock};
use crate::utils::file_reader::FileContent;
use yew::prelude::*;

pub fn create_code_styled_markdown(title: String, input: String) -> Html {
    html! {
        <CodeBlock
            title={Some(title)}
            code={input}
            show_line_numbers={true}
            max_height={Some("60vh".to_string())}
        />
    }
}

pub fn create_dynamic_styled_markdown(title: String, file: String) -> Html {
    html! {
        <Card class="file-content-card">
            <FileContent file_path={file} title={title} />
        </Card>
    }
}
