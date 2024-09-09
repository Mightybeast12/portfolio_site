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
 
#[function_component(Info)]
pub fn about_route() -> Html {
    info_page::info_page() 
}
  
#[function_component(HomePage)] 
pub fn home_route() -> Html {
    home_page::home_page() 
} 