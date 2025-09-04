# Portfolio Site

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
│   ├── deploy.yml          # Application deployment
│   └── terraform.yml       # Infrastructure management
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

**Option 2: GitHub Actions**
- Push to `main` branch
- Modify `Cargo.toml` to trigger deployment
- Use "Run workflow" button

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
gcloud run services list --region=europe-west9

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
- [GITHUB_SECRETS.md](docs/GITHUB_SECRETS.md) - GitHub secrets and service account setup
- [Terraform Documentation](https://www.terraform.io/docs)
- [Google Cloud Run Documentation](https://cloud.google.com/run/docs)

## 🔄 CI/CD Workflows

### Application Deployment (`deploy.yml`)
- Triggers on `Cargo.toml` changes or manual dispatch
- Builds Docker image
- Pushes to Artifact Registry
- Deploys to Cloud Run

### Infrastructure Management (`terraform.yml`)
- Triggers on `terraform/` directory changes
- Runs `terraform plan` on PRs
- Applies changes on merge to main
- Includes security and validation checks

## 🌐 Live Site

The portfolio is deployed at: [Your Domain](https://firathonca.online)

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Need help?** Check the [DEPLOYMENT.md](docs/DEPLOYMENT.md) guide or run `./scripts/terraform-check.sh` for validation.
