use cv_portfolio_site::theme::{generate_global_styles, use_theme_colors, ThemeProvider};
use cv_portfolio_site::utils::router::RouterComponent;
use stylist::yew::Global;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme_colors();

    html! {
        <ThemeProvider>
            if let Some(theme) = theme {
                <Global css={generate_global_styles(&theme)} />
            }
            <RouterComponent/>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
