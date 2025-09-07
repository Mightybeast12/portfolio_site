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
    let show_fullscreen = use_state(|| false);
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

    let open_fullscreen = {
        let show_fullscreen = show_fullscreen.clone();
        Callback::from(move |_| {
            show_fullscreen.set(true);
        })
    };

    let close_fullscreen = {
        let show_fullscreen = show_fullscreen.clone();
        Callback::from(move |_| {
            show_fullscreen.set(false);
        })
    };

    html! {
        <div class={props.class.clone()}>
            if image_count > 1 {
                <button onclick={prev_image} class="carousel-control prev">{"‹"}</button>
            }

            <div class="carousel-inner">
                <div class="image-container" onclick={open_fullscreen.clone()}>
                    <img
                        src={image_src.clone()}
                        alt={props.default_alt.clone()}
                        class="carousel-image clickable"
                    />
                </div>

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

            if image_count > 1 {
                <button onclick={next_image} class="carousel-control next">{"›"}</button>
            }

            if *show_fullscreen {
                <div class="fullscreen-overlay" onclick={close_fullscreen.clone()}>
                    <div class="fullscreen-container">
                        <img
                            src={image_src}
                            alt={props.default_alt.clone()}
                            class="fullscreen-image"
                        />
                        <button class="fullscreen-close" onclick={close_fullscreen}>{"×"}</button>
                    </div>
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
