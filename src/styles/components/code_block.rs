//! Code block component styles

use crate::theme::Theme;
use stylist::Style;

pub fn code_block_container() -> Style {
    Style::new(
        r#"
        background: var(--color-surface);
        border: var(--border-width) solid var(--color-border);
        border-radius: var(--border-radius-lg);
        overflow: hidden;
        box-shadow: var(--shadow-lg);
        transition: all var(--transition-normal);
        margin: var(--spacing-md) 0;

        &:hover {
            box-shadow: var(--shadow-xl);
        }
    "#,
    )
    .expect("Failed to create code block container styles")
}

pub fn code_block_header() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: var(--spacing-sm) var(--spacing-md);
        background: var(--color-code-background);
        border-bottom: var(--border-width) solid var(--color-border);
        min-height: 48px;
    "#,
    )
    .expect("Failed to create code block header styles")
}

pub fn code_block_header_left() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        gap: var(--spacing-md);
    "#,
    )
    .expect("Failed to create code block header left styles")
}

pub fn code_block_window_controls() -> Style {
    Style::new(
        r#"
        display: flex;
        gap: var(--spacing-xs);
    "#,
    )
    .expect("Failed to create code block window controls styles")
}

pub fn code_block_control_red() -> Style {
    Style::new(
        r#"
        width: 12px;
        height: 12px;
        border-radius: 50%;
        background: var(--color-error);
    "#,
    )
    .expect("Failed to create code block control red styles")
}

pub fn code_block_control_yellow() -> Style {
    Style::new(
        r#"
        width: 12px;
        height: 12px;
        border-radius: 50%;
        background: var(--color-warning);
    "#,
    )
    .expect("Failed to create code block control yellow styles")
}

pub fn code_block_control_green() -> Style {
    Style::new(
        r#"
        width: 12px;
        height: 12px;
        border-radius: 50%;
        background: var(--color-success);
    "#,
    )
    .expect("Failed to create code block control green styles")
}

pub fn code_block_title_section() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    "#,
    )
    .expect("Failed to create code block title section styles")
}

pub fn code_block_title() -> Style {
    Style::new(
        r#"
        color: var(--color-text-primary);
        font-weight: var(--font-weight-medium);
        font-size: var(--font-size-sm);
    "#,
    )
    .expect("Failed to create code block title styles")
}

pub fn code_block_language_badge() -> Style {
    Style::new(
        r#"
        background: var(--color-primary);
        color: var(--color-text-on-primary);
        padding: 2px var(--spacing-xs);
        border-radius: var(--border-radius-sm);
        font-size: var(--font-size-xs);
        font-weight: var(--font-weight-medium);
    "#,
    )
    .expect("Failed to create code block language badge styles")
}

pub fn code_block_copy_button() -> Style {
    Style::new(
        r#"
        background: transparent;
        border: var(--border-width) solid var(--color-border);
        border-radius: var(--border-radius-sm);
        color: var(--color-text-muted);
        cursor: pointer;
        padding: var(--spacing-xs);
        transition: all var(--transition-fast);
        display: flex;
        align-items: center;
        justify-content: center;
        min-width: 32px;
        min-height: 32px;

        &:hover {
            background: var(--color-surface-hover);
            border-color: var(--color-border-hover);
            color: var(--color-text-primary);
        }

        &:focus {
            outline: 2px solid var(--color-primary);
            outline-offset: 2px;
        }
    "#,
    )
    .expect("Failed to create code block copy button styles")
}

pub fn code_block_content(max_height: Option<&str>) -> Style {
    let max_height_rule = max_height
        .map(|h| format!("max-height: {};", h))
        .unwrap_or_default();

    Style::new(format!(
        r#"
        display: flex;
        background: var(--color-code-background);
        {}
        overflow: auto;
    "#,
        max_height_rule
    ))
    .expect("Failed to create code block content styles")
}

pub fn code_block_line_numbers() -> Style {
    Style::new(
        r#"
        background: var(--color-surface);
        border-right: var(--border-width) solid var(--color-border);
        padding: var(--spacing-md) var(--spacing-sm);
        user-select: none;
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        min-width: 40px;
    "#,
    )
    .expect("Failed to create code block line numbers styles")
}

pub fn code_block_line_number() -> Style {
    Style::new(
        r#"
        color: var(--color-text-muted);
        font-family: var(--font-mono);
        font-size: var(--font-size-sm);
        line-height: 1.5;
    "#,
    )
    .expect("Failed to create code block line number styles")
}

pub fn code_block_pre() -> Style {
    Style::new(
        r#"
        margin: 0;
        padding: var(--spacing-md);
        background: transparent;
        overflow: auto;
        flex: 1;
        scrollbar-width: thin;
        scrollbar-color: var(--color-border) transparent;

        &::-webkit-scrollbar {
            width: 8px;
            height: 8px;
        }

        &::-webkit-scrollbar-track {
            background: transparent;
        }

        &::-webkit-scrollbar-thumb {
            background: var(--color-border);
            border-radius: 4px;
        }

        &::-webkit-scrollbar-thumb:hover {
            background: var(--color-border-hover);
        }
    "#,
    )
    .expect("Failed to create code block pre styles")
}

pub fn code_block_code() -> Style {
    Style::new(
        r#"
        font-family: var(--font-mono);
        font-size: var(--font-size-sm);
        line-height: 1.5;
        color: var(--color-code-text);
        background: transparent;
    "#,
    )
    .expect("Failed to create code block code styles")
}

pub fn code_block_spinner() -> Style {
    Style::new(
        r#"
        width: 16px;
        height: 16px;
        border: 2px solid var(--color-border);
        border-radius: 50%;
        border-top: 2px solid var(--color-primary);
        animation: spin 1s linear infinite;

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    "#,
    )
    .expect("Failed to create code block spinner styles")
}
