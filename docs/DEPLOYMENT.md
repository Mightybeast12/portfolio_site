# Deployment Guide

This guide explains how to deploy and manage the portfolio site infrastructure using Terraform and GitHub Actions.

## Overview

The deployment system uses a **separated concerns approach**:
- **Terraform**: Manages infrastructure (Artifact Registry, Cloud Run, IAM, etc.)
- **GitHub Actions**: Handles application deployments and infrastructure updates
- **Local Scripts**: Allows manual deployments from your machine

## Initial Setup (First Time)

### Prerequisites
- Docker installed and running
- Google Cloud CLI (`gcloud`) installed and authenticated
- Terraform installed (v1.5.0+)
- Git repository with proper secrets configured

### Required GitHub Secrets
Add these secrets to your GitHub repository:
- `GCP_SA_KEY`: Service account JSON key with necessary permissions

For detailed setup instructions, see [GITHUB_SECRETS.md](GITHUB_SECRETS.md).

### Step-by-Step Initial Deployment

1. **Clone and Navigate to Project**
   ```bash
   git clone <your-repo>
   cd portfolio_site
   ```

2. **Initialize Terraform**
   ```bash
   cd terraform
   terraform init
   ```

3. **Review and Apply Infrastructure**
   ```bash
   # Review what will be created
   terraform plan

   # Apply infrastructure (creates Artifact Registry, IAM, etc.)
   terraform apply
   ```

   **Note**: The first `terraform apply` will create the Artifact Registry but Cloud Run service creation may fail because no Docker image exists yet. This is expected.

4. **Build and Push Initial Image**
   ```bash
   cd ..
   ./scripts/build-and-deploy.sh
   ```

5. **Complete Infrastructure Setup**
   ```bash
   cd terraform
   terraform apply
   ```

   Now Cloud Run service will be created successfully using the pushed image.

## Deployment Workflows

### 1. Infrastructure Changes (Terraform)

**Automatic via GitHub Actions:**
- Changes to `terraform/` directory trigger the Terraform workflow
- Pull requests show plan output for review
- Merges to `main` automatically apply changes

**Manual Local Deployment:**
```bash
cd terraform
terraform plan    # Review changes
terraform apply   # Apply changes
```

### 2. Application Deployments

**Option A: Local Deployment**
```bash
./scripts/build-and-deploy.sh
```

**Option B: GitHub Actions**
- Push changes to `main` branch
- Modify `Cargo.toml` to trigger deployment
- Or use "Run workflow" button for manual trigger

### 3. Complete Teardown

To destroy all infrastructure:
```bash
cd terraform
terraform destroy
```

## Configuration

### Terraform Variables
Key variables in `terraform/terraform.tfvars`:
- `project_id`: GCP project ID
- `region`: Deployment region
- `app_name`: Cloud Run service name
- `custom_domain`: Your domain name

### Build Script Configuration
The build script automatically reads configuration from `terraform/terraform.tfvars`.

## Troubleshooting

### Common Issues

**1. "Artifact Registry does not exist"**
```bash
# Solution: Run Terraform first
cd terraform && terraform apply
```

**2. "Cloud Run service creation failed - image not found"**
```bash
# Solution: Push initial image first
./scripts/build-and-deploy.sh
# Then apply Terraform again
cd terraform && terraform apply
```

**3. "Docker daemon not running"**
```bash
# Solution: Start Docker
# On macOS: Start Docker Desktop
# On Linux: sudo systemctl start docker
```

**4. "Permission denied" errors**
```bash
# Solution: Authenticate with GCP
gcloud auth login
gcloud auth configure-docker europe-west1-docker.pkg.dev
```

### Checking Status

**View current infrastructure:**
```bash
cd terraform
terraform show
```

**Check Cloud Run service:**
```bash
gcloud run services list --region=europe-west1
```

**View recent deployments:**
```bash
cat scripts/.last_version
cat scripts/.last_image
```

## Advanced Usage

### Environment-Specific Deployments

The system supports different deployment modes:

**Local Development:**
```bash
cd terraform
terraform apply -var="auto_build=true"
```

**CI/CD Environment:**
```bash
terraform apply -var="ci_environment=true" -var="auto_build=false"
```

### Custom Image Tags

To deploy a specific image version:
```bash
# Build with custom tag
docker build -t europe-west1-docker.pkg.dev/portfolio-site-434710/cv-portfolio-repo/rust-image-cv-image:v2.0.0 .
docker push europe-west1-docker.pkg.dev/portfolio-site-434710/cv-portfolio-repo/rust-image-cv-image:v2.0.0

# Update Cloud Run service
gcloud run deploy firat-portfolio-site \
  --image europe-west1-docker.pkg.dev/portfolio-site-434710/cv-portfolio-repo/rust-image-cv-image:v2.0.0 \
  --region europe-west1
```

### Rollback Procedures

**Using Terraform:**
```bash
cd terraform
git checkout <previous-commit>
terraform apply
```

**Using Cloud Run:**
```bash
# List revisions
gcloud run revisions list --service=firat-portfolio-site --region=europe-west1

# Rollback to specific revision
gcloud run services update-traffic firat-portfolio-site \
  --to-revisions=<revision-name>=100 \
  --region=europe-west1
```

## Security Considerations

- Service account keys are stored as GitHub secrets
- Artifact Registry has cleanup policies to manage storage costs
- Cloud Run service uses least-privilege IAM roles
- All traffic is encrypted in transit

## Monitoring and Logs

**View Cloud Run logs:**
```bash
gcloud logs read "resource.type=cloud_run_revision AND resource.labels.service_name=firat-portfolio-site" --limit=50
```

**Monitor deployments:**
- GitHub Actions: Check workflow runs in repository
- GCP Console: Cloud Run service metrics and logs
- Local: Check `scripts/.last_version` for deployment history

## Cost Optimization

- Artifact Registry cleanup policies remove old images
- Cloud Run scales to zero when not in use
- Resource limits prevent unexpected costs
- Use `terraform destroy` when not needed for development
