//! Theme type definitions and structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Dark => write!(f, "dark"),
            ThemeMode::Light => write!(f, "light"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
    pub shadows: ThemeShadows,
    pub borders: ThemeBorders,
    pub transitions: ThemeTransitions,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Primary colors
    pub primary: &'static str,
    pub primary_hover: &'static str,
    pub primary_active: &'static str,

    // Background colors
    pub background: &'static str,
    pub surface: &'static str,
    pub surface_hover: &'static str,
    pub overlay: &'static str,

    // Text colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub text_on_primary: &'static str,

    // Code colors
    pub code_background: &'static str,
    pub code_text: &'static str,
    pub code_comment: &'static str,
    pub code_keyword: &'static str,
    pub code_string: &'static str,
    pub code_number: &'static str,
    pub code_function: &'static str,

    // Status colors
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub info: &'static str,

    // Border colors
    pub border: &'static str,
    pub border_hover: &'static str,
}

#[derive(Debug, Clone)]
pub struct ThemeSpacing {
    pub xs: &'static str,  // 4px
    pub sm: &'static str,  // 8px
    pub md: &'static str,  // 16px
    pub lg: &'static str,  // 24px
    pub xl: &'static str,  // 32px
    pub xxl: &'static str, // 48px
}

#[derive(Debug, Clone)]
pub struct ThemeTypography {
    pub font_family: &'static str,
    pub font_mono: &'static str,
    pub font_size_xs: &'static str,
    pub font_size_sm: &'static str,
    pub font_size_md: &'static str,
    pub font_size_lg: &'static str,
    pub font_size_xl: &'static str,
    pub font_size_xxl: &'static str,
    pub line_height: &'static str,
    pub font_weight_normal: &'static str,
    pub font_weight_medium: &'static str,
    pub font_weight_bold: &'static str,
}

#[derive(Debug, Clone)]
pub struct ThemeShadows {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

#[derive(Debug, Clone)]
pub struct ThemeBorders {
    pub radius_sm: &'static str,
    pub radius_md: &'static str,
    pub radius_lg: &'static str,
    pub width: &'static str,
}

#[derive(Debug, Clone)]
pub struct ThemeTransitions {
    pub fast: &'static str,
    pub normal: &'static str,
    pub slow: &'static str,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                // Primary colors
                primary: "#007bff",
                primary_hover: "#0056b3",
                primary_active: "#004085",

                // Background colors
                background: "hsl(0, 2%, 12%)",
                surface: "rgb(43, 43, 42)",
                surface_hover: "rgb(53, 53, 52)",
                overlay: "rgba(0, 0, 0, 0.5)",

                // Text colors
                text_primary: "whitesmoke",
                text_secondary: "antiquewhite",
                text_muted: "hsl(33, 17%, 79%)",
                text_on_primary: "white",

                // Code colors
                code_background: "hsl(0, 0%, 18%)",
                code_text: "hsl(33, 17%, 79%)",
                code_comment: "#6a9955",
                code_keyword: "#569cd6",
                code_string: "#ce9178",
                code_number: "#b5cea8",
                code_function: "#dcdcaa",

                // Status colors
                success: "#00ca4e",
                warning: "#ffbd44",
                error: "#ff605c",
                info: "#007bff",

                // Border colors
                border: "#3b3b3b",
                border_hover: "#4b4b4b",
            },
            spacing: ThemeSpacing {
                xs: "4px",
                sm: "8px",
                md: "16px",
                lg: "24px",
                xl: "32px",
                xxl: "48px",
            },
            typography: ThemeTypography {
                font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
                font_mono: "'Fira Code', 'JetBrains Mono', 'Source Code Pro', monospace",
                font_size_xs: "12px",
                font_size_sm: "14px",
                font_size_md: "16px",
                font_size_lg: "18px",
                font_size_xl: "24px",
                font_size_xxl: "32px",
                line_height: "1.5",
                font_weight_normal: "400",
                font_weight_medium: "500",
                font_weight_bold: "700",
            },
            shadows: ThemeShadows {
                sm: "0 1px 2px rgba(0, 0, 0, 0.05)",
                md: "0 4px 6px rgba(0, 0, 0, 0.1)",
                lg: "0px 10px 10px hsl(250, 10%, 12%)",
                xl: "0 20px 25px rgba(0, 0, 0, 0.15)",
            },
            borders: ThemeBorders {
                radius_sm: "4px",
                radius_md: "8px",
                radius_lg: "12px",
                width: "1px",
            },
            transitions: ThemeTransitions {
                fast: "0.15s ease",
                normal: "0.3s ease",
                slow: "0.5s ease",
            },
        }
    }

    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                // Primary colors
                primary: "#007bff",
                primary_hover: "#0056b3",
                primary_active: "#004085",

                // Background colors
                background: "#ffffff",
                surface: "#f8f9fa",
                surface_hover: "#e9ecef",
                overlay: "rgba(0, 0, 0, 0.1)",

                // Text colors
                text_primary: "#212529",
                text_secondary: "#495057",
                text_muted: "#6c757d",
                text_on_primary: "white",

                // Code colors
                code_background: "#f8f9fa",
                code_text: "#495057",
                code_comment: "#6a737d",
                code_keyword: "#d73a49",
                code_string: "#032f62",
                code_number: "#005cc5",
                code_function: "#6f42c1",

                // Status colors
                success: "#28a745",
                warning: "#ffc107",
                error: "#dc3545",
                info: "#17a2b8",

                // Border colors
                border: "#dee2e6",
                border_hover: "#adb5bd",
            },
            spacing: ThemeSpacing {
                xs: "4px",
                sm: "8px",
                md: "16px",
                lg: "24px",
                xl: "32px",
                xxl: "48px",
            },
            typography: ThemeTypography {
                font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
                font_mono: "'Fira Code', 'JetBrains Mono', 'Source Code Pro', monospace",
                font_size_xs: "12px",
                font_size_sm: "14px",
                font_size_md: "16px",
                font_size_lg: "18px",
                font_size_xl: "24px",
                font_size_xxl: "32px",
                line_height: "1.5",
                font_weight_normal: "400",
                font_weight_medium: "500",
                font_weight_bold: "700",
            },
            shadows: ThemeShadows {
                sm: "0 1px 2px rgba(0, 0, 0, 0.05)",
                md: "0 4px 6px rgba(0, 0, 0, 0.1)",
                lg: "0 10px 15px rgba(0, 0, 0, 0.1)",
                xl: "0 20px 25px rgba(0, 0, 0, 0.15)",
            },
            borders: ThemeBorders {
                radius_sm: "4px",
                radius_md: "8px",
                radius_lg: "12px",
                width: "1px",
            },
            transitions: ThemeTransitions {
                fast: "0.15s ease",
                normal: "0.3s ease",
                slow: "0.5s ease",
            },
        }
    }
}
