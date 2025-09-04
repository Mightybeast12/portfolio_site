#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Function to display error messages and exit
error_exit() {
  echo -e "${RED}ERROR: $1${NC}" >&2
  exit 1
}

# Function to display success messages
success_msg() {
  echo -e "${GREEN}$1${NC}"
}

# Function to display warning messages
warning_msg() {
  echo -e "${YELLOW}$1${NC}"
}

# Configuration
PROJECT_ID=$(gcloud config get-value project)
REGION="US"
WORKSPACES=("dev" "stg" "prod")

# Check if backend.tf exists and extract values
if [ -f "$(pwd)/backend.tf" ]; then
  BASE_BUCKET_NAME=$(grep -o 'bucket[[:space:]]*=[[:space:]]*"[^"]*"' "$(pwd)/backend.tf" | cut -d'"' -f2)
  
  if [ -z "$BASE_BUCKET_NAME" ]; then
    BASE_BUCKET_NAME="${PROJECT_ID}-terraform-state"
  fi
else
  BASE_BUCKET_NAME="${PROJECT_ID}-terraform-state"
fi

# Check for GCP credentials
check_gcp_credentials() {
  echo "Checking GCP credentials..."
  if ! gcloud auth list --filter=status:ACTIVE --format="value(account)" | head -n1 &>/dev/null; then
    error_exit "GCP credentials not found. Please run 'gcloud auth login' to authenticate."
  fi
  success_msg "GCP credentials verified."
}

# Create GCS bucket for state storage
create_gcs_bucket() {
  echo "Setting up GCS bucket for state storage..."

  if gsutil ls -b gs://$BASE_BUCKET_NAME &>/dev/null; then
    warning_msg "GCS bucket '$BASE_BUCKET_NAME' already exists, skipping creation."
  else
    echo "Creating GCS bucket '$BASE_BUCKET_NAME'..."
    gsutil mb -p $PROJECT_ID -c STANDARD -l $REGION gs://$BASE_BUCKET_NAME || error_exit "Failed to create GCS bucket"
    success_msg "GCS bucket created successfully."
  fi

  echo "Configuring GCS bucket..."

  # Enable versioning
  echo "Enabling versioning..."
  gsutil versioning set on gs://$BASE_BUCKET_NAME || error_exit "Failed to enable versioning"

  # Set lifecycle policy
  echo "Setting lifecycle policy..."
  cat > lifecycle.json << EOF
{
  "lifecycle": {
    "rule": [
      {
        "action": {"type": "Delete"},
        "condition": {"age": 30, "isLive": false}
      }
    ]
  }
}
EOF
  gsutil lifecycle set lifecycle.json gs://$BASE_BUCKET_NAME || error_exit "Failed to set lifecycle policy"
  rm lifecycle.json

  success_msg "GCS bucket configured successfully."

  # Grant GitHub Actions service account access to bucket
  echo "Granting GitHub Actions access to bucket..."
  gsutil iam ch serviceAccount:github-actions@${PROJECT_ID}.iam.gserviceaccount.com:roles/storage.admin gs://$BASE_BUCKET_NAME || warning_msg "Failed to grant bucket access - may need to be done via Terraform"
}

# Create workspace folders
setup_workspaces() {
  echo "Setting up workspace folders in GCS bucket..."

  for workspace in "${WORKSPACES[@]}"; do
    echo "Setting up folder for workspace: $workspace"
    
    # Create empty state file
    echo '{}' > empty_state.json
    
    # Upload empty state file
    gsutil cp empty_state.json gs://$BASE_BUCKET_NAME/env/$workspace/terraform.tfstate || error_exit "Failed to create workspace folder for $workspace"
    
    rm empty_state.json
  done

  success_msg "Workspace folders created successfully."

  echo "📋 Important: Ensure GitHub Actions service account has bucket access:"
  echo "  gsutil iam ch serviceAccount:github-actions@${PROJECT_ID}.iam.gserviceaccount.com:roles/storage.admin gs://$BASE_BUCKET_NAME"
}

# Migrate state for a specific workspace
migrate_state() {
  local workspace=$1
  local state_file="terraform.tfstate"

  echo "Migrating state for workspace: $workspace"

  # Check if local state exists
  if [ ! -f "$state_file" ]; then
    warning_msg "Local state file '$state_file' not found. Skipping migration."
    return
  fi

  # Backup local state
  echo "Creating backup of local state..."
  cp "$state_file" "${workspace}-state-backup.tfstate" || error_exit "Failed to backup state file"

  # Upload state to GCS
  echo "Uploading state to GCS..."
  gsutil cp "${workspace}-state-backup.tfstate" gs://$BASE_BUCKET_NAME/env/$workspace/terraform.tfstate || error_exit "Failed to upload state file"

  # Verify upload
  if gsutil ls gs://$BASE_BUCKET_NAME/env/$workspace/terraform.tfstate &>/dev/null; then
    success_msg "State file for workspace '$workspace' successfully migrated to GCS."
  else
    error_exit "Failed to verify state file upload for workspace '$workspace'."
  fi
}

