//! Home page styles

use stylist::Style;

pub fn home_page_container() -> Style {
    Style::new(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: var(--spacing-xl);
        padding: var(--spacing-lg);
        max-width: 1200px;
        margin: 0 auto;
    "#,
    )
    .expect("Failed to create home page container styles")
}

pub fn home_page_header() -> Style {
    Style::new(
        r#"
        text-align: center;
        margin-bottom: var(--spacing-xl);

        h1 {
            font-size: var(--font-size-xxl);
            font-weight: var(--font-weight-bold);
            color: var(--color-text-primary);
            margin-bottom: var(--spacing-md);
        }

        p {
            font-size: var(--font-size-lg);
            color: var(--color-text-secondary);
            max-width: 600px;
            margin: 0 auto;
        }
    "#,
    )
    .expect("Failed to create home page header styles")
}

pub fn home_page_project_card() -> Style {
    Style::new(
        r#"
        width: 100%;
        max-width: 900px;
    "#,
    )
    .expect("Failed to create home page project card styles")
}

pub fn home_page_card_actions() -> Style {
    Style::new(
        r#"
        display: flex;
        justify-content: center;
        margin-top: var(--spacing-md);
        gap: var(--spacing-sm);

        @media (max-width: 768px) {
            flex-direction: column;
            align-items: center;
        }
    "#,
    )
    .expect("Failed to create home page card actions styles")
}
