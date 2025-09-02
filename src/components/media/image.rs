//! Image display components

use yew::prelude::*;

/// Props for a basic image component
#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: String,
    pub alt: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub loading: Option<String>,
}

/// Basic image component with optional styling and loading attributes
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let mut img_attrs = vec![("src", props.src.clone()), ("alt", props.alt.clone())];

    if let Some(class) = &props.class {
        img_attrs.push(("class", class.clone()));
    }

    if let Some(width) = &props.width {
        img_attrs.push(("width", width.clone()));
    }

    if let Some(height) = &props.height {
        img_attrs.push(("height", height.clone()));
    }

    if let Some(loading) = &props.loading {
        img_attrs.push(("loading", loading.clone()));
    }

    html! {
        <img
            src={props.src.clone()}
            alt={props.alt.clone()}
            class={props.class.clone()}
            width={props.width.clone()}
            height={props.height.clone()}
            loading={props.loading.clone()}
        />
    }
}

/// Props for a zoomable image component
#[derive(Properties, PartialEq)]
pub struct ZoomImageProps {
    pub src: String,
    pub alt: String,
    #[prop_or("zoom-image".to_string())]
    pub class: String,
    #[prop_or_default]
    pub container_class: Option<String>,
}

/// Image component with zoom functionality
#[function_component(ZoomImage)]
pub fn zoom_image(props: &ZoomImageProps) -> Html {
    let container_class = props
        .container_class
        .as_deref()
        .unwrap_or("image-container")
        .to_string();
    let src = props.src.clone();
    let alt = props.alt.clone();
    let class = props.class.clone();

    html! {
        <div class={container_class}>
            <img
                src={src}
                alt={alt}
                class={class}
            />
        </div>
    }
}

/// Props for a responsive image component
#[derive(Properties, PartialEq)]
pub struct ResponsiveImageProps {
    pub src: String,
    pub alt: String,
    #[prop_or("responsive-image".to_string())]
    pub class: String,
    #[prop_or_default]
    pub srcset: Option<String>,
    #[prop_or_default]
    pub sizes: Option<String>,
}

/// Responsive image component with srcset support
#[function_component(ResponsiveImage)]
pub fn responsive_image(props: &ResponsiveImageProps) -> Html {
    let src = props.src.clone();
    let alt = props.alt.clone();
    let class = props.class.clone();
    let srcset = props.srcset.clone();
    let sizes = props.sizes.clone();

    html! {
        <img
            src={src}
            alt={alt}
            class={class}
            srcset={srcset}
            sizes={sizes}
        />
    }
}
