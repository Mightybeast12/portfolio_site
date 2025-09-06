use cv_portfolio_site::routing::RouterComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // Add some lint flaws for testing
    let _unused_variable = String::from("test");
    let mut _mutable_but_never_changed = 42;
    let _redundant_clone = "hello".to_string().clone();

    // Unnecessary return statement
    return html! {
        <RouterComponent/>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
