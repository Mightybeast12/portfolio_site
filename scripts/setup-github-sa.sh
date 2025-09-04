#!/bin/bash

set -e

PROJECT_ID=$(gcloud config get-value project)
SA_NAME="github-actions"
SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"
KEY_FILE="github-actions-key.json"

# Parse flags
if [[ "$1" == "--delete" ]]; then
    echo "🗑️ Deleting service account: $SA_EMAIL"
    gcloud iam service-accounts delete $SA_EMAIL --project=$PROJECT_ID --quiet
    echo "✅ Service account deleted!"
    exit 0
fi

if [[ "$1" == "--recreate" ]]; then
    echo "🔄 Recreating service account: $SA_EMAIL"
    gcloud iam service-accounts delete $SA_EMAIL --project=$PROJECT_ID --quiet 2>/dev/null || true
    sleep 2
fi

echo "🔧 Setting up GitHub Actions service account..."

# Create service account
echo "Creating service account: $SA_EMAIL"
if [[ "$1" == "--recreate" ]] || ! gcloud iam service-accounts describe $SA_EMAIL --project=$PROJECT_ID &>/dev/null; then
    gcloud iam service-accounts create $SA_NAME \
        --display-name="GitHub Actions" \
        --description="Service account for GitHub Actions CI/CD" \
        --project=$PROJECT_ID
    echo "Waiting for service account to propagate..."
    sleep 10
else
    echo "Service account already exists, continuing..."
fi

# Set expiration (30 days from now)
EXPIRY=$(date -u -v+30d +"%Y-%m-%dT%H:%M:%SZ")
CONDITION="request.time < timestamp(\"$EXPIRY\")"
TITLE="github-actions-temp-access"

# Grant permissions with timeout
echo "Granting Artifact Registry writer permissions (expires: $EXPIRY)..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/artifactregistry.writer" \
    --condition="expression=$CONDITION,title=$TITLE"

echo "Granting Cloud Run admin permissions (expires: $EXPIRY)..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/run.admin" \
    --condition="expression=$CONDITION,title=$TITLE"

echo "Granting service account token creator permissions (expires: $EXPIRY)..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:${SA_EMAIL}" \
    --role="roles/iam.serviceAccountTokenCreator" \
    --condition="expression=$CONDITION,title=$TITLE"

# Generate key
echo "Generating service account key..."
gcloud iam service-accounts keys create $KEY_FILE \
    --iam-account="${SA_EMAIL}" \
    --project=$PROJECT_ID

echo "✅ Service account setup complete!"
echo "📄 Key file created: $KEY_FILE"
echo ""
echo "📋 Next steps:"
echo "1. Copy the contents of $KEY_FILE"
echo "2. Add it to GitHub repository secrets as GCP_SA_KEY"
echo "3. Delete the key file: rm $KEY_FILE"
echo ""
echo "Service account email: $SA_EMAIL"
echo ""
echo "Usage:"
echo "  ./scripts/setup-github-sa.sh          # Create/update service account"
echo "  ./scripts/setup-github-sa.sh --recreate # Delete and recreate"
echo "  ./scripts/setup-github-sa.sh --delete   # Delete service account"