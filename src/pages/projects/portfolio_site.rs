use yew::prelude::*;

pub fn portfolio_site_page() -> Html {
    html! {
        <div class="home-container">
            <div class="markdown-container">
                {portfolio_site_markdown()}
                <div class="button-20-container">
                    <a href="https://github.com/Mightybeast12/portfolio_site" target="_blank" class="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"VIEW ON GITHUB"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

pub fn portfolio_site_intro() -> Html {
    html! {
        <div>
            <h1>{"Portfolio Site - Terraform Infrastructure"}</h1>
            <p>{"This repository contains a Rust-based portfolio site with Terraform infrastructure for deployment to Google Cloud Platform."}</p>
            <ul class="feature-list">
                <li>{"Serverless deployment with Google Cloud Run"}</li>
                <li>{"Automated SSL/TLS certificates and infrastructure"}</li>
                <li>{"Cost-optimized (≈$0/month within free tier)"}</li>
                <li>{"One-click deployment and rollback scripts"}</li>
            </ul>
        </div>
    }
}

pub fn portfolio_site_markdown() -> Html {
    html! {
        <div>
            <h1>{"Portfolio Site - Terraform Infrastructure"}</h1>

            <p>{"This repository contains a Rust-based portfolio site with Terraform infrastructure for deployment to Google Cloud Platform."}</p>

            <h2>{"Architecture"}</h2>
            <ul>
                <li><strong>{"Frontend"}</strong>{": Rust + Trunk (compiled to WebAssembly)"}</li>
                <li><strong>{"Web Server"}</strong>{": Nginx (containerized)"}</li>
                <li><strong>{"Container Registry"}</strong>{": Google Artifact Registry with lifecycle policies"}</li>
                <li><strong>{"Compute"}</strong>{": Google Cloud Run (serverless containers)"}</li>
                <li><strong>{"Domain"}</strong>{": Cloud Run Domain Mapping with free SSL certificates"}</li>
                <li><strong>{"Infrastructure"}</strong>{": Terraform (Infrastructure as Code)"}</li>
            </ul>

            <h2>{"Key Features"}</h2>
            <ul>
                <li>{"Serverless deployment with Google Cloud Run"}</li>
                <li>{"🔒 Automated SSL/TLS certificates"}</li>
                <li>{"📦 Multi-stage Docker builds for optimization"}</li>
                <li>{"⚡ Infrastructure as Code with Terraform"}</li>
                <li>{"🔄 Automated CI/CD pipeline"}</li>
                <li>{"💰 Cost-optimized (≈$0/month within free tier)"}</li>
                <li>{"🔧 One-click deployment scripts"}</li>
                <li>{"📊 CloudWatch monitoring and alerting"}</li>
            </ul>

            <h2>{"Technology Stack"}</h2>
            <ul>
                <li>{"Frontend: Rust + Yew + WebAssembly"}</li>
                <li>{"Build Tool: Trunk"}</li>
                <li>{"Container: Docker + Nginx"}</li>
                <li>{"Infrastructure: Terraform"}</li>
                <li>{"Cloud Platform: Google Cloud (Cloud Run, Artifact Registry)"}</li>
                <li>{"Monitoring: CloudWatch + Slack integration"}</li>
            </ul>

            <h2>{"Infrastructure Highlights"}</h2>
            <p>{"The project showcases modern DevOps practices:"}</p>
            <ul>
                <li>{"Infrastructure as Code with Terraform modules"}</li>
                <li>{"Serverless containers with automatic scaling"}</li>
                <li>{"Cost-effective domain mapping (saves ~$18/month vs load balancer)"}</li>
                <li>{"Automated image lifecycle management"}</li>
                <li>{"One-click deployment and rollback capabilities"}</li>
                <li>{"Comprehensive monitoring and alerting setup"}</li>
            </ul>

            <h2>{"Deployment Features"}</h2>
            <ul>
                <li>{"🔄 One-command deployment: "}<code>{"./deploy.sh"}</code></li>
                <li>{"⏪ Easy rollback: "}<code>{"./scripts/rollback.sh"}</code></li>
                <li>{"🗂️ Automated backups with lifecycle policies"}</li>
                <li>{"📈 Real-time monitoring and Slack notifications"}</li>
                <li>{"🔐 Security best practices with least-privilege IAM"}</li>
                <li>{"💾 Image cleanup and cost optimization"}</li>
            </ul>
        </div>
    }
}
