use yew::prelude::*;
use pulldown_cmark::{html, Parser};

fn create_markdown(markdown_input: &str) -> Html {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}
 
fn create_inspekt_markdown() -> Html{
    let markdown_input = r#"
# Inspekt
Inspekt is a CLI based tool that 
A tailor-made Linux CLI-based Digital Cinema Package inspection tool performing over 180 checks, including hash validation, JPEG2000 bit checks, and CCAP/subtitle language consistency, significantly boosting the mastering team's productivity.
######
 **Git**: Version Control **|** 
 **Docker**: Auto deployment/building **|**
 **Linux**: Os **|** **Python** 
    "# ;
    create_markdown(markdown_input)
}  
 
fn create_alula_auto_ingest_markdown() -> Html {
    let markdown = r#"
 # Alula Auto Ingest

Automated the data ingestion process using Docker microservices, tailored to handle sender-specific factors (e.g., Apple, Netflix). The system dynamically creates folder structures and sends notifications to relevant teams, such as creating Pro Tools sessions for Audio QC, based on the incoming data. 
#
 **Docker**: Containerized Service **|**
 **Git**: Version Control **|**
 **Microsoft Teams**: Notifications\
 **Adaptive Cards**: Interaction and Asset Updates **|**
 **Power Automate**: Teams HTTP Request Handling
 **AWS EC2**: Flask Relay Serve   
    "# ;
    create_markdown(&markdown) 
}
 
fn create_alula_bible_markdown() -> Html {
    let markdown = r#"
# Alula Bible
Custom Excel workflow for Apple titles to create labels for outgoing data.\
**Python** **|** **Excel** **|** **Windows**
    "# ; 
    create_markdown(&markdown) 
 
}
 
const LOGO_SIZE: &str = "44px"; 
 
// fn add_html_picture(site_url: &str,site_logo_url: &str){
//     html!(
//         
//     )
// }

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

  
#[function_component]
fn App() -> Html {
    html! {
        <div> 
        <h1> {firat_header()} </h1> 
            <div> 
                {create_inspekt_markdown()}
                {create_alula_auto_ingest_markdown()}
                {create_alula_bible_markdown()}
            </div> 
        </div> 
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
