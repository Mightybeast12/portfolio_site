//! Loading component styles

use stylist::Style;

pub fn loading_base() -> Style {
    Style::new(
        r#"
        display: inline-block;
        border-radius: 50%;
        animation: spin 1s linear infinite;

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    "#,
    )
    .expect("Failed to create base loading styles")
}

pub fn loading_small() -> Style {
    Style::new(
        r#"
        width: 16px;
        height: 16px;
        border: 2px solid var(--color-border);
        border-top: 2px solid var(--color-primary);
    "#,
    )
    .expect("Failed to create small loading styles")
}

pub fn loading_medium() -> Style {
    Style::new(
        r#"
        width: 24px;
        height: 24px;
        border: 3px solid var(--color-border);
        border-top: 3px solid var(--color-primary);
    "#,
    )
    .expect("Failed to create medium loading styles")
}

pub fn loading_large() -> Style {
    Style::new(
        r#"
        width: 32px;
        height: 32px;
        border: 4px solid var(--color-border);
        border-top: 4px solid var(--color-primary);
    "#,
    )
    .expect("Failed to create large loading styles")
}
