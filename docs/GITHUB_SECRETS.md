# GitHub Repository Secrets & Service Account Setup

This document outlines the required GitHub secrets and Google Cloud service account roles needed for the CI/CD pipeline.

## 🔐 Required GitHub Secrets

Add these secrets to your GitHub repository under **Settings > Secrets and variables > Actions**:

### `GCP_SA_KEY`
**Type**: Repository Secret
**Description**: Google Cloud Service Account JSON key for authentication
**Format**: Complete JSON key file content

**How to obtain**:
1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Navigate to **IAM & Admin > Service Accounts**
3. Create a new service account or use existing one
4. Click on the service account email
5. Go to **Keys** tab
6. Click **Add Key > Create new key**
7. Choose **JSON** format
8. Download the JSON file
9. Copy the entire contents of the JSON file into this secret

**Example format**:
```json
{
  "type": "service_account",
  "project_id": "portfolio-site-434710",
  "private_key_id": "...",
  "private_key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----\n",
  "client_email": "github-actions@portfolio-site-434710.iam.gserviceaccount.com",
  "client_id": "...",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "..."
}
```

## 🔑 Required Service Account Roles

The service account used in `GCP_SA_KEY` must have the following IAM roles:

### Core Deployment Roles
```
roles/run.admin                    # Cloud Run service management
roles/iam.serviceAccountUser       # Use service accounts
roles/artifactregistry.admin       # Artifact Registry management
roles/storage.admin                # Cloud Storage (for Terraform state if using GCS backend)
```

### Additional Required Roles
```
roles/compute.networkAdmin         # Network configuration
roles/dns.admin                    # Domain mapping (if using custom domains)
roles/serviceusage.serviceUsageAdmin  # Enable/disable APIs
roles/resourcemanager.projectIamAdmin # IAM management
```

### Minimal Permission Set (Alternative)
If you prefer minimal permissions, create a custom role with these permissions:

```json
{
  "title": "Portfolio Site Deployer",
  "description": "Custom role for portfolio site CI/CD",
  "stage": "GA",
  "includedPermissions": [
    "run.services.create",
    "run.services.delete",
    "run.services.get",
    "run.services.list",
    "run.services.update",
    "run.services.setIamPolicy",
    "run.services.getIamPolicy",
    "run.revisions.get",
    "run.revisions.list",
    "run.configurations.get",
    "run.configurations.list",
    "run.locations.list",
    "artifactregistry.repositories.create",
    "artifactregistry.repositories.delete",
    "artifactregistry.repositories.get",
    "artifactregistry.repositories.list",
    "artifactregistry.repositories.update",
    "artifactregistry.repositories.setIamPolicy",
    "artifactregistry.repositories.getIamPolicy",
    "artifactregistry.dockerimages.get",
    "artifactregistry.dockerimages.list",
    "iam.serviceAccounts.create",
    "iam.serviceAccounts.delete",
    "iam.serviceAccounts.get",
    "iam.serviceAccounts.list",
    "iam.serviceAccounts.update",
    "iam.serviceAccounts.actAs",
    "resourcemanager.projects.get",
    "resourcemanager.projects.getIamPolicy",
    "resourcemanager.projects.setIamPolicy"
  ]
}
```

## 🛠️ Service Account Setup Commands

### Option 1: Using gcloud CLI

```bash
# Set your project ID
export PROJECT_ID="portfolio-site-434710"

# Create service account
gcloud iam service-accounts create github-actions \
    --display-name="GitHub Actions Service Account" \
    --description="Service account for GitHub Actions CI/CD" \
    --project=$PROJECT_ID

# Get the service account email
export SA_EMAIL="github-actions@${PROJECT_ID}.iam.gserviceaccount.com"

# Assign required roles
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/run.admin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/iam.serviceAccountUser"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/artifactregistry.admin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/compute.networkAdmin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/serviceusage.serviceUsageAdmin"

# Create and download key
gcloud iam service-accounts keys create github-actions-key.json \
    --iam-account=$SA_EMAIL \
    --project=$PROJECT_ID

# Display the key (copy this to GitHub secrets)
cat github-actions-key.json
```

