//! Code display card components

use crate::constants::ui::*;
use crate::utils::file_reader::FileContent;
use yew::prelude::*;

/// Props for the code card component
#[derive(Properties, PartialEq)]
pub struct CodeCardProps {
    pub title: String,
    pub content: String,
    #[prop_or(CODE_CARD_1_CLASS.to_string())]
    pub class: String,
}

/// A styled code card component with terminal-like appearance
#[function_component(CodeCard)]
pub fn code_card(props: &CodeCardProps) -> Html {
    html! {
        <div>
            <div class={props.class.clone()}>
                <div class="top-card-code-1">
                    <div class="circle-card-code-1 red"></div>
                    <div class="circle-card-code-1 yellow"></div>
                    <div class="circle-card-code-1 green"></div>
                </div>
                <div class="header-card-code-1">
                    <h2 id="card-code-1-title">{&props.title}</h2>
                </div>
                <div class="cardcode-1-code-container">
                    <pre id="card-code-1-code-block">{&props.content}</pre>
                </div>
            </div>
        </div>
    }
}

/// Props for the dynamic file content card
#[derive(Properties, PartialEq)]
pub struct DynamicCodeCardProps {
    pub title: String,
    pub file_path: String,
    #[prop_or(CARD_PADDING.to_string())]
    pub padding: String,
}

/// A card component that dynamically loads and displays file content
#[function_component(DynamicCodeCard)]
pub fn dynamic_code_card(props: &DynamicCodeCardProps) -> Html {
    let style = format!("padding: {};", props.padding);

    html! {
        <div style={style}>
            <FileContent file_path={props.file_path.clone()} title={props.title.clone()} />
        </div>
    }
}

// Legacy function wrappers for backward compatibility
// These will be deprecated once all usage is migrated to components

/// Legacy wrapper for create_code_styled_markdown function
pub fn create_code_styled_markdown(title: String, input: String) -> Html {
    html! {
        <CodeCard title={title} content={input} />
    }
}

/// Legacy wrapper for create_dynamic_styled_markdown function
pub fn create_dynamic_styled_markdown(title: String, file: String) -> Html {
    html! {
        <DynamicCodeCard title={title} file_path={file} />
    }
}
