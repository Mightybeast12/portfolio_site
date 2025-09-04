# Google Cloud Platform Setup Guide

This guide walks you through setting up the required Google Cloud Platform (GCP) APIs and IAM roles for the portfolio site infrastructure.

## 📋 Prerequisites

- Google Cloud Project created (`portfolio-site-434710`)
- `gcloud` CLI installed and authenticated
- Project Owner or Editor permissions (to assign roles)

## 🔧 Required GCP APIs

The following APIs must be enabled in your project before running Terraform:

### Core APIs
```bash
# Enable all required APIs at once
gcloud services enable \
    artifactregistry.googleapis.com \
    run.googleapis.com \
    iam.googleapis.com \
    serviceusage.googleapis.com \
    compute.googleapis.com \
    cloudbuild.googleapis.com \
    --project=portfolio-site-434710
```

### Individual API Descriptions

| API | Service Name | Purpose |
|-----|--------------|---------|
| **Artifact Registry API** | `artifactregistry.googleapis.com` | Store Docker images |
| **Cloud Run API** | `run.googleapis.com` | Deploy containerized applications |
| **Identity and Access Management API** | `iam.googleapis.com` | Manage service accounts and permissions |
| **Service Usage API** | `serviceusage.googleapis.com` | Enable/disable other APIs |
| **Compute Engine API** | `compute.googleapis.com` | Network and compute resources |
| **Cloud Build API** | `cloudbuild.googleapis.com` | Build Docker images (optional) |

### Verify APIs are Enabled

```bash
# List all enabled APIs
gcloud services list --enabled --project=portfolio-site-434710

# Check specific APIs
gcloud services list --enabled --filter="name:artifactregistry.googleapis.com OR name:run.googleapis.com OR name:iam.googleapis.com" --project=portfolio-site-434710
```

## 🔑 Required IAM Roles

### For Terraform Service Account (GitHub Actions)

The service account used in `GCP_SA_KEY` needs these roles:

#### Core Deployment Roles
```bash
PROJECT_ID="portfolio-site-434710"
SA_EMAIL="github-actions@${PROJECT_ID}.iam.gserviceaccount.com"  # Replace with your SA email

# Artifact Registry management
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/artifactregistry.admin"

# Cloud Run management
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/run.admin"

# Service account management
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/iam.serviceAccountAdmin"

# IAM policy management
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/resourcemanager.projectIamAdmin"

# Service usage (enable APIs)
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/serviceusage.serviceUsageAdmin"

# Network management (for domain mapping)
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/compute.networkAdmin"
```

#### Role Descriptions

| Role | Purpose | Permissions |
|------|---------|-------------|
| **`roles/artifactregistry.admin`** | Manage Artifact Registry repositories | Create, delete, configure Docker repositories |
| **`roles/run.admin`** | Manage Cloud Run services | Deploy, update, delete Cloud Run services |
| **`roles/iam.serviceAccountAdmin`** | Manage service accounts | Create service accounts for Cloud Run |
| **`roles/resourcemanager.projectIamAdmin`** | Manage IAM policies | Assign roles to service accounts |
| **`roles/serviceusage.serviceUsageAdmin`** | Enable/disable APIs | Enable required GCP APIs |
| **`roles/compute.networkAdmin`** | Manage network resources | Configure domain mapping and networking |

### For Local Development

If you're running Terraform locally, your user account needs similar permissions:

```bash
USER_EMAIL="your-email@domain.com"  # Replace with your email
PROJECT_ID="portfolio-site-434710"

# Add roles to your user account
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="user:${USER_EMAIL}" \
    --role="roles/artifactregistry.admin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="user:${USER_EMAIL}" \
    --role="roles/run.admin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="user:${USER_EMAIL}" \
    --role="roles/iam.serviceAccountAdmin"

gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="user:${USER_EMAIL}" \
    --role="roles/resourcemanager.projectIamAdmin"
```

## 🛠️ Service Account Setup

### Option 1: Create New Service Account

```bash
PROJECT_ID="portfolio-site-434710"
SA_NAME="github-actions"
SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

# Create service account
gcloud iam service-accounts create $SA_NAME \
    --display-name="GitHub Actions Service Account" \
    --description="Service account for GitHub Actions CI/CD" \
    --project=$PROJECT_ID

# Add all required roles (run the commands from the "Core Deployment Roles" section above)

# Create and download key
gcloud iam service-accounts keys create github-actions-key.json \
    --iam-account=$SA_EMAIL \
    --project=$PROJECT_ID

# Display key content (copy this to GitHub secrets)
cat github-actions-key.json
```

### Option 2: Use Existing Service Account

