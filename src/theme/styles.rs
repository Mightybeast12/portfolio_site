//! Global styles and CSS generation for the theme system

use crate::theme::Theme;
use stylist::{yew::styled_component, Style};

pub fn generate_global_styles(theme: &Theme) -> Style {
    Style::new(format!(
        r#"
        :root {{
            /* Theme Colors */
            --color-primary: {primary};
            --color-primary-hover: {primary_hover};
            --color-primary-active: {primary_active};

            --color-background: {background};
            --color-surface: {surface};
            --color-surface-hover: {surface_hover};
            --color-overlay: {overlay};

            --color-text-primary: {text_primary};
            --color-text-secondary: {text_secondary};
            --color-text-muted: {text_muted};
            --color-text-on-primary: {text_on_primary};

            --color-code-background: {code_background};
            --color-code-text: {code_text};
            --color-code-comment: {code_comment};
            --color-code-keyword: {code_keyword};
            --color-code-string: {code_string};
            --color-code-number: {code_number};
            --color-code-function: {code_function};

            --color-success: {success};
            --color-warning: {warning};
            --color-error: {error};
            --color-info: {info};

            --color-border: {border};
            --color-border-hover: {border_hover};

            /* Spacing */
            --spacing-xs: {spacing_xs};
            --spacing-sm: {spacing_sm};
            --spacing-md: {spacing_md};
            --spacing-lg: {spacing_lg};
            --spacing-xl: {spacing_xl};
            --spacing-xxl: {spacing_xxl};

            /* Typography */
            --font-family: {font_family};
            --font-mono: {font_mono};
            --font-size-xs: {font_size_xs};
            --font-size-sm: {font_size_sm};
            --font-size-md: {font_size_md};
            --font-size-lg: {font_size_lg};
            --font-size-xl: {font_size_xl};
            --font-size-xxl: {font_size_xxl};
            --line-height: {line_height};
            --font-weight-normal: {font_weight_normal};
            --font-weight-medium: {font_weight_medium};
            --font-weight-bold: {font_weight_bold};

            /* Shadows */
            --shadow-sm: {shadow_sm};
            --shadow-md: {shadow_md};
            --shadow-lg: {shadow_lg};
            --shadow-xl: {shadow_xl};

            /* Borders */
            --border-radius-sm: {border_radius_sm};
            --border-radius-md: {border_radius_md};
            --border-radius-lg: {border_radius_lg};
            --border-width: {border_width};

            /* Transitions */
            --transition-fast: {transition_fast};
            --transition-normal: {transition_normal};
            --transition-slow: {transition_slow};
        }}

        * {{
            box-sizing: border-box;
        }}

        html {{
            font-family: var(--font-family);
            line-height: var(--line-height);
            background-color: var(--color-background);
            color: var(--color-text-primary);
            transition: background-color var(--transition-normal), color var(--transition-normal);
        }}

        body {{
            margin: 0;
            padding: 0;
            font-size: var(--font-size-md);
            font-weight: var(--font-weight-normal);
        }}

        h1, h2, h3, h4, h5, h6 {{
            color: var(--color-text-secondary);
            font-weight: var(--font-weight-bold);
            margin: var(--spacing-lg) 0 var(--spacing-md) 0;
            line-height: 1.2;
        }}

        h1 {{ font-size: var(--font-size-xxl); }}
        h2 {{ font-size: var(--font-size-xl); }}
        h3 {{ font-size: var(--font-size-lg); }}

        p {{
            margin: 0 0 var(--spacing-md) 0;
            color: var(--color-text-primary);
        }}

        a {{
            color: var(--color-primary);
            text-decoration: none;
            transition: color var(--transition-fast);
        }}

        a:hover {{
            color: var(--color-primary-hover);
        }}

        code {{
            font-family: var(--font-mono);
            font-size: var(--font-size-sm);
            background-color: var(--color-code-background);
            color: var(--color-code-text);
            padding: 2px 4px;
            border-radius: var(--border-radius-sm);
        }}

        pre {{
            font-family: var(--font-mono);
            background-color: var(--color-code-background);
            color: var(--color-code-text);
            padding: var(--spacing-md);
            border-radius: var(--border-radius-md);
            overflow-x: auto;
            margin: var(--spacing-md) 0;
        }}

        pre code {{
            background: none;
            padding: 0;
        }}

        .theme-toggle {{
            background: var(--color-surface);
            border: var(--border-width) solid var(--color-border);
            border-radius: var(--border-radius-md);
            color: var(--color-text-primary);
            cursor: pointer;
            padding: var(--spacing-sm);
            transition: all var(--transition-fast);
            font-size: var(--font-size-lg);
            min-width: 44px;
            min-height: 44px;
            display: flex;
            align-items: center;
            justify-content: center;
        }}

        .theme-toggle:hover {{
            background: var(--color-surface-hover);
            border-color: var(--color-border-hover);
            transform: translateY(-1px);
        }}

        .theme-toggle:focus {{
            outline: 2px solid var(--color-primary);
            outline-offset: 2px;
        }}

        .theme-toggle-icon {{
            display: block;
            transition: transform var(--transition-fast);
        }}

        .theme-toggle:hover .theme-toggle-icon {{
            transform: scale(1.1);
        }}
        "#,
        // Colors
        primary = theme.colors.primary,
        primary_hover = theme.colors.primary_hover,
        primary_active = theme.colors.primary_active,
        background = theme.colors.background,
        surface = theme.colors.surface,
        surface_hover = theme.colors.surface_hover,
        overlay = theme.colors.overlay,
        text_primary = theme.colors.text_primary,
        text_secondary = theme.colors.text_secondary,
        text_muted = theme.colors.text_muted,
        text_on_primary = theme.colors.text_on_primary,
        code_background = theme.colors.code_background,
        code_text = theme.colors.code_text,
        code_comment = theme.colors.code_comment,
        code_keyword = theme.colors.code_keyword,
        code_string = theme.colors.code_string,
        code_number = theme.colors.code_number,
        code_function = theme.colors.code_function,
        success = theme.colors.success,
        warning = theme.colors.warning,
        error = theme.colors.error,
        info = theme.colors.info,
        border = theme.colors.border,
        border_hover = theme.colors.border_hover,
        // Spacing
        spacing_xs = theme.spacing.xs,
        spacing_sm = theme.spacing.sm,
        spacing_md = theme.spacing.md,
        spacing_lg = theme.spacing.lg,
        spacing_xl = theme.spacing.xl,
        spacing_xxl = theme.spacing.xxl,
        // Typography
        font_family = theme.typography.font_family,
        font_mono = theme.typography.font_mono,
        font_size_xs = theme.typography.font_size_xs,
        font_size_sm = theme.typography.font_size_sm,
        font_size_md = theme.typography.font_size_md,
        font_size_lg = theme.typography.font_size_lg,
        font_size_xl = theme.typography.font_size_xl,
        font_size_xxl = theme.typography.font_size_xxl,
        line_height = theme.typography.line_height,
        font_weight_normal = theme.typography.font_weight_normal,
        font_weight_medium = theme.typography.font_weight_medium,
        font_weight_bold = theme.typography.font_weight_bold,
        // Shadows
        shadow_sm = theme.shadows.sm,
        shadow_md = theme.shadows.md,
        shadow_lg = theme.shadows.lg,
        shadow_xl = theme.shadows.xl,
        // Borders
        border_radius_sm = theme.borders.radius_sm,
        border_radius_md = theme.borders.radius_md,
        border_radius_lg = theme.borders.radius_lg,
        border_width = theme.borders.width,
        // Transitions
        transition_fast = theme.transitions.fast,
        transition_normal = theme.transitions.normal,
        transition_slow = theme.transitions.slow,
    ))
    .expect("Failed to create global styles")
}

// Component styles
pub mod component_styles {
    use stylist::Style;

    pub fn card() -> Style {
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
        .expect("Failed to create card styles")
    }

    pub fn card_hover() -> Style {
        Style::new(
            r#"
            background: var(--color-surface-hover);
            border-color: var(--color-border-hover);
            box-shadow: var(--shadow-lg);
            transform: translateY(-2px);
        "#,
        )
        .expect("Failed to create card hover styles")
    }

    pub fn button_primary() -> Style {
        Style::new(
            r#"
            background: var(--color-primary);
            color: var(--color-text-on-primary);
            border: none;
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
        "#,
        )
        .expect("Failed to create button primary styles")
    }

    pub fn button_primary_hover() -> Style {
        Style::new(
            r#"
            background: var(--color-primary-hover);
            transform: translateY(-1px);
            box-shadow: var(--shadow-md);
        "#,
        )
        .expect("Failed to create button primary hover styles")
    }

    pub fn loading_spinner() -> Style {
        Style::new(
            r#"
            display: inline-block;
            width: 20px;
            height: 20px;
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
        .expect("Failed to create loading spinner styles")
    }
}
