use yew::prelude::*;

pub fn gitlab_terraform_page() -> Html {
    html! {
        <div class="home-container">
            <div class="markdown-container">
                {gitlab_terraform_markdown()}
                <div class="button-20-container">
                    <a href="https://github.com/Mightybeast12/gitlab-terraform" target="_blank" class="button-link">
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

pub fn gitlab_terraform_intro() -> Html {
    html! {
        <div>
            <h1>{"Comprehensive Infrastructure Platform"}</h1>
            <p>{"Complete enterprise infrastructure platform on AWS with GitLab, identity management, logging, monitoring, documentation, and secure remote access."}</p>
            <ul class="feature-list">
                <li>{"Self-hosted GitLab CE with auto-scaling runners"}</li>
                <li>{"Identity management with Keycloak SSO"}</li>
                <li>{"Centralized logging with Graylog and monitoring"}</li>
                <li>{"Multi-environment support (dev/staging/production)"}</li>
            </ul>
        </div>
    }
}

pub fn gitlab_terraform_markdown() -> Html {
    html! {
        <div>
            <h1>{"Comprehensive Infrastructure Platform"}</h1>

            <p>{"This Terraform project provisions a complete enterprise infrastructure platform on AWS with GitLab, identity management, logging, monitoring, documentation, help desk, and secure remote access."}</p>

            <h2>{"Core Infrastructure"}</h2>
            <ul>
                <li><strong>{"VPC with Public Subnets"}</strong>{": Isolated network environment with internet gateway"}</li>
                <li><strong>{"AWS Client VPN"}</strong>{": Secure remote access with certificate-based authentication"}</li>
                <li><strong>{"Database"}</strong>{": PostgreSQL RDS instance with automated initialization"}</li>
                <li><strong>{"GitLab Instance"}</strong>{": Self-hosted GitLab CE on EC2 with auto-scaling runners"}</li>
                <li><strong>{"Vaultwarden Instance"}</strong>{": Self-hosted Bitwarden-compatible password manager"}</li>
            </ul>

            <h2>{"Platform Services"}</h2>
            <ul>
                <li>{"🔐 Keycloak: Identity and access management with SSO capabilities"}</li>
                <li>{"📊 Graylog: Centralized logging platform with Elasticsearch and MongoDB"}</li>
                <li>{"📈 Monitoring: CloudWatch integration with Slack notifications"}</li>
                <li>{"📚 WikiJS: Modern documentation platform"}</li>
                <li>{"🎫 Zammad: Help desk and customer support system"}</li>
            </ul>

            <h2>{"Key Features"}</h2>

            <h3>{"Security Features"}</h3>
            <ul>
                <li>{"🔒 VPN Access: Secure remote access to all platform services"}</li>
                <li>{"📜 Certificate Authentication: PKI-based VPN authentication"}</li>
                <li>{"🏗️ Network Isolation: VPC with proper subnet segmentation"}</li>
                <li>{"👤 IAM Roles: Least-privilege access for all components"}</li>
                <li>{"🛡️ Security Groups: Network-level access controls"}</li>
                <li>{"🔐 Encryption: Encryption at rest and in transit for all services"}</li>
            </ul>

            <h3>{"GitLab Features"}</h3>
            <ul>
                <li>{"📈 Auto-scaling Runners: Spot instances that scale based on CI/CD queue depth"}</li>
                <li>{"💾 Automated Backups: Daily backups to S3 with configurable retention"}</li>
                <li>{"💰 Cost Optimization: Spot instances for runners with configurable max price"}</li>
                <li>{"📊 Monitoring: CloudWatch integration for queue monitoring and alerting"}</li>
            </ul>

            <h2>{"Multi-Environment Support"}</h2>
            <p>{"Supports three environments with different configurations:"}</p>
            <ul>
                <li><strong>{"Development"}</strong>{": eu-west-2, 10.1.0.0/16, t3.small instances"}</li>
                <li><strong>{"Staging"}</strong>{": eu-west-1, 10.2.0.0/16, t3.medium instances"}</li>
                <li><strong>{"Production"}</strong>{": eu-central-1, 10.3.0.0/16, t3.large instances"}</li>
            </ul>

            <h2>{"Infrastructure Modules"}</h2>
            <ul>
                <li>{"🌐 Networking: VPC, subnets, security groups"}</li>
                <li>{"🔒 Client VPN: Secure remote access with certificates"}</li>
                <li>{"💾 Database: PostgreSQL RDS with initialization"}</li>
                <li>{"🦊 GitLab: GitLab CE with auto-scaling runners"}</li>
                <li>{"🔑 Keycloak: Identity and access management"}</li>
                <li>{"📊 Graylog: Centralized logging and analytics"}</li>
                <li>{"📈 Monitoring: CloudWatch and Slack integration"}</li>
                <li>{"📚 WikiJS: Documentation platform"}</li>
                <li>{"🎫 Zammad: Help desk system"}</li>
                <li>{"🔐 Vaultwarden: Password management"}</li>
            </ul>

            <h2>{"Cost Optimization"}</h2>
            <ul>
                <li>{"💰 Spot Instances: GitLab runners use cost-effective spot instances"}</li>
                <li>{"📈 Auto-scaling: Services scale based on demand"}</li>
                <li>{"📊 Resource Tagging: Comprehensive tagging for cost allocation"}</li>
                <li>{"⏰ Scheduling: Non-production environment scheduling"}</li>
            </ul>

            <h2>{"Enterprise Ready"}</h2>
            <ul>
                <li>{"🔄 Backup and Recovery: Automated backup procedures"}</li>
                <li>{"📋 Compliance: Audit logging and reporting capabilities"}</li>
                <li>{"🔒 Security: Multi-layered security controls"}</li>
                <li>{"📊 Monitoring: Comprehensive monitoring and alerting"}</li>
                <li>{"📚 Documentation: Complete setup and maintenance guides"}</li>
            </ul>
        </div>
    }
}
