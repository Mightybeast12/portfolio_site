use yew::prelude::{html,Html};
 
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
                <a href={github_url} target="_blank" rel="noopener noreferrer">
                    <img src={github_logo_url} alt="GitHub Logo" style={format!("width: {}; height: {}; ", LOGO_SIZE, LOGO_SIZE)} />
                </a>
                <a href={linkedin_url} target="_blank" rel="noopener noreferrer">
                    <img src={linkedin_logo_url} alt="LinkedIn Logo" style={format!("width: {}; height: {}; ", LOGO_SIZE, LOGO_SIZE)} />
                </a>
            </div>
        </div>
    )
}



pub fn build_header() -> Html {
    firat_header() 
    } 