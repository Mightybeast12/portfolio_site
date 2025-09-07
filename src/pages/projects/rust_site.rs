use yew::prelude::*;

use crate::components::media::image_carousel_builder;
use crate::services::markdown_service;
use crate::shared::ui::cards;

pub fn port_folio_site_markdown() -> Html {
    let markdown = r#"
# Rust-Based Portfolio Website

This website is built using **Rust** and the **Yew** framework to showcase my work on various projects. The site is hosted on **Google Cloud Run** with automated CI/CD deployment through GitHub Actions.

## Key Features:
- Built with **Yew's routing** and **web-worker** capabilities
- Packaged in a **Docker container** for easy deployment
- Deployed on **Google Cloud Run** for scalability
- Infrastructure managed with **Terraform**
- Automatically built and pushed to **Google Cloud Artifact Registry**
- Custom domain mapping with Cloudflare DNS integration
- Responsive design with modular component architecture

"#;

    markdown_service::create_markdown(markdown)
}

pub fn file_structure_showcase() -> Html {
    let file_tree = r#"
portfolio_site/
├── src/
│   ├── main.rs                    # Entry point
│   ├── lib.rs                     # Library root
│   ├── components/                # UI Components
│   │   ├── buttons/
│   │   ├── cards/
│   │   ├── layout/
│   │   ├── media/
│   │   └── navigation/
│   ├── pages/                     # Page components
│   │   ├── core/                  # Core pages (home, contact, etc)
│   │   └── projects/              # Project showcase pages
│   ├── routing/                   # Yew router configuration
│   ├── services/                  # Services (markdown, file)
│   ├── shared/                    # Shared utilities
│   │   ├── helpers/
│   │   └── ui/
│   └── constants/                 # Constants and configs
├── static/                        # Static assets
│   ├── styles/                    # CSS files
│   └── [project folders]/         # Project-specific assets
├── terraform/                     # Infrastructure as Code
│   ├── artifact-registry.tf       # Docker registry
│   ├── cloud-run.tf              # Cloud Run service
│   ├── cloud-dns.tf              # DNS configuration
│   ├── docker-build.tf           # Build automation
│   ├── domain-mapping.tf         # Custom domain setup
│   ├── iam.tf                    # Service accounts & permissions
│   └── variables.tf              # Configuration variables
├── .github/workflows/             # CI/CD pipelines
├── Cargo.toml                    # Rust dependencies
├── Dockerfile                    # Container configuration
├── nginx.conf                    # Web server config
└── index.html                    # HTML template
"#;

    cards::create_code_styled_markdown("Project Structure".to_string(), file_tree.to_string())
}

pub fn rust_code_examples() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px;">
            {main_rs_example()}
            {router_example()}
            {component_example()}
        </div>
    }
}

fn main_rs_example() -> Html {
    let code = r#"// src/main.rs - Application Entry Point
use cv_portfolio_site::routing::RouterComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <RouterComponent/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"#;

    cards::create_code_styled_markdown("main.rs".to_string(), code.to_string())
}

fn router_example() -> Html {
    let code = r#"// src/routing/router.rs - Route Configuration
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,

    #[at("/rust_site")]
    RustSite,

    #[at("/stock_tracker")]
    StockTrackerPage,

    // ... other routes
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => core::home::home_page(),
        Route::RustSite => projects::rust_site::rust_site(),
        Route::StockTrackerPage => projects::stock_tracker::stock_tracker_page(),
        // ... route handlers
    }
}"#;

    cards::create_code_styled_markdown("router.rs".to_string(), code.to_string())
}

fn component_example() -> Html {
    let code = r#"// Example Yew Component Structure
use yew::prelude::*;

#[function_component(ProjectPage)]
pub fn project_page() -> Html {
    html! {
        <div style="display: flex; flex-direction: column;">
            {markdown_content()}
            {code_examples()}
            {image_carousel()}
        </div>
    }
}"#;

    cards::create_code_styled_markdown("Component Pattern".to_string(), code.to_string())
}

pub fn terraform_examples() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px;">
            {cloud_run_tf_example()}
            {artifact_registry_tf_example()}
            {variables_tf_example()}
        </div>
    }
}

