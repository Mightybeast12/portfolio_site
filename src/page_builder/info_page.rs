use crate::constants::social::*;
use crate::utils::button_elements;
use yew::prelude::*;

pub fn info_page() -> Html {
    html! {
        <div class="info-page-container">
            <div class="markdown-container">
                <h1>{"About Me"}</h1>
                <div class="info-content">
                    <div class="profile-section">
                        <h2>{"Professional Background"}</h2>
                        <p>
                            {"I'm a passionate software developer with expertise in full-stack development, "}
                            {"infrastructure automation, and modern web technologies. I specialize in Rust, "}
                            {"Python, JavaScript, and cloud technologies."}
                        </p>
                    </div>

                    <div class="skills-section">
                        <h2>{"Technical Skills"}</h2>
                        <div class="skills-grid">
                            <div class="skill-category">
                                <h3>{"Languages"}</h3>
                                <ul class="feature-list">
                                    <li>{"Rust"}</li>
                                    <li>{"Python"}</li>
                                    <li>{"JavaScript/TypeScript"}</li>
                                    <li>{"SQL"}</li>
                                </ul>
                            </div>
                            <div class="skill-category">
                                <h3>{"Frameworks & Tools"}</h3>
                                <ul class="feature-list">
                                    <li>{"Yew (Rust WebAssembly)"}</li>
                                    <li>{"Docker & Kubernetes"}</li>
                                    <li>{"Terraform"}</li>
                                    <li>{"GitHub Actions"}</li>
                                </ul>
                            </div>
                            <div class="skill-category">
                                <h3>{"Cloud Platforms"}</h3>
                                <ul class="feature-list">
                                    <li>{"Google Cloud Platform"}</li>
                                    <li>{"AWS"}</li>
                                    <li>{"GitLab CI/CD"}</li>
                                    <li>{"Cloud Run"}</li>
                                </ul>
                            </div>
                        </div>
                    </div>

                    <div class="contact-section">
                        <h2>{"Connect With Me"}</h2>
                        <p>{"Feel free to reach out for collaboration opportunities or just to say hello!"}</p>

                        <div class="contact-methods">
                            <div class="email-section">
                                <h3>{"Email"}</h3>
                                <div class="email-container">
                                    <span class="email-icon">{"📧"}</span>
                                    <span class="email-address">{"firathonca@gmail.com"}</span>
                                </div>
                            </div>

                            <div class="social-section">
                                <h3>{"Social Media"}</h3>
                                <div class="social-buttons">
                                    {button_elements::create_github_button_animated()}
                                    {button_elements::create_linked_in_button_animated()}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