If you already have a service account, just add the required roles:

```bash
# Find your service account email
gcloud iam service-accounts list --project=portfolio-site-434710

# Add roles using the commands from the "Core Deployment Roles" section
```

## 🔍 Verification Commands

### Check Current Permissions

```bash
PROJECT_ID="portfolio-site-434710"
SA_EMAIL="github-actions@${PROJECT_ID}.iam.gserviceaccount.com"

# List all roles for the service account
gcloud projects get-iam-policy $PROJECT_ID \
    --flatten="bindings[].members" \
    --filter="bindings.members:serviceAccount:${SA_EMAIL}" \
    --format="table(bindings.role)"

# Check specific permissions
gcloud projects test-iam-permissions $PROJECT_ID \
    --permissions="artifactregistry.repositories.create,iam.serviceAccounts.create,run.services.create"
```

### Test API Access

```bash
# Test Artifact Registry access
gcloud artifacts repositories list --location=europe-west9 --project=portfolio-site-434710

# Test Cloud Run access
gcloud run services list --region=europe-west9 --project=portfolio-site-434710

# Test IAM access
gcloud iam service-accounts list --project=portfolio-site-434710
```

## 🚨 Troubleshooting

### Common Errors and Solutions

#### Error: "Permission denied on resource"
```
Error 403: Permission 'artifactregistry.repositories.create' denied
```

**Solution**: Add the missing role:
```bash
gcloud projects add-iam-policy-binding portfolio-site-434710 \
    --member="serviceAccount:YOUR_SA_EMAIL" \
    --role="roles/artifactregistry.admin"
```

#### Error: "API not enabled"
```
Error 403: Artifact Registry API has not been used in project
```

**Solution**: Enable the API:
```bash
gcloud services enable artifactregistry.googleapis.com --project=portfolio-site-434710
```

#### Error: "Service account does not exist"
```
Error 404: Service account github-actions@project.iam.gserviceaccount.com does not exist
```

**Solution**: Create the service account first:
```bash
gcloud iam service-accounts create github-actions --project=portfolio-site-434710
```

### Debugging Steps

1. **Check Authentication**:
   ```bash
   gcloud auth list
   gcloud config get-value project
   ```

2. **Verify API Status**:
   ```bash
   gcloud services list --enabled --project=portfolio-site-434710
   ```

3. **Check Service Account**:
   ```bash
   gcloud iam service-accounts describe YOUR_SA_EMAIL --project=portfolio-site-434710
   ```

4. **Test Permissions**:
   ```bash
   gcloud auth activate-service-account --key-file=path/to/key.json
   gcloud projects test-iam-permissions portfolio-site-434710 --permissions="artifactregistry.repositories.create"
   ```

## 🔒 Security Best Practices

### Principle of Least Privilege

Instead of using broad roles like `Editor` or `Owner`, use specific roles:

```bash
# ❌ Avoid this (too broad)
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/editor"

# ✅ Use this (specific permissions)
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/artifactregistry.admin"
```

### Key Rotation

Regularly rotate service account keys:

```bash
# List existing keys
gcloud iam service-accounts keys list --iam-account=$SA_EMAIL

# Create new key
gcloud iam service-accounts keys create new-key.json --iam-account=$SA_EMAIL

# Delete old key (after updating GitHub secrets)
gcloud iam service-accounts keys delete KEY_ID --iam-account=$SA_EMAIL
```

### Monitoring

Enable audit logging to monitor service account usage:

```bash
# Enable audit logs for IAM
gcloud logging sinks create iam-audit-sink \
    bigquery.googleapis.com/projects/$PROJECT_ID/datasets/audit_logs \
    --log-filter='protoPayload.serviceName="iam.googleapis.com"'
```

## 📋 Setup Checklist

- [ ] Enable required GCP APIs
- [ ] Create or identify service account for GitHub Actions
- [ ] Assign required IAM roles to service account
- [ ] Generate service account key
- [ ] Add `GCP_SA_KEY` to GitHub repository secrets
- [ ] Test Terraform locally (optional)
- [ ] Run GitHub Actions workflow
- [ ] Verify resources are created successfully

## 🔗 Related Documentation

- [GITHUB_SECRETS.md](GITHUB_SECRETS.md) - GitHub secrets setup
- [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment workflows
- [Google Cloud IAM Documentation](https://cloud.google.com/iam/docs)
- [Artifact Registry Documentation](https://cloud.google.com/artifact-registry/docs)
- [Cloud Run Documentation](https://cloud.google.com/run/docs)

---

**Need help?** If you encounter issues not covered here, check the GitHub Actions logs for specific error messages and refer to the troubleshooting section above.
