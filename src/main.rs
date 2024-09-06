use yew::prelude::*;
use cv_portfolio_site::*; 
use page_builder::home_page; 
use page_builder::header_builder; 
use utils::file_reader; 
  
#[function_component]
fn App() -> Html {
    html! {
        <div> 
        <h1> {header_builder::build_header()} </h1> 
        {home_page::create_home_page()} 
        {page_builder::code_show_case_page::create_code_markdown("static/Firat Honca/Docker Build")} 
        </div> 
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
