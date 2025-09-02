use cv_portfolio_site::utils::router::RouterComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <RouterComponent/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
