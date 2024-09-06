
use yew::prelude::*; 

use crate::utils::mark_down_utils;

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
    mark_down_utils::create_markdown(&markdown_input) 
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

pub fn create_home_page() -> Html{
    html!(
        <div>
            {create_inspekt_markdown()}
            {create_alula_auto_ingest_markdown()}
            {create_alula_bible_markdown()}
        </div>
    )
     


}