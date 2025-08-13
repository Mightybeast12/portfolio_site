//! Navbar component styles

use stylist::Style;

pub fn navbar_main() -> Style {
    Style::new(
        r#"
        background: var(--color-surface);
        border-bottom: var(--border-width) solid var(--color-border);
        box-shadow: var(--shadow-sm);
        position: sticky;
        top: 0;
        z-index: 100;
        transition: all var(--transition-normal);
    "#,
    )
    .expect("Failed to create navbar main styles")
}

pub fn navbar_container() -> Style {
    Style::new(
        r#"
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 var(--spacing-md);
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 80px;
    "#,
    )
    .expect("Failed to create navbar container styles")
}

pub fn navbar_brand() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);

        img {
            height: 40px;
            width: auto;
            border-radius: var(--border-radius-sm);
        }
    "#,
    )
    .expect("Failed to create navbar brand styles")
}

pub fn navbar_brand_text() -> Style {
    Style::new(
        r#"
        font-size: var(--font-size-lg);
        font-weight: var(--font-weight-bold);
        color: var(--color-text-primary);
        text-decoration: none;

        &:hover {
            color: var(--color-primary);
        }
    "#,
    )
    .expect("Failed to create navbar brand text styles")
}

pub fn navbar_menu() -> Style {
    Style::new(
        r#"
        display: flex;
        list-style: none;
        margin: 0;
        padding: 0;
        gap: var(--spacing-lg);
        align-items: center;
    "#,
    )
    .expect("Failed to create navbar menu styles")
}

pub fn navbar_item() -> Style {
    Style::new(
        r#"
        a {
            color: var(--color-text-secondary);
            text-decoration: none;
            font-weight: var(--font-weight-medium);
            padding: var(--spacing-sm) var(--spacing-md);
            border-radius: var(--border-radius-md);
            transition: all var(--transition-fast);

            &:hover {
                background: var(--color-surface-hover);
                color: var(--color-text-primary);
            }
        }
    "#,
    )
    .expect("Failed to create navbar item styles")
}

pub fn navbar_actions() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        gap: var(--spacing-md);
    "#,
    )
    .expect("Failed to create navbar actions styles")
}

pub fn navbar_hamburger() -> Style {
    Style::new(
        r#"
        display: none;
        flex-direction: column;
        cursor: pointer;
        padding: var(--spacing-xs);

        span {
            width: 25px;
            height: 3px;
            background: var(--color-text-primary);
            margin: 3px 0;
            transition: var(--transition-fast);
            border-radius: var(--border-radius-sm);
        }

        @media (max-width: 768px) {
            display: flex;
        }
    "#,
    )
    .expect("Failed to create navbar hamburger styles")
}

pub fn navbar_menu_mobile() -> Style {
    Style::new(
        r#"
        @media (max-width: 768px) {
            display: none;
        }
    "#,
    )
    .expect("Failed to create navbar menu mobile styles")
}
