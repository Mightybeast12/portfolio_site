//! Button component styles

use stylist::Style;

pub fn button_base() -> Style {
    Style::new(
        r#"
        border-radius: var(--border-radius-md);
        padding: var(--spacing-sm) var(--spacing-md);
        font-size: var(--font-size-md);
        font-weight: var(--font-weight-medium);
        cursor: pointer;
        transition: all var(--transition-fast);
        min-height: 44px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: var(--spacing-sm);
        border: none;

        &:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }

        &:hover:not(:disabled) {
            transform: translateY(-1px);
            box-shadow: var(--shadow-md);
        }

        &:focus {
            outline: 2px solid var(--color-primary);
            outline-offset: 2px;
        }
    "#,
    )
    .expect("Failed to create base button styles")
}

pub fn button_primary() -> Style {
    Style::new(
        r#"
        background: var(--color-primary);
        color: var(--color-text-on-primary);

        &:hover:not(:disabled) {
            background: var(--color-primary-hover);
        }

        &:active {
            background: var(--color-primary-active);
        }
    "#,
    )
    .expect("Failed to create primary button styles")
}

pub fn button_secondary() -> Style {
    Style::new(
        r#"
        background: var(--color-surface);
        color: var(--color-text-primary);
        border: var(--border-width) solid var(--color-border);

        &:hover:not(:disabled) {
            background: var(--color-surface-hover);
            border-color: var(--color-border-hover);
        }
    "#,
    )
    .expect("Failed to create secondary button styles")
}

pub fn button_ghost() -> Style {
    Style::new(
        r#"
        background: transparent;
        color: var(--color-text-primary);

        &:hover:not(:disabled) {
            background: var(--color-surface-hover);
        }
    "#,
    )
    .expect("Failed to create ghost button styles")
}
