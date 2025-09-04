use yew::prelude::*;

use crate::page_builder::code_show_case_elements;
use crate::utils::card_elements;
use crate::utils::image_utils::image_carousel_builder;
use crate::utils::mark_down_utils;

pub fn alula_auto_ingest_markdown() -> Html {
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
    "#;
    mark_down_utils::create_markdown(markdown)
}

fn notification_scriptshowcase() -> Html {
    let file = "static/AutoIngest/Notification.py".to_string();

    card_elements::create_dynamic_styled_markdown("Notification - Inspekt".to_string(), file)
}

fn auto_ingest_image_showcase() -> Html {
    let images = vec![
        "auto_ingest/success_message.png".to_string(),
        "auto_ingest/Docerfile.png".to_string(),
        "auto_ingest/Log_inprogress.png".to_string(),
        "auto_ingest/SeqOutput.png".to_string(),
        "auto_ingest/Ingestmonitor_Notification.png".to_string(),
        "auto_ingest/Updated_screenshot.png".to_string(),
        "auto_ingest/Logger.png".to_string(),
        "auto_ingest/payload_example.png".to_string(),
        "auto_ingest/log_output.png".to_string(),
    ];
    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}

pub fn auto_ingest_page() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            {alula_auto_ingest_markdown()}
            // Images
            {auto_ingest_image_showcase()}
            <h2> {"Example of Auto Deplyoment"} </h2>
            {code_show_case_elements::auto_ingest_docker_container_showcase()}
            <h2> {"Notify Teams Script"} </h2>
            {notification_scriptshowcase()}
        </div>
    }
}
