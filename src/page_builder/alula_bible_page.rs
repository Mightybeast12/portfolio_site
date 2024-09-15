use yew::prelude::*; 
use yew_router::prelude::*; 
 
use crate::utils::mark_down_utils;
use crate::utils::image_utils::image_carousel_builder;
 
pub fn create_alula_bible_markdown() -> Html {
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
 
 
fn alula_bible_image_showcase() -> Html {
let images = vec![
    "stock_tracker/firo_tracker_dashboard.png".to_string(),
    "put images".to_string(),
];

    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}

 
  
pub fn alula_bible_page() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            {create_alula_bible_markdown()} 
             
        </div>
    
    }
}