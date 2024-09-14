
use yew::prelude::*; 

use crate::utils::mark_down_utils;
use crate::utils::card_elements;
use crate::page_builder::code_show_case_elements;
 
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

pub fn home_page() -> Html{
    html!(
        <div class="home-container">
            <div class = "markdown-container"> 
            {create_alula_auto_ingest_markdown()}
            {create_alula_bible_markdown()}
            </div> 
            {code_show_case_elements::auto_ingest_docker_container_showcase()} 
            {code_show_case_elements::rust_docker_container_showcase()} 
        </div>
    )
}