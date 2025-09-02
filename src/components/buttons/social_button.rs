//! Social media button components with animated tooltips

use crate::constants::social::*;
use yew::prelude::*;

/// Props for social media tooltip button
#[derive(Properties, PartialEq)]
pub struct SocialTooltipProps {
    pub name: String,
    pub username: String,
    pub link: String,
    pub link_text: String,
    pub icon_svg: Html,
}

/// A social media button with animated tooltip
#[function_component(SocialTooltip)]
pub fn social_tooltip(props: &SocialTooltipProps) -> Html {
    html! {
        <div class="unique-tooltip-container">
            <div class="unique-tooltip">
                <div class="unique-profile">
                    <div class="unique-user">
                        <div class="unique-details">
                            <div class="unique-name">{&props.name}</div>
                            <div class="unique-username">{&props.username}</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="unique-text">
                <a class="unique-icon" href={props.link.clone()}>
                    <div class="unique-layer">
                        <span></span>
                        <span></span>
                        <span></span>
                        <span></span>
                        {props.icon_svg.clone()}
                    </div>
                    <div class="unique-text">{&props.link_text}</div>
                </a>
            </div>
        </div>
    }
}

/// LinkedIn social button component
#[function_component(LinkedInButton)]
pub fn linkedin_button() -> Html {
    let icon_svg = html! {
        <svg class="unique-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
            <path d={LINKEDIN_SVG_PATH}></path>
        </svg>
    };

    html! {
        <SocialTooltip
            name={PERSONAL_NAME.to_string()}
            username={FULL_NAME.to_string()}
            link={LINKEDIN_URL.to_string()}
            link_text={LINKEDIN_LABEL.to_string()}
            icon_svg={icon_svg}
        />
    }
}

/// GitHub social button component
#[function_component(GitHubButton)]
pub fn github_button() -> Html {
    let icon_svg = html! {
        <svg class="unique-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
            {for GITHUB_SVG_PATHS.iter().zip(GITHUB_SVG_FILL_COLORS.iter()).map(|(path, color)| {
                html! { <path fill={*color} d={*path}></path> }
            })}
        </svg>
    };

    html! {
        <SocialTooltip
            name={PERSONAL_NAME.to_string()}
            username={GITHUB_USERNAME.to_string()}
            link={GITHUB_URL.to_string()}
            link_text={GITHUB_LABEL.to_string()}
            icon_svg={icon_svg}
        />
    }
}
