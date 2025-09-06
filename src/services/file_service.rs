use crate::shared::ui::cards;
use gloo_net::http::Request;
use yew::prelude::*;

/// Asynchronously fetches file content, returning a String.
/// Returns "content not found" if the file is missing or there's an error.
async fn fetch_file_content(file_path: &str) -> String {
    match Request::get(file_path).send().await {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                text
            } else {
                "content not found".to_string()
            }
        }
        Err(_) => "content not found".to_string(),
    }
}

/// Props for the `FileContent` component
#[derive(Properties, PartialEq)]
pub struct FileContentProps {
    pub file_path: String,
    pub title: String,
}

#[function_component(FileContent)]
pub fn file_content(props: &FileContentProps) -> Html {
    let content = use_state(String::new);
    let file_path = props.file_path.clone();
    let title = props.title.clone();

    {
        let content = content.clone();
        use_effect(move || {
            let content = content.clone();
            let file_path = file_path.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_content = fetch_file_content(&file_path).await;
                content.set(fetched_content);
            });

            || ()
        });
    }

    html! {
        <div>
            { cards::create_code_styled_markdown(title, content.to_string()) }
        </div>
    }
}
