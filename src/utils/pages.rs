use yew::prelude::*;
 
use crate::page_builder::{home_page, inspekt_showcase_page};
 
#[function_component(InspektPage)]
pub fn inspekt_route() -> Html {
    inspekt_showcase_page::inspekt_page()
}
#[function_component(HomePage)] 
pub fn home_route() -> Html {
    home_page::home_page() 
} 