fn cloud_run_tf_example() -> Html {
    let code = "# terraform/cloud-run.tf - Cloud Run Service Configuration
resource \"google_cloud_run_v2_service\" \"portfolio_site\" {
  name     = var.app_name
  location = var.region
  ingress  = \"INGRESS_TRAFFIC_ALL\"

  template {
    service_account = google_service_account.cloud_run_service.email

    scaling {
      max_instance_count = var.max_instances
    }

    containers {
      image = \"${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:${var.image_tag}\"

      ports {
        container_port = 8080
      }

      resources {
        limits = {
          cpu    = var.cpu_limit
          memory = var.memory_limit
        }
      }

      env {
        name  = \"ENVIRONMENT\"
        value = \"production\"
      }
    }
  }

  traffic {
    percent = 100
    type    = \"TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST\"
  }
}";

    cards::create_code_styled_markdown("cloud-run.tf".to_string(), code.to_string())
}

fn artifact_registry_tf_example() -> Html {
    let code = "# terraform/artifact-registry.tf - Docker Registry Setup
resource \"google_artifact_registry_repository\" \"portfolio_images\" {
  provider      = google
  location      = var.region
  repository_id = var.repo_name
  description   = \"Docker repository for portfolio site images\"
  format        = \"DOCKER\"

  cleanup_policies {
    id     = \"keep-recent-versions\"
    action = \"DELETE\"
    condition {
      tag_state  = \"TAGGED\"
      older_than = \"604800s\"  # 7 days
    }
  }

  cleanup_policies {
    id     = \"delete-untagged\"
    action = \"DELETE\"
    condition {
      tag_state  = \"UNTAGGED\"
      older_than = \"4800s\"  # 80 minutes
    }
  }
}";

    cards::create_code_styled_markdown("artifact-registry.tf".to_string(), code.to_string())
}

fn variables_tf_example() -> Html {
    let code = "# terraform/variables.tf - Configuration Variables
variable \"project_id\" {
  description = \"The GCP project ID\"
  type        = string
  default     = \"portfolio-site-434710\"
}

variable \"region\" {
  description = \"The GCP region for resources\"
  type        = string
  default     = \"europe-west1\"
}

variable \"app_name\" {
  description = \"Name of the application\"
  type        = string
  default     = \"firat-portfolio-site\"
}

variable \"custom_domain\" {
  description = \"Custom domain for the portfolio site\"
  type        = string
  default     = \"firathonca.online\"
}

variable \"max_instances\" {
  description = \"Maximum number of Cloud Run instances\"
  type        = number
  default     = 2
}

variable \"cpu_limit\" {
  description = \"CPU limit for Cloud Run service\"
  type        = string
  default     = \"1000m\"
}";

    cards::create_code_styled_markdown("variables.tf".to_string(), code.to_string())
}

fn portfolio_showcase_images() -> Html {
    let images = vec![
        "rust_site/cloud_run_home_page.png".to_string(),
        "rust_site/auto_deploy_cloud.png".to_string(),
        "rust_site/github_home.png".to_string(),
        "rust_site/button_styles.png".to_string(),
        "rust_site/artificat_docker.png".to_string(),
        "rust_site/creating_elements.png".to_string(),
        "rust_site/routing.png".to_string(),
    ];

    html! {
        <div>
            {image_carousel_builder(images)}
        </div>
    }
}

pub fn rust_site() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 40px;">
            {port_folio_site_markdown()}

            <div style="width: 100%; max-width: 900px;">
                <h2 style="margin-bottom: 20px;">{"📁 Project Structure"}</h2>
                {file_structure_showcase()}
            </div>

            <div style="width: 100%; max-width: 900px;">
                <h2 style="margin-bottom: 20px;">{"🦀 Rust Code Examples"}</h2>
                {rust_code_examples()}
            </div>

            <div style="width: 100%; max-width: 900px;">
                <h2 style="margin-bottom: 20px;">{"🔧 Terraform Infrastructure"}</h2>
                {terraform_examples()}
            </div>

            <div style="width: 100%; max-width: 900px;">
                <h2 style="margin-bottom: 20px;">{"Screenshots"}</h2>
                {portfolio_showcase_images()}
            </div>
        </div>
    }
}
