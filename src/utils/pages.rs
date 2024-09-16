use yew::prelude::*;
 
use crate::page_builder::*;
 
#[function_component(InspektPage)]
pub fn inspekt_route() -> Html {
    inspekt_showcase_page::inspekt_page()
}
 
#[function_component(ContactPage)]
pub fn contact_route() -> Html {
    contact_page::contact_page()
}
 
#[function_component(StockTrackerPage)]
pub fn stock_tracker_route() -> Html {
    stock_tracker_page::stock_tracker_page() 
}
 
#[function_component(AlulaBiblePage)]
pub fn alula_bible_route() -> Html {
    alula_bible_page::alula_bible_page() 
}
 
#[function_component(AutoIngestPage)]
pub fn alula_bible_route() -> Html {
    auto_ingest_page::auto_ingest_page() 
}
 
#[function_component(RustSite)]
pub fn alula_bible_route() -> Html {
    rust_site::rust_site() 
}
 
#[function_component(Info)]
pub fn about_route() -> Html {
    info_page::info_page() 
}
  
#[function_component(LandingPage)]
pub fn landing_route() -> Html {
    landing_page::landing_page() 
}
#[function_component(HomePage)] 
pub fn home_route() -> Html {
    home_page::home_page() 
} 