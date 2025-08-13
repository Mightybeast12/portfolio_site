//! Card component styles

use stylist::Style;

pub fn card_base() -> Style {
    Style::new(
        r#"
        background: var(--color-surface);
        border: var(--border-width) solid var(--color-border);
        border-radius: var(--border-radius-md);
        padding: var(--spacing-md);
        box-shadow: var(--shadow-md);
        transition: all var(--transition-normal);
    "#,
    )
    .expect("Failed to create base card styles")
}

pub fn card_hoverable() -> Style {
    Style::new(
        r#"
        &:hover {
            background: var(--color-surface-hover);
            border-color: var(--color-border-hover);
            box-shadow: var(--shadow-lg);
            transform: translateY(-2px);
        }
    "#,
    )
    .expect("Failed to create hoverable card styles")
}

pub fn card_elevated() -> Style {
    Style::new(
        r#"
        box-shadow: var(--shadow-lg);

        &:hover {
            box-shadow: var(--shadow-xl);
            transform: translateY(-4px);
        }
    "#,
    )
    .expect("Failed to create elevated card styles")
}

pub fn card_compact() -> Style {
    Style::new(
        r#"
        padding: var(--spacing-sm);
    "#,
    )
    .expect("Failed to create compact card styles")
}
