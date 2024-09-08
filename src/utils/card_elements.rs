
use yew::prelude::*;
 
  
pub fn create_code_styled_markdown(title:String,input:String) -> Html {
    html! {
        <div class="card-code-1">
            <div class="top">
                <div class="circle red"></div>
                <div class="circle yellow"></div>
                <div class="circle green"></div>
            </div>
            <div class="header">
                <h2 id="title2">{title}</h2>
            </div>
            <div class="code-container">
                <pre id="code">{input}</pre>
            </div>
        </div>
    }
}