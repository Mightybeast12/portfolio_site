use yew::prelude::{html, Html};
use yew_router::prelude::*; 
use crate::utils::router::Route;
use crate::utils::button_elements; 

const LOGO_SIZE: &str = "44px";

fn firat_header() -> Html {
    let page_title = "Firat Honca";
    let github_url = "https://github.com/Mightybeast12";
    let linkedin_url = "https://www.linkedin.com/in/firathonca"; 
    let github_logo_url = "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"; 
    let linkedin_logo_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/LinkedIn_logo_initials.png"; 

    html!(
        <div style="display: flex; justify-content: space-between; align-items: center;">
            <h2 style="color: rgb(209, 167, 111); font-size: 48px;">{ page_title }</h2>
            <div style="display: flex; gap: 15px;">
                <Link<Route> to={Route::InspektPage}>
                    <button class ="button-54" role="button"> 
                    {"Inspekt"} 
                    </button>
                </Link<Route>>
                <Link<Route> to={Route::HomePage}>
                    <button class ="button-54" role="button"> 
                    {"Home"} 
                    </button>
                </Link<Route>>
                 
                {button_elements::button_4(github_url.to_string(),github_logo_url.to_string())} 
                {button_elements::button_4(linkedin_url.to_string(),linkedin_logo_url.to_string())} 
            </div>
        </div>
    )
}

pub fn build_header() -> Html {
    firat_header()
}
