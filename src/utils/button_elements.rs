
use yew::prelude::{html, Html};
 
const LOGO_SIZE: &str = "44px";
 
pub fn button_4(site_url: String, image_url: String) -> Html {
    html! {
        <a href={site_url} target="_blank" rel="noopener noreferrer">
            <button class="button-4" role="button">
                <img src={image_url} alt="URL Error"
                     style={format!("width: {}; height: {};", LOGO_SIZE, LOGO_SIZE)} />
            </button>
        </a>
    }
}