# Display backend configuration
show_backend_config() {
  echo ""
  success_msg "Terraform backend infrastructure setup complete!"
  echo ""
  echo "Your versions.tf configuration should look like this:"
  echo "----------------"
  echo 'terraform {'
  echo '  backend "gcs" {'
  echo '    bucket = "'$BASE_BUCKET_NAME'"'
  echo '    prefix = "terraform/state"'
  echo '  }'
  echo '}'
  echo "----------------"
}

# Update versions.tf file with backend configuration
update_backend_file() {
  local versions_file="$(pwd)/terraform/versions.tf"

  echo "Updating versions.tf file with backend configuration..."

  # Add backend block to existing versions.tf
  if [ -f "$versions_file" ]; then
    # Check if backend already exists
    if grep -q "backend \"gcs\"" "$versions_file"; then
      warning_msg "Backend configuration already exists in versions.tf"
      return
    fi
    
    # Insert backend after terraform block opening
    sed -i.bak '/terraform {/a\
  backend "gcs" {\
    bucket = "'$BASE_BUCKET_NAME'"\
    prefix = "terraform/state"\
  }\
' "$versions_file"
    
    success_msg "Backend configuration added to versions.tf"
  else
    error_exit "versions.tf file not found"
  fi
}

# Destroy backend infrastructure
destroy_backend() {
  local bucket_name=$1

  echo "Destroying backend infrastructure..."

  # Check if bucket exists
  if gsutil ls -b gs://$bucket_name &>/dev/null; then
    echo "Emptying GCS bucket '$bucket_name'..."
    gsutil -m rm -r gs://$bucket_name/** || true

    echo "Deleting GCS bucket '$bucket_name'..."
    gsutil rb gs://$bucket_name
    success_msg "GCS bucket '$bucket_name' deleted."
  else
    warning_msg "GCS bucket '$bucket_name' does not exist."
  fi

  success_msg "Backend infrastructure destroyed successfully."
}

# Read backend configuration from versions.tf file
read_backend_config() {
  local versions_file="$(pwd)/terraform/versions.tf"
  local bucket_name=""

  if [ ! -f "$versions_file" ]; then
    error_exit "versions.tf file not found. Cannot determine backend configuration."
  fi

  # Extract bucket name
  bucket_name=$(grep -o 'bucket[[:space:]]*=[[:space:]]*"[^"]*"' "$versions_file" | cut -d'"' -f2)

  if [ -z "$bucket_name" ]; then
    error_exit "Could not extract bucket name from versions.tf"
  fi

  echo "Found backend configuration:"
  echo "  Bucket: $bucket_name"

  BACKEND_BUCKET_NAME=$bucket_name
}

# Main function
main() {
  local command=${1:-"setup"}
  local workspace=${2:-"dev"}

  case $command in
    setup)
      check_gcp_credentials
      create_gcs_bucket
      setup_workspaces
      show_backend_config
      update_backend_file
      ;;
    migrate)
      check_gcp_credentials
      migrate_state "$workspace"
      ;;
    destroy)
      check_gcp_credentials
      read_backend_config
      echo "WARNING: This will delete all Terraform state data in the GCS bucket."
      echo "Are you sure you want to destroy the backend infrastructure? (y/N)"
      read -r confirm
      if [[ "$confirm" =~ ^[Yy]$ ]]; then
        destroy_backend "$BACKEND_BUCKET_NAME"
      else
        echo "Destroy operation cancelled."
      fi
      ;;
    *)
      echo "Usage: $0 [setup|migrate|destroy] [workspace]"
      echo ""
      echo "Commands:"
      echo "  setup   - Set up GCS bucket for Terraform backend"
      echo "  migrate - Migrate current state to GCS"
      echo "  destroy - Destroy backend infrastructure (GCS bucket)"
      echo ""
      echo "Examples:"
      echo "  $0 setup          - Set up backend infrastructure"
      echo "  $0 migrate        - Migrate current state to GCS"
      echo "  $0 destroy        - Destroy backend infrastructure"
      exit 1
      ;;
  esac
}

# Run main function with all arguments
main "$@"