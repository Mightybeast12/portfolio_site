
use yew::prelude::*; 
use yew_router::prelude::*; 
 
use crate::utils::mark_down_utils;
use crate::utils::card_elements;
use crate::page_builder::code_show_case_elements;
use crate::utils::router::Route; 
use crate::page_builder::inspekt_showcase_page; 
use crate::page_builder::stock_tracker_page; 
 
fn create_alula_auto_ingest_markdown() -> Html {
    let markdown = r#"
 # Alula Auto Ingest
### Automated Data Ingestion System

A fully automated **Docker**-based microservices architecture designed to streamline the data ingestion process, tailored to handle sender-specific factors such as **Apple** and **Netflix** content.

## Features
**Dynamic Folder Structure Creation:** Automatically generates folder structures based on incoming data, customized for specific content providers (e.g., **Apple**, **Netflix**).
**Automated Notifications:** Sends real-time notifications to relevant teams, ensuring timely updates and action on new data.
**Pro Tools Session Creation:** Automatically creates **Pro Tools** sessions for **Audio QC**, simplifying the workflow for audio quality control tasks.
**Microservices Architecture:** Scalable **Docker** microservices allow flexible handling of varying workloads and sender-specific requirements.
**Customizable Workflow:** Tailored to meet the unique needs of each content provider, ensuring efficient data handling and processing.

Perfect for media teams managing diverse content pipelines and looking for an efficient, automated workflow for data ingestion!

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

A custom **Excel** workflow designed for handling Apple titles and generating labels for outgoing data, streamlining the data management process.

## Features
 **Automated Label Creation:** Automatically generates labels for Apple titles with precision and efficiency.
 **Python Integration:** Utilizes **Python** scripts for advanced data processing and manipulation within **Excel**.
 **Excel Compatibility:** Seamless integration with **Excel** for easy data handling and customization.
 **Windows Environment:** Optimized to run on **Windows**, leveraging native system resources for smooth performance.
 **Efficient Workflow:** A tailored process to handle large volumes of data, simplifying workflows for outgoing labels.

Ideal for professionals dealing with high volumes of data needing quick and accurate labeling solutions!

**Python** **|** **Excel** **|** **Windows**
    "# ; 
    mark_down_utils::create_markdown(&markdown) 
 
}

pub fn home_page() -> Html {
    html!(
        <div class="home-container">
            <div class="markdown-container"> 
                {inspekt_showcase_page::inspekt_markdown_intro_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::InspektPage} classes="button-link">
                        <div class="button-20">  
                                <span class="button-20-text">{"CHECK OUT"}</span>   
                                <span class="button-20-arrow"></span>  
                                </div>
                    </Link<Route>>
                </div>
            </div>
            <div class="markdown-container"> 
                {create_alula_auto_ingest_markdown()}
                <div class="button-20-container">
                    <Link<Route> to={Route::InspektPage} classes="button-link">
                        <div class="button-20">  
                                <span class="button-20-text">{"Check out"}</span>   
                                <span class="button-20-arrow"></span>  
                                </div>
                    </Link<Route>>
                </div>
            </div>
            <div class="markdown-container"> 
            {stock_tracker_page::create_stock_tracker_markdown()} 
                <div class="button-20-container">
                    <Link<Route> to={Route::StockTrackerPage} classes="button-link">
                        <div class="button-20">  
                            <span class="button-20-text">{"CHECK OUT"}</span>   
                            <span class="button-20-arrow"></span>  
                        </div>
                    </Link<Route>>
                </div>
            </div>
            <div class="markdown-container"> 
                {create_alula_bible_markdown()}
                <div class="button-20-container"> 
                    <Link<Route> to={Route::InspektPage} classes="button-link">
                        <div class="button-20">  
                                <span class="button-20-text">{"Check out"}</span>   
                                <span class="button-20-arrow"></span>  
                                </div>
                    </Link<Route>>
                </div>
                {code_show_case_elements::auto_ingest_docker_container_showcase()} 
                </div> 
            {code_show_case_elements::rust_docker_container_showcase()} 
        </div>
    )
}
