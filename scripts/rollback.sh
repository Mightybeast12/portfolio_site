#!/bin/bash

# Rollback script for portfolio site
# Allows rolling back to a previous version

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TERRAFORM_DIR="$ROOT_DIR/terraform"

# Source configuration from terraform.tfvars
if [ -f "$TERRAFORM_DIR/terraform.tfvars" ]; then
    PROJECT_ID=$(grep '^project_id' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    REGION=$(grep '^region' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    APP_NAME=$(grep '^app_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    IMAGE_NAME=$(grep '^image_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    REPO_NAME=$(grep '^repo_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
else
    echo -e "${RED}Error: terraform.tfvars not found in $TERRAFORM_DIR${NC}"
    exit 1
fi

REPO_BASE="${REGION}-docker.pkg.dev/${PROJECT_ID}/${REPO_NAME}"

# Function to list available images
list_images() {
    echo -e "${YELLOW}Available images in repository:${NC}"
    gcloud artifacts docker images list "${REPO_BASE}/${IMAGE_NAME}" \
        --format="table(IMAGE,TAGS,CREATE_TIME)" \
        --sort-by="~CREATE_TIME" \
        --limit=10
}

# Function to rollback to specific version
rollback_to_version() {
    local target_image="$1"

    echo -e "${YELLOW}Rolling back to: $target_image${NC}"

    # Update Cloud Run service with the target image
    gcloud run deploy "$APP_NAME" \
        --image "$target_image" \
        --region "$REGION" \
        --platform managed \
        --quiet

    echo -e "${GREEN}✓ Rollback complete${NC}"

    # Get service status
    SERVICE_URL=$(gcloud run services describe "$APP_NAME" \
        --region="$REGION" \
        --format="value(status.url)")

    echo -e "${BLUE}Service URL:${NC} $SERVICE_URL"
}

# Function to rollback to previous version
rollback_to_previous() {
    if [ -f "$SCRIPT_DIR/.last_image" ]; then
        local last_image=$(cat "$SCRIPT_DIR/.last_image")
        echo -e "${BLUE}Rolling back to previous version: $last_image${NC}"
        rollback_to_version "$last_image"
    else
        echo -e "${RED}Error: No previous version information found${NC}"
        echo -e "${YELLOW}Use --list to see available versions${NC}"
        exit 1
    fi
}

# Function to rollback to latest
rollback_to_latest() {
    local latest_image="${REPO_BASE}/${IMAGE_NAME}:latest"
    echo -e "${BLUE}Rolling back to latest version: $latest_image${NC}"
    rollback_to_version "$latest_image"
}

# Main function
main() {
    case "${1:-}" in
        --list|-l)
            list_images
            ;;
        --previous|-p)
            rollback_to_previous
            ;;
        --latest)
            rollback_to_latest
            ;;
        --version|-v)
            if [ -z "$2" ]; then
                echo -e "${RED}Error: Version tag required${NC}"
                echo "Usage: $0 --version <tag>"
                exit 1
            fi
            local target_image="${REPO_BASE}/${IMAGE_NAME}:$2"
            rollback_to_version "$target_image"
            ;;
        --help|-h|"")
            echo "Portfolio Site Rollback Script"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --list, -l           List available image versions"
            echo "  --previous, -p       Rollback to previous version"
            echo "  --latest             Rollback to latest version"
            echo "  --version <tag>      Rollback to specific version tag"
            echo "  --help, -h           Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --list                              # List available versions"
            echo "  $0 --previous                          # Rollback to previous"
            echo "  $0 --version v1.0.0-20240113-abc1234  # Rollback to specific version"
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
}

# Check if required tools are available
if ! command -v gcloud &> /dev/null; then
    echo -e "${RED}Error: gcloud CLI not found${NC}"
    exit 1
fi

main "$@"
