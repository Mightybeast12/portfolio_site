//! Theme provider component with global state management

use crate::theme::{Theme, ThemeMode};
use gloo_storage::{LocalStorage, Storage};
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::prelude::*;

const THEME_STORAGE_KEY: &str = "portfolio_theme_mode";

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub toggle_theme: Callback<()>,
}

impl ThemeContext {
    pub fn new(theme: Theme, toggle_theme: Callback<()>) -> Self {
        Self {
            theme,
            toggle_theme,
        }
    }
}

pub type ThemeContextType = UseStateHandle<Option<Rc<ThemeContext>>>;

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    // Load theme from localStorage or default to dark
    let initial_mode = LocalStorage::get(THEME_STORAGE_KEY).unwrap_or(ThemeMode::Dark);

    let theme_mode = use_state(|| initial_mode);

    // Create theme based on current mode
    let theme = match *theme_mode {
        ThemeMode::Dark => Theme::dark(),
        ThemeMode::Light => Theme::light(),
    };

    // Toggle callback
    let toggle_theme = {
        let theme_mode = theme_mode.clone();
        Callback::from(move |_| {
            let new_mode = match *theme_mode {
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Light => ThemeMode::Dark,
            };

            // Save to localStorage
            let _ = LocalStorage::set(THEME_STORAGE_KEY, &new_mode);

            // Update state
            theme_mode.set(new_mode);
        })
    };

    // Create context value
    let context_value = use_state(|| {
        Some(Rc::new(ThemeContext::new(
            theme.clone(),
            toggle_theme.clone(),
        )))
    });

    // Update context when theme changes
    use_effect_with_deps(
        {
            let context_value = context_value.clone();
            move |theme: &Theme| {
                context_value.set(Some(Rc::new(ThemeContext::new(
                    theme.clone(),
                    toggle_theme.clone(),
                ))));
                || ()
            }
        },
        theme.clone(),
    );

    html! {
        <ContextProvider<ThemeContextType> context={context_value}>
            <div class="theme-root" data-theme={theme.mode.to_string()}>
                {for props.children.iter()}
            </div>
        </ContextProvider<ThemeContextType>>
    }
}

#[hook]
pub fn use_theme() -> Option<Rc<ThemeContext>> {
    let context = use_context::<ThemeContextType>()?;
    (*context).clone()
}

// Convenience hook for getting just the theme
#[hook]
pub fn use_theme_colors() -> Option<Theme> {
    use_theme().map(|ctx| ctx.theme.clone())
}

// Theme toggle button component
#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let theme_ctx = use_theme();

    let theme_ctx = match theme_ctx {
        Some(ctx) => ctx,
        None => return html! {<div>{"Theme not available"}</div>},
    };

    let onclick = {
        let toggle = theme_ctx.toggle_theme.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    let icon = match theme_ctx.theme.mode {
        ThemeMode::Dark => "🌙",
        ThemeMode::Light => "☀️",
    };

    let title = match theme_ctx.theme.mode {
        ThemeMode::Dark => "Switch to light mode",
        ThemeMode::Light => "Switch to dark mode",
    };

    html! {
        <button
            class={classes!("theme-toggle", props.class.clone())}
            onclick={onclick}
            title={title}
            aria-label={title}
        >
            <span class="theme-toggle-icon">{icon}</span>
        </button>
    }
}
