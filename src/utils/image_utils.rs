
use yew::prelude::{html,Html};


pub fn single_image_div_builder(file: &str) -> Html {
    html! {
        <div style = "display: flex; justify-content: center; align-items: center;">
            <img 
            src={format!("static/{}", file)} alt="Image not Found!"  
            style= "border-radius: 10px;
                    display: block;
                    max-width: 100%;
                    height:auto;
                    margin: 20px 0;"
                     
             />
        </div>
    }
}


fn image_carousel_builder_helper(file: &str) -> Html {
    html! {
        <div style = "display: flex; justify-content: center; align-items: center;">
            <img 
            src={format!("/static/{}", file)} alt="Image not Found!"  
            style= "border-radius: 10px;
                    display: block;
                    max-width: 100%;
                    height:auto;
                    margin: 20px 0;"
             />
        </div>
    }
}

pub fn image_carousel_builder(image_list: &[&str]) -> Html {
    html! {
        <section class="carousel" aria-label="Gallery">
            <ol class="carousel__viewport">
                { for image_list.iter().enumerate().map(|(_, file)| {
                    html! {
                            <div class="carousel__snapper">
                                { image_carousel_builder_helper(file) }
                            </div>
                    }
                })}
            </ol>
        </section>
    }
}
