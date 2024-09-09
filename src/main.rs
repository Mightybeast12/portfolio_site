use yew::prelude::*;
use cv_portfolio_site::*;
use crate::utils::router::RouterComponent; 
#[function_component(App)]
fn app() -> Html {
    html! {
        <RouterComponent/>  
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
