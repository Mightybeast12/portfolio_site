use yew::prelude::*;
 
use crate::utils::image_utils::image_carousel_builder;
use crate::utils::mark_down_utils;
use crate::utils::card_elements;

fn inspekt_markdown_intro_markdown() -> Html {
    let markdown_input = r#"
# Inspekt
---  
A Linux-based command-line tool meticulously designed for the inspection of Digital Cinema Packages (DCPs). Developed to meet the needs of mastering teams, it performs over 180 rigorous checks, including hash validation, JPEG2000 bit-depth verification, and CCAP/subtitle language consistency. Additionally, Inspekt verifies metadata in Composition Playlists (CPLs) and dynamically inspects DCPs across various package types, such as SMPTE A 2.1/2 and IOP packages. By automating these essential checks, Inspekt significantly enhances productivity and ensures that each DCP complies with industry standards.
######
 **Git**: Version Control **|** 
 **Docker**: Auto deployment/building **|**
 **Linux**: OS **|** **Python** 
    "#;
    html!(
        <div class="markdown-container">
            {mark_down_utils::create_markdown(&markdown_input)}
        </div>
    )
}

fn inspekt_first_image_show_case() -> Html {
    let images = vec![
        "inspekt/Passed.png".to_string(),
        "inspekt/gitportfolio.png".to_string(),
        "inspekt/Progress_bar.png".to_string(),
    ];
    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}

fn inspek_part_two_markdown()  -> Html{
    let markdown_input = r#"
---
Inspekt checks all aspects of a **DCP**(*Digital Cinema Package*) 
- *XML* validation
- *JPEG2000* guard bit validation (**Causes older Projectors to crash**)
- **Asset Timing Checks** 
- **Atmos Sync Test**: Broken Sync trak validation
- **PDF and HTML Outputs**: Exports results into a portable format 
- ****: Exports results into a portable format 
 
    "#;
    html!(
        <div class="markdown-container">
            {mark_down_utils::create_markdown(&markdown_input)}
        </div>
    ) 
}

fn inspekt_report_play() -> Html {
    let html_content = include_str!("../../static/inspekt/test_report.html").to_string();
    html! {
        <div class = "inspekt-play-container">
            <h1 style = "margin-left: 40%;"> {"Test Report Example"} </h1>
            { Html::from_html_unchecked(AttrValue::from(html_content)) }
        </div>
    }
}
 
fn inspekt_second_image_show_case() -> Html {
    let images = vec![
        "inspekt/webuiexpanded.png".to_string(),
        "inspekt/failed.png".to_string(),
        "inspekt/autodeploy.png".to_string(),
        "inspekt/pdf_utils.png".to_string(),
    ];
    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}
 
fn inspekt_third_image_show_case() -> Html {
    let images = vec![
        "inspekt/file_utils.png".to_string(),
        "inspekt/logger_code.png".to_string(),
    ];
    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}
 
fn inspekt_code_show_case() -> Html {
     
    let file = "static/AutoIngest/Notification.py".to_string(); 
     
    card_elements::create_dynamic_styled_markdown("Notification - Inspekt".to_string(),file) 
}

fn inspekt_all_image() -> Html {
    let images = vec![
        "inspekt/autodeploy.png".to_string(),
        "inspekt/failed.png".to_string(),
        "inspekt/file_utils.png".to_string(),
        "inspekt/gitportfolio.png".to_string(),
        "inspekt/logger_code.png".to_string(),
        "inspekt/Passed.png".to_string(),
        "inspekt/pdf_utils.png".to_string(),
        "inspekt/Progress_bar.png".to_string(),
        "inspekt/recursive_dict.png".to_string(),
        "inspekt/reeloverview.png".to_string(),
        "inspekt/webuiexpanded.png".to_string(),
         
    ];
    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}

pub fn inspekt_page() -> Html {
    html! {
        <div>
            {inspekt_markdown_intro_markdown()}
            {inspekt_first_image_show_case()}
            {inspek_part_two_markdown()}
            {inspekt_report_play()}
            {inspekt_second_image_show_case()}
            {inspekt_code_show_case()}
            {inspekt_third_image_show_case()}
            {inspekt_all_image()}
        </div>
    }
}
