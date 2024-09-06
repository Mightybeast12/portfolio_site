use yew::prelude::*;
use crate::utils::{mark_down_utils, file_reader};

pub fn create_code_markdown(file: &str) -> Html {
    
    // Read the file
    let file_content = file_reader::read_file(&file);
    
    // Determine the markdown content based on file reading result
    let markdown = match file_content {
        Ok(content) => content,
        Err(e) => format!("# Content Not Found\nFile: {}\nError:{}", file,e),
    };
    
    html! {
        <div>
            { mark_down_utils::create_markdown(&markdown) }
        </div>
    }
}