### Option 2: Using Terraform

Add this to your Terraform configuration:

```hcl
# Service account for GitHub Actions
resource "google_service_account" "github_actions" {
  account_id   = "github-actions"
  display_name = "GitHub Actions Service Account"
  description  = "Service account for GitHub Actions CI/CD"
}

# Required IAM roles
resource "google_project_iam_member" "github_actions_roles" {
  for_each = toset([
    "roles/run.admin",
    "roles/iam.serviceAccountUser",
    "roles/artifactregistry.admin",
    "roles/compute.networkAdmin",
    "roles/serviceusage.serviceUsageAdmin"
  ])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.github_actions.email}"
}

# Create service account key
resource "google_service_account_key" "github_actions_key" {
  service_account_id = google_service_account.github_actions.name
}

# Output the key (base64 encoded)
output "github_actions_key" {
  value     = google_service_account_key.github_actions_key.private_key
  sensitive = true
}
```

## 🔍 Verification Steps

### 1. Test Service Account Permissions
```bash
# Authenticate with the service account
gcloud auth activate-service-account --key-file=github-actions-key.json

# Test Cloud Run access
gcloud run services list --region=europe-west1

# Test Artifact Registry access
gcloud artifacts repositories list --location=europe-west1

# Test IAM access
gcloud iam service-accounts list
```

### 2. Test GitHub Actions
1. Push a change to the `terraform/` directory
2. Check the GitHub Actions workflow runs successfully
3. Verify Terraform plan appears in PR comments
4. Merge PR and verify Terraform apply succeeds

## 🚨 Security Best Practices

### Service Account Security
- **Principle of Least Privilege**: Only assign necessary roles
- **Regular Rotation**: Rotate service account keys periodically
- **Monitoring**: Enable audit logging for service account usage
- **Conditional Access**: Consider using Workload Identity Federation instead of keys

### GitHub Secrets Security
- **Environment Protection**: Use environment-specific secrets for production
- **Branch Protection**: Require PR reviews for changes to workflows
- **Audit Trail**: Monitor secret usage in Actions logs

### Alternative: Workload Identity Federation (Recommended)

For enhanced security, consider using Workload Identity Federation instead of service account keys:

```yaml
# In your GitHub Actions workflow
- name: Authenticate to Google Cloud
  uses: google-github-actions/auth@v2
  with:
    workload_identity_provider: 'projects/123456789/locations/global/workloadIdentityPools/github-pool/providers/github-provider'
    service_account: 'github-actions@portfolio-site-434710.iam.gserviceaccount.com'
```

## 📋 Checklist

- [ ] Create Google Cloud service account
- [ ] Assign required IAM roles
- [ ] Generate service account key
- [ ] Add `GCP_SA_KEY` to GitHub repository secrets
- [ ] Test GitHub Actions workflow
- [ ] Verify Terraform operations work
- [ ] Enable audit logging
- [ ] Set up key rotation schedule

## 🆘 Troubleshooting

### Common Permission Errors

**"Permission denied" on Cloud Run**
```bash
# Add Cloud Run Admin role
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/run.admin"
```

**"Cannot access Artifact Registry"**
```bash
# Add Artifact Registry Admin role
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/artifactregistry.admin"
```

**"Cannot create IAM bindings"**
```bash
# Add Project IAM Admin role
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/resourcemanager.projectIamAdmin"
```

### Debugging Steps
1. Check GitHub Actions logs for specific error messages
2. Verify service account has required roles in GCP Console
3. Test service account locally with same operations
4. Ensure project APIs are enabled (Cloud Run, Artifact Registry, etc.)

---

**Need help?** Check the [DEPLOYMENT.md](DEPLOYMENT.md) guide or contact your GCP administrator.
