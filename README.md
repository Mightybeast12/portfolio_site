# Portfolio Site

[![Rust CI - Lint, Test, Security & Dependency Checks](https://github.com/Mightybeast12/portfolio_site/actions/workflows/ci-quality.yml/badge.svg)](https://github.com/Mightybeast12/portfolio_site/actions/workflows/ci-quality.yml)
[![Auto-merge](https://github.com/Mightybeast12/portfolio_site/actions/workflows/auto-merge.yml/badge.svg)](https://github.com/Mightybeast12/portfolio_site/actions/workflows/auto-merge.yml)
[![Terraform Deploy](https://github.com/Mightybeast12/portfolio_site/actions/workflows/terraform.yml/badge.svg)](https://github.com/Mightybeast12/portfolio_site/actions/workflows/terraform.yml)
[![Auto-close stale issues & PRs (30/14 days)](https://github.com/Mightybeast12/portfolio_site/actions/workflows/maintenance.yml/badge.svg)](https://github.com/Mightybeast12/portfolio_site/actions/workflows/maintenance.yml)

A Rust-based portfolio website deployed on Google Cloud Run with automated CI/CD using GitHub Actions and Terraform.

## 🚀 Quick Start

### Prerequisites
- Docker Desktop or Colima
- Google Cloud CLI (`gcloud`)
- Terraform (v1.5.0+)
- Rust toolchain

### Initial Deployment

1. **Setup Infrastructure**
   ```bash
   cd terraform
   terraform init
   terraform apply
   ```

2. **Deploy Application**
   ```bash
   ./scripts/build-and-deploy.sh
   ```

3. **Complete Setup** (if Cloud Run creation failed initially)
   ```bash
   cd terraform
   terraform apply
   ```

## 📁 Project Structure

```
portfolio_site/
├── .github/workflows/
│   ├── auto-merge.yml      # Automated PR merging for dependencies
│   ├── ci-quality.yml      # Rust CI - lint, test, security & dependency checks
│   ├── maintenance.yml     # Issue and PR stale management
│   └── terraform.yml       # Unified infrastructure and deployment management
├── .github/
│   └── dependabot.yml      # Automated dependency updates
├── src/                    # Rust source code
├── terraform/              # Infrastructure as Code
│   ├── *.tf               # Terraform configurations
│   └── terraform.tfvars   # Environment variables
├── scripts/
│   ├── build-and-deploy.sh    # Local deployment script
│   └── terraform-check.sh     # Terraform validation
├── static/                 # Static assets
└── docs/                   # Documentation
    ├── DEPLOYMENT.md       # Detailed deployment guide
    ├── GCP_SETUP.md        # Google Cloud Platform setup
    └── GITHUB_SECRETS.md   # GitHub secrets setup
```

## 🔧 Development Workflow

### Local Development
```bash
# Run locally
trunk serve

# Build and test
cargo build
cargo test
```

### Deployment Options

**Option 1: Local Deployment**
```bash
./scripts/build-and-deploy.sh
```

**Option 2: GitHub Actions (Unified Terraform Workflow)**
- Push to `main` branch with changes to `Cargo.toml` or `terraform/` directory
- Automatic deployment triggered by version changes or infrastructure modifications
- Use "Run workflow" button for manual deployment
- Rich output display shows deployment URLs and configuration details

### Infrastructure Changes
```bash
# Validate changes
./scripts/terraform-check.sh

# Apply changes
cd terraform
terraform apply
```

## 🏗️ Architecture

### Infrastructure Components
- **Google Cloud Run**: Serverless container hosting
- **Artifact Registry**: Docker image storage
- **IAM**: Service accounts and permissions
- **Domain Mapping**: Custom domain configuration

### Deployment Strategy
- **Separated Concerns**: Terraform manages infrastructure, GitHub Actions handles deployments
- **Smart Building**: Automatic image building only when needed
- **Lifecycle Management**: Terraform ignores image changes to allow independent deployments

## 🔐 Security Features

- Service account authentication
- Least-privilege IAM roles
- Encrypted traffic (HTTPS)
- No hardcoded secrets
- Artifact Registry cleanup policies

## 📊 Monitoring & Logs

```bash
# View Cloud Run logs
gcloud logs read "resource.type=cloud_run_revision AND resource.labels.service_name=firat-portfolio-site" --limit=50

# Check service status
gcloud run services list --region=europe-west1

# View deployment history
cat scripts/.last_version
```

## 🛠️ Troubleshooting

### Common Issues

**"Artifact Registry does not exist"**
```bash
cd terraform && terraform apply
```

**"Image not found" during first deployment**
```bash
./scripts/build-and-deploy.sh
cd terraform && terraform apply
```

**Docker daemon not running**
```bash
# Start Docker Desktop or Colima
colima start  # or start Docker Desktop
```

### Useful Commands

```bash
# Check Terraform configuration
./scripts/terraform-check.sh

# Format Terraform files
cd terraform && terraform fmt

# View current infrastructure
cd terraform && terraform show

# Destroy all infrastructure
cd terraform && terraform destroy
```

## 📚 Documentation

- [DEPLOYMENT.md](docs/DEPLOYMENT.md) - Comprehensive deployment guide
- [GCP_SETUP.md](docs/GCP_SETUP.md) - Google Cloud Platform APIs and IAM setup
- [GITHUB_SECRETS.md](docs/GITHUB_SECRETS.md) - GitHub secrets and service account setup
- [Terraform Documentation](https://www.terraform.io/docs)
- [Google Cloud Run Documentation](https://cloud.google.com/run/docs)

## 🔄 CI/CD Workflows

### Rust CI - Lint, Test, Security & Dependency Checks (`ci-quality.yml`)
- Triggers on push/PR to main branch, weekly schedule, and manual dispatch
- Comprehensive quality checks including:
  - **Linting & Auto-fix**: Runs `cargo clippy` with automatic fixes for code quality
  - **Code Formatting**: Enforces consistent code style
  - **Testing**: Runs all tests with `cargo test`
  - **Security Audit**: Scans for known vulnerabilities with `cargo audit`
  - **Dependency Check**: Identifies outdated dependencies with `cargo outdated`
  - **License Compliance**: Reports licenses of all dependencies
  - **WASM Build**: Builds release bundle with Trunk
- Auto-increments patch version when clippy fixes are applied
- Special handling for Dependabot PRs (no auto-fix to prevent loops)
- Generates comprehensive workflow summaries

### Unified Infrastructure & Deployment (`terraform.yml`)
- Triggers on `Cargo.toml` changes, `terraform/` directory changes, or manual dispatch
- Smart change detection for infrastructure, application, and version changes
- Multi-job architecture with infrastructure planning and build-deploy phases
- Builds Docker image and pushes to Artifact Registry
- Deploys to Cloud Run with Terraform
- Rich output display showing Cloud Run URLs, custom domain URLs, Docker image info, and DNS configuration
- Proper Terraform state management with remote backend

### Automated PR Merging (`auto-merge.yml`)
- Automatically merges dependency update PRs from Dependabot
- Ensures all CI checks pass before merging
- Streamlines dependency management process

### Repository Maintenance (`maintenance.yml`)
- Runs daily at 1:30 AM UTC to manage inactive issues and PRs
- Marks issues as stale after 30 days of inactivity
- Marks PRs as stale after 14 days of inactivity
- Automatically closes stale items after 7 additional days
- Exempts important labels (pinned, security, enhancement, bug, work-in-progress)
- Excludes draft PRs from stale management
- Generates maintenance summary reports

### Automated Dependency Updates (Dependabot)
- Weekly updates for Rust dependencies, GitHub Actions, and Docker
- Automatically creates PRs for dependency updates
- Ignores major version updates for core dependencies (Yew, Yew-router)
- Properly labeled and assigned PRs
- Works seamlessly with auto-merge workflow

## 🌐 Live Site

**🚀 [View Live Portfolio](https://firathonca.online)**

The portfolio is automatically deployed using GitHub Actions with Terraform and hosted on Google Cloud Run.

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Need help?** Check the [DEPLOYMENT.md](docs/DEPLOYMENT.md) guide or run `./scripts/terraform-check.sh` for validation.
