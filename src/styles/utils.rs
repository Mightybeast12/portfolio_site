//! Style utilities and helper functions

use stylist::Style;

/// Flex utilities
pub fn flex_center() -> Style {
    Style::new(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
    "#,
    )
    .expect("Failed to create flex center styles")
}

pub fn flex_column() -> Style {
    Style::new(
        r#"
        display: flex;
        flex-direction: column;
    "#,
    )
    .expect("Failed to create flex column styles")
}

pub fn flex_row() -> Style {
    Style::new(
        r#"
        display: flex;
        flex-direction: row;
    "#,
    )
    .expect("Failed to create flex row styles")
}

/// Spacing utilities
pub fn margin_auto() -> Style {
    Style::new("margin: 0 auto;").expect("Failed to create margin auto styles")
}

pub fn padding_none() -> Style {
    Style::new("padding: 0;").expect("Failed to create padding none styles")
}

/// Text utilities
pub fn text_center() -> Style {
    Style::new("text-align: center;").expect("Failed to create text center styles")
}

pub fn text_left() -> Style {
    Style::new("text-align: left;").expect("Failed to create text left styles")
}

pub fn text_right() -> Style {
    Style::new("text-align: right;").expect("Failed to create text right styles")
}

/// Visibility utilities
pub fn hidden() -> Style {
    Style::new("display: none;").expect("Failed to create hidden styles")
}

pub fn visible() -> Style {
    Style::new("display: block;").expect("Failed to create visible styles")
}

/// Responsive utilities
pub fn mobile_hidden() -> Style {
    Style::new(
        r#"
        @media (max-width: 768px) {
            display: none;
        }
    "#,
    )
    .expect("Failed to create mobile hidden styles")
}

pub fn desktop_hidden() -> Style {
    Style::new(
        r#"
        @media (min-width: 769px) {
            display: none;
        }
    "#,
    )
    .expect("Failed to create desktop hidden styles")
}
