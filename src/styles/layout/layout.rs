//! Layout component styles

use stylist::Style;

pub fn layout_main() -> Style {
    Style::new(
        r#"
        min-height: 100vh;
        background-color: var(--color-background);
        color: var(--color-text-primary);
        transition: background-color var(--transition-normal), color var(--transition-normal);

        main {
            min-height: calc(100vh - 80px); /* Adjust based on navbar height */
            padding: var(--spacing-md);
        }
    "#,
    )
    .expect("Failed to create layout main styles")
}

pub fn layout_container() -> Style {
    Style::new(
        r#"
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 var(--spacing-md);
    "#,
    )
    .expect("Failed to create layout container styles")
}

pub fn layout_section() -> Style {
    Style::new(
        r#"
        margin-bottom: var(--spacing-xl);

        &:last-child {
            margin-bottom: 0;
        }
    "#,
    )
    .expect("Failed to create layout section styles")
}
