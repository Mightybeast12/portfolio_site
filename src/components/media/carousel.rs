//! Image carousel components

use yew::prelude::*;

/// Props for individual carousel image
#[derive(Properties, PartialEq)]
pub struct CarouselImageProps {
    pub src: String,
    pub alt: String,
    #[prop_or("zoom-image".to_string())]
    pub class: String,
}

/// Individual carousel image component
#[function_component(CarouselImage)]
pub fn carousel_image(props: &CarouselImageProps) -> Html {
    html! {
        <div class="image-container">
            <img
                src={props.src.clone()}
                alt={props.alt.clone()}
                class={props.class.clone()}
            />
        </div>
    }
}

/// Props for the image carousel
#[derive(Properties, PartialEq)]
pub struct ImageCarouselProps {
    pub images: Vec<String>,
    #[prop_or("carousel".to_string())]
    pub class: String,
    #[prop_or("/static".to_string())]
    pub base_path: String,
    #[prop_or("Image not Found!".to_string())]
    pub default_alt: String,
}

/// Interactive image carousel component with navigation controls
#[function_component(ImageCarousel)]
pub fn image_carousel(props: &ImageCarouselProps) -> Html {
    let current_index = use_state(|| 0);
    let image_count = props.images.len();

    // Early return if no images
    if image_count == 0 {
        return html! {
            <div class="carousel-empty">
                <p>{"No images to display"}</p>
            </div>
        };
    }

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

    let current_image = &props.images[*current_index];
    let image_src = format!("{}/{}", props.base_path, current_image);

    html! {
        <div class={props.class.clone()}>
            if image_count > 1 {
                <button onclick={prev_image} class="carousel-control prev">{"‹"}</button>
            }

            <div class="carousel-inner">
                <CarouselImage
                    src={image_src}
                    alt={props.default_alt.clone()}
                />
            </div>

            if image_count > 1 {
                <button onclick={next_image} class="carousel-control next">{"›"}</button>
            }

            if image_count > 1 {
                <div class="carousel-indicators">
                    {for (0..image_count).map(|index| {
                        let is_active = index == *current_index;
                        let current_index = current_index.clone();
                        let onclick = Callback::from(move |_| current_index.set(index));

                        html! {
                            <button
                                class={if is_active { "indicator active" } else { "indicator" }}
                                onclick={onclick}
                            />
                        }
                    })}
                </div>
            }
        </div>
    }
}

// Legacy function wrapper for backward compatibility
/// Legacy wrapper for image_carousel_builder function
pub fn image_carousel_builder(images: Vec<String>) -> Html {
    html! {
        <ImageCarousel images={images} />
    }
}
