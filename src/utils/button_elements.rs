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
 
fn tooltip(name: String, user_name: String, link: String,link_text:String, icon_element: Html) -> Html {
    html! {
        <div class="unique-tooltip-container">
            <div class="unique-tooltip">
                <div class="unique-profile">
                    <div class="unique-user">
                        <div class="unique-details">
                            <div class="unique-name">{name}</div>
                            <div class="unique-username">{user_name}</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="unique-text">
                <a class="unique-icon" href={link}>
                    <div class="unique-layer">
                        <span></span>
                        <span></span>
                        <span></span>
                        <span></span>
                        {icon_element}
                    </div>
                    <div class="unique-text">{link_text}</div>
                </a>
            </div>
        </div>
    }
}

 
pub fn create_linked_in_button_animated() -> Html {
    let name = "Firat".to_string();
    let user_name = "Firat Honca".to_string();
    let link = "https://www.linkedin.com/in/firathonca/".to_string();
    let link_text = "LinkedIn".to_string(); 
    let icon_element = html! {
        <svg class="unique-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
            <path d="M41,4H9C6.24,4,4,6.24,4,9v32c0,2.76,2.24,5,5,5h32c2.76,0,5-2.24,5-5V9C46,6.24,43.76,4,41,4z M17,20v19h-6V20H17z M11,14.47c0-1.4,1.2-2.47,3-2.47s2.93,1.07,3,2.47c0,1.4-1.12,2.53-3,2.53C12.2,17,11,15.87,11,14.47z M39,39h-6c0,0,0-9.26,0-10 c0-2-1-4-3.5-4.04h-0.08C27,24.96,26,27.02,26,29c0,0.91,0,10,0,10h-6V20h6v2.56c0,0,1.93-2.56,5.81-2.56 c3.97,0,7.19,2.73,7.19,8.26V39z"></path>
        </svg>
    };
    tooltip(name, user_name, link, link_text, icon_element)
}

pub fn create_github_button_animated() -> Html {
    let name = "Firat".to_string();
    let user_name = "Mightybeast12".to_string(); 
    let link = "https://github.com/Mightybeast12".to_string();
    let link_text = "GitHub".to_string(); 
    let icon_element = html! {
        <svg class="unique-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
            <path fill="#e53935" d="M24 43L16 20 32 20z"></path>
            <path fill="#ff7043" d="M24 43L42 20 32 20z"></path>
            <path fill="#e53935" d="M37 5L42 20 32 20z"></path>
            <path fill="#ffa726" d="M24 43L42 20 45 28z"></path>
            <path fill="#ff7043" d="M24 43L6 20 16 20z"></path>
            <path fill="#e53935" d="M11 5L6 20 16 20z"></path>
            <path fill="#ffa726" d="M24 43L6 20 3 28z"></path>
        </svg>
    };
    tooltip(name, user_name, link, link_text, icon_element)
}
