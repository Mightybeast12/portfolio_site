use yew::prelude::*;
use yew_router::prelude::*; 
use cv_portfolio_site::*;
use page_builder::home_page::home_page;
 
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            { home_page() }  
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
