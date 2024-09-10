use yew::prelude::*;
use crate::utils::button_elements; 
pub fn landing_page() -> Html {
    html!{
    <div> 
        <div> 
            {button_elements::create_github_button_animated()}
         
            {button_elements::create_linked_in_button_animated()}
        </div>
    </div> 
    }

} 