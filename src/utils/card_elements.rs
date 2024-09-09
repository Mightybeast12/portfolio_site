
use yew::prelude::*;
use crate::utils::file_reader::FileContent;
  
pub fn create_code_styled_markdown(title:String,input:String) -> Html {
    html! {
        <div> 
            <div class="card-code-1">
                <div class="top-card-code-1">
                    <div class="circle-card-code-1 red"></div>
                    <div class="circle-card-code-1 yellow"></div>
                    <div class="circle-card-code-1 green"></div>
                </div>
                <div class="header-card-code-1">
                    <h2 id="card-code-1-title">{title}</h2>
                </div>
                <div class="cardcode-1-code-container">
                    <pre id="card-code-1-code-block">{input}</pre>
                </div>
            </div>
        </div>
    }
}
 
pub fn create_dynamic_styled_markdown(title: String, file: String) -> Html {
    html! {
        <div style="padding: 20px;">  
            <FileContent file_path={file} title={title} />
        </div>
    }
}
