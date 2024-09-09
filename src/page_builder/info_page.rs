use yew::prelude::*;
use crate::page_builder::header_builder; 
  
pub fn info_page() -> Html{
    html!{
        <div>
            {header_builder::build_header()}
            <h1> {"Place Holder"} </h1>
        </div> 
    }
}   