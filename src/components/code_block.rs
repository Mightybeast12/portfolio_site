//! Modern code block component with syntax highlighting and copy functionality

use crate::styles::components::code_block as styles;
use crate::theme::{use_theme, use_theme_colors};
use gloo_utils::window;
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = clipboard)]
    static CLIPBOARD: web_sys::Clipboard;

    #[wasm_bindgen(method, js_name = writeText)]
    fn write_text(this: &web_sys::Clipboard, text: &str) -> js_sys::Promise;
}

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub code: String,
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub show_line_numbers: bool,
    #[prop_or_default]
    pub max_height: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let copy_state = use_state(|| CopyState::Ready);

    // Copy functionality
    let copy_code = {
        let code = props.code.clone();
        let copy_state = copy_state.clone();

        Callback::from(move |_: MouseEvent| {
            let code = code.clone();
            let copy_state = copy_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                copy_state.set(CopyState::Copying);

                match copy_to_clipboard(&code).await {
                    Ok(_) => {
                        copy_state.set(CopyState::Success);
                        // Reset after 2 seconds
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_state.set(CopyState::Ready);
                        })
                        .forget();
                    }
                    Err(_) => {
                        copy_state.set(CopyState::Error);
                        // Reset after 2 seconds
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_state.set(CopyState::Ready);
                        })
                        .forget();
                    }
                }
            });
        })
    };

    // Language detection if not provided
    let detected_language = props
        .language
        .clone()
        .or_else(|| detect_language(&props.code));

    // Process code for line numbers
    let lines: Vec<&str> = props.code.lines().collect();
    let line_count = lines.len();

    html! {
        <div class={classes!(styles::code_block_container(), props.class.clone())}>
            // Header with title, language, and copy button
            <div class={styles::code_block_header()}>
                <div class={styles::code_block_header_left()}>
                    // macOS-style window controls
                    <div class={styles::code_block_window_controls()}>
                        <div class={styles::code_block_control_red()}></div>
                        <div class={styles::code_block_control_yellow()}></div>
                        <div class={styles::code_block_control_green()}></div>
                    </div>

                    // Title and language
                    <div class={styles::code_block_title_section()}>
                        if let Some(title) = &props.title {
                            <span class={styles::code_block_title()}>{title}</span>
                        }
                        if let Some(lang) = &detected_language {
                            <span class={styles::code_block_language_badge()}>{lang.to_uppercase()}</span>
                        }
                    </div>
                </div>

                // Copy button
                <button
                    class={styles::code_block_copy_button()}
                    onclick={copy_code}
                    title="Copy code"
                    aria-label="Copy code to clipboard"
                >
                    {match *copy_state {
                        CopyState::Ready => html! {
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                            </svg>
                        },
                        CopyState::Copying => html! {
                            <div class={styles::code_block_spinner()}></div>
                        },
                        CopyState::Success => html! {
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"/>
                            </svg>
                        },
                        CopyState::Error => html! {
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z"/>
                            </svg>
                        },
                    }}
                </button>
            </div>

            // Code content
            <div class={styles::code_block_content(props.max_height.as_deref())}>
                if props.show_line_numbers {
                    <div class={styles::code_block_line_numbers()}>
                        {for (1..=line_count).map(|i| html! {
                            <div class={styles::code_block_line_number()}>{i}</div>
                        })}
                    </div>
                }

                <pre class={styles::code_block_pre()}>
                    <code class={styles::code_block_code()}>
                        {highlight_code(&props.code, detected_language.as_deref())}
                    </code>
                </pre>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum CopyState {
    Ready,
    Copying,
    Success,
    Error,
}

// Simple syntax highlighting (can be enhanced with a proper syntax highlighter)
fn highlight_code(code: &str, language: Option<&str>) -> Html {
    // For now, return plain text. This can be enhanced with proper syntax highlighting
    html! {
        <span>{code}</span>
    }
}

// Simple language detection based on file extensions or content
fn detect_language(code: &str) -> Option<String> {
    if code.contains("fn ") && code.contains("->") {
        Some("rust".to_string())
    } else if code.contains("FROM ") && code.contains("RUN ") {
        Some("dockerfile".to_string())
    } else if code.contains("function ") || code.contains("const ") || code.contains("=>") {
        Some("javascript".to_string())
    } else if code.contains("def ") && code.contains(":") {
        Some("python".to_string())
    } else if code.contains("#include") || code.contains("int main") {
        Some("c".to_string())
    } else {
        None
    }
}

// Copy to clipboard functionality
async fn copy_to_clipboard(text: &str) -> Result<(), JsValue> {
    let promise = unsafe { CLIPBOARD.write_text(text) };
    wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(())
}
