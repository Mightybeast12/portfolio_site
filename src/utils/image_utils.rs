use yew::prelude::*;

fn image_carousel_builder_helper(file: &str) -> Html {
    html! {
        <div style="display: flex; justify-content: center; align-items: center;">
            <img 
                src={format!("/static/{}", file)} alt="Image not Found!"  
                style="border-radius: 10px;
                       display: block;
                       max-width: 100%;
                       height: auto;
                       margin: 20px 0;"
            />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CarouselProps {
    images: Vec<String>, // List of image file names (assuming they are stored locally)
}

#[function_component(ImageCarousel)]
fn image_carousel(props: &CarouselProps) -> Html {
    let current_index = use_state(|| 0);
    let image_count = props.images.len();

    let prev_image = {
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let prev_index = if *current_index == 0 {
                image_count - 1
            } else {
                *current_index - 1
            };
            current_index.set(prev_index);
        })
    };

    let next_image = {
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let next_index = (*current_index + 1) % image_count;
            current_index.set(next_index);
        })
    };

    html! {
        <div class="carousel">
            <button onclick={prev_image} class="carousel-control prev">{"‹"}</button>

            <div class="carousel-inner">
                { image_carousel_builder_helper(&props.images[*current_index]) }
            </div>

            <button onclick={next_image} class="carousel-control next">{"›"}</button>
        </div>
    }
}
pub fn image_carousel_builder(images: Vec<String>) -> Html {
    html! {
        <ImageCarousel images={images} />
    }
}


