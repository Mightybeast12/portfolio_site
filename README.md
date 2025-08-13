# Portfolio Site - Terraform Infrastructure

This repository contains a Rust-based portfolio site with Terraform infrastructure for deployment to Google Cloud Platform.

## Architecture

- **Frontend**: Rust + Trunk (compiled to WebAssembly)
- **Web Server**: Nginx (containerized)
- **Container Registry**: Google Artifact Registry with lifecycle policies
- **Compute**: Google Cloud Run (serverless containers)
- **Domain**: Cloud Run Domain Mapping with free SSL certificates
- **Infrastructure**: Terraform (Infrastructure as Code)

## Project Structure

```
├── terraform/                 # Terraform infrastructure files
│   ├── providers.tf           # Google Cloud provider configuration
│   ├── versions.tf            # Terraform and provider versions
│   ├── variables.tf           # Input variables with defaults
│   ├── terraform.tfvars       # Variable values (customize for your setup)
│   ├── artifact-registry.tf   # Docker registry with lifecycle policies
│   ├── iam.tf                 # Service accounts and permissions
│   ├── cloud-run.tf           # Cloud Run service deployment
│   ├── domain-mapping.tf      # Cloud Run domain mapping (free SSL)
│   └── outputs.tf             # Output values and instructions
├── scripts/                   # Deployment and management scripts
│   ├── build-and-deploy.sh    # Build and deploy workflow
│   ├── rollback.sh            # Rollback to previous versions
│   └── cleanup-images.sh      # Manual image cleanup
├── src/                       # Rust source code
├── static/                    # Static assets
├── Dockerfile                 # Multi-stage Docker build
├── nginx.conf                 # Nginx configuration
└── build.sh                   # Original build script (replaced)
```

## Quick Start

### 1. Prerequisites

- [Terraform](https://terraform.io/) >= 1.0
- [Google Cloud CLI](https://cloud.google.com/sdk/docs/install)
- [Docker](https://docs.docker.com/get-docker/)
- Git
- Rust toolchain (for local development)

### 2. Initial Setup

1. **Clone and configure**:
   ```bash
   git clone <your-repo>
   cd portfolio_site
   ```

2. **Authenticate with Google Cloud**:
   ```bash
   gcloud auth login
   gcloud config set project portfolio-site-434710
   ```

3. **Domain is already configured for**: `portfolio.firathonca.online`

### 3. Deploy Infrastructure

1. **Initialize Terraform**:
   ```bash
   cd terraform
   terraform init
   ```

2. **Review the plan**:
   ```bash
   terraform plan
   ```

3. **Deploy infrastructure**:
   ```bash
   terraform apply
   ```

4. **Note the outputs**: After deployment, Terraform will show important information like:
   - Domain mapping status
   - DNS configuration needed
   - Repository URLs

### 4. Configure DNS

Add this CNAME record to your DNS provider (Porkbun):

```
Type: CNAME
Host: portfolio
Target: ghs.googlehosted.com
Full record: portfolio.firathonca.online -> ghs.googlehosted.com
```

### 5. Build and Deploy Application

```bash
# From project root
./scripts/build-and-deploy.sh
```

This script will:
- Build the Docker image locally
- Tag it with a version (timestamp + git commit)
- Push to Artifact Registry
- Update Cloud Run service
- Test the deployment

## Daily Workflow

### Deploy New Version
```bash
./scripts/build-and-deploy.sh
```

### Check Deployment Status
```bash
# View service details
gcloud run services describe firat-portfolio-site --region=europe-west9

# Check recent deployments
gcloud run revisions list --service=firat-portfolio-site --region=europe-west9
```

### Rollback if Needed
```bash
# List available versions
./scripts/rollback.sh --list

# Rollback to previous version
./scripts/rollback.sh --previous

# Rollback to specific version
./scripts/rollback.sh --version v1.0.0-20240113-abc1234
```

## Image Management

The Artifact Registry automatically keeps only the 2 most recent images. For manual management:

```bash
# List all images
./scripts/cleanup-images.sh --list

# List old images that would be cleaned
./scripts/cleanup-images.sh --list-old

# Manual cleanup (with confirmation)
./scripts/cleanup-images.sh --delete-old
```

## Configuration

### Environment Variables

Modify environment variables in `terraform/cloud-run.tf`:

```hcl
env {
  name  = "CUSTOM_VAR"
  value = "custom_value"
}
```

### Scaling Configuration

Adjust scaling in `terraform/variables.tf`:

```hcl
variable "max_instances" {
  default = 5  # Increase for higher traffic
}
```

## Troubleshooting

### Common Issues

1. **Domain mapping not ready**:
   - Domain mappings can take 5-15 minutes to provision
   - Check status: `terraform output domain_mapping_status`

2. **Domain not accessible**:
   - Verify CNAME record is correct: `portfolio.firathonca.online -> ghs.googlehosted.com`
   - Domain must be verified in Google Cloud Console first
   - SSL certificate provisioning takes time

3. **Build failures**:
   - Ensure Docker is running
   - Check gcloud authentication: `gcloud auth list`
   - Verify Artifact Registry permissions

4. **Cloud Run deployment issues**:
   - Check service logs: `gcloud run services logs read firat-portfolio-site --region=europe-west9`
   - Verify image exists in registry

### Useful Commands

```bash
# Check Terraform state
terraform show

# View all outputs
terraform output

# Check Cloud Run status
gcloud run services list --region=europe-west9

# View application logs
gcloud run services logs read firat-portfolio-site --region=europe-west9 --limit=50

# List images in registry
gcloud artifacts docker images list europe-west9-docker.pkg.dev/portfolio-site-434710/cv-portfolio-repo/rust-image-cv-image

# Check domain mapping status
gcloud run domain-mappings list --region=europe-west9
```

## Cost Optimization

- **Cloud Run**: Pay only for requests (free tier: 2M requests/month)
- **Domain Mapping**: FREE (no load balancer costs!)
- **Artifact Registry**: $0.10/GB/month (first 0.5GB free)
- **Total Cost**: ~$0/month within free tier limits