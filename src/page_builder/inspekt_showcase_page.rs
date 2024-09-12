use yew::prelude::*;
use crate::utils::image_utils::image_carousel_builder; 
 
pub fn inspekt_page() -> Html{
    let image = vec![
        "inspekt/autodeploy.png".to_string(),
        "inspekt/failed.png".to_string(),

    ]; 
     
    let image2 = vec![
        "inspekt/autodeploy.png".to_string(),
        "inspekt/failed.png".to_string(),

    ]; 
    html!{
        <div>
            <div> {image_carousel_builder(image)} </div> 
            <div> {image_carousel_builder(image2)} </div> 
         
        </div>
          
    }
} 
 
// #[function_component(App)]
// fn app() -> Html {
//     let images = vec![
//         "image1.jpg".to_string(),
//         "image2.jpg".to_string(),
//         "image3.jpg".to_string(),
//     ];
// 
//     html! {
//         <div>
//             <ImageCarousel images={images.clone()} />
//         </div>
//     }
// }