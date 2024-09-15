
use yew::prelude::*; 
use yew_router::prelude::*; 
 
use crate::utils::mark_down_utils;
use crate::utils::card_elements;
use crate::page_builder::code_show_case_elements;
use crate::utils::router::Route; 
use crate::page_builder::inspekt_showcase_page; 
 
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
    mark_down_utils::create_markdown(&markdown) 
}
 
fn create_alula_bible_markdown() -> Html {
    let markdown = r#"
# Alula Bible
Custom Excel workflow for Apple titles to create labels for outgoing data.\
**Python** **|** **Excel** **|** **Windows**
    "# ; 
    mark_down_utils::create_markdown(&markdown) 
 
}

pub fn home_page() -> Html {
    html!(
        <div class="home-container">
            <div class="markdown-container"> 
                {inspekt_showcase_page::inspekt_markdown_intro_markdown()}
                <Link<Route> to={Route::InspektPage} >
                    <div class="button-20-container">
                        <div class="button-20">  
                            <span class="button-20-text">{"More"}</span>   
                            <span class="button-20-arrow"></span>   
                        </div>
                    </div>
                </Link<Route>>
            </div>
            <div class="markdown-container"> 
                {create_alula_auto_ingest_markdown()}
                <Link<Route> to={Route::InspektPage} >
                    <div class="button-20-container">
                        <div class="button-20">  
                            <span class="button-20-text">{"Check out"}</span>   
                            <span class="button-20-arrow"></span>   
                        </div>
                    </div>
                </Link<Route>>
            </div>
            <div class="markdown-container"> 
                {create_alula_bible_markdown()}
                <div class="button-20-container"> 
                    <div class="button-20">  
                        <span class="button-20-text">{"Check out"}</span>   
                        <span class="button-20-arrow"></span>   
                    </div>
                </div>
                {code_show_case_elements::auto_ingest_docker_container_showcase()} 
            </div> 
            {code_show_case_elements::rust_docker_container_showcase()} 
        </div>
    )
}
