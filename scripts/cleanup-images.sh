#!/bin/bash

# Manual cleanup script for old images in Artifact Registry
# Use this if you need to manually clean up images beyond the automatic lifecycle policy

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
    IMAGE_NAME=$(grep '^image_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    REPO_NAME=$(grep '^repo_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    RETENTION_COUNT=$(grep '^image_retention_count' "$TERRAFORM_DIR/terraform.tfvars" | cut -d' ' -f3)
else
    echo -e "${RED}Error: terraform.tfvars not found in $TERRAFORM_DIR${NC}"
    exit 1
fi

REPO_BASE="${REGION}-docker.pkg.dev/${PROJECT_ID}/${REPO_NAME}"

# Function to list all images with details
list_all_images() {
    echo -e "${YELLOW}All images in repository:${NC}"
    gcloud artifacts docker images list "${REPO_BASE}/${IMAGE_NAME}" \
        --format="table(IMAGE,TAGS,CREATE_TIME,UPDATE_TIME)" \
        --sort-by="~CREATE_TIME"
}

# Function to list images that would be deleted
list_old_images() {
    echo -e "${YELLOW}Images older than ${RETENTION_COUNT} most recent:${NC}"

    # Get all images sorted by creation time (newest first)
    local images=$(gcloud artifacts docker images list "${REPO_BASE}/${IMAGE_NAME}" \
        --format="value(IMAGE)" \
        --sort-by="~CREATE_TIME")

    # Skip the first N images (most recent)
    local old_images=$(echo "$images" | tail -n +$((RETENTION_COUNT + 1)))

    if [ -z "$old_images" ]; then
        echo -e "${GREEN}No old images found to clean up${NC}"
        return 0
    fi

    echo "$old_images" | while read -r image; do
        if [ -n "$image" ]; then
            gcloud artifacts docker images describe "$image" \
                --format="table(IMAGE,TAGS,CREATE_TIME)"
        fi
    done
}

# Function to delete old images
delete_old_images() {
    echo -e "${YELLOW}Deleting images older than ${RETENTION_COUNT} most recent...${NC}"

    # Get all images sorted by creation time (newest first)
    local images=$(gcloud artifacts docker images list "${REPO_BASE}/${IMAGE_NAME}" \
        --format="value(IMAGE)" \
        --sort-by="~CREATE_TIME")

    # Skip the first N images (most recent)
    local old_images=$(echo "$images" | tail -n +$((RETENTION_COUNT + 1)))

    if [ -z "$old_images" ]; then
        echo -e "${GREEN}No old images found to clean up${NC}"
        return 0
    fi

    local count=0
    echo "$old_images" | while read -r image; do
        if [ -n "$image" ]; then
            echo -e "${BLUE}Deleting: $image${NC}"
            gcloud artifacts docker images delete "$image" --quiet
            ((count++))
        fi
    done

    echo -e "${GREEN}✓ Deleted $count old images${NC}"
}

# Function to delete specific image by tag
delete_by_tag() {
    local tag="$1"
    local target_image="${REPO_BASE}/${IMAGE_NAME}:${tag}"

    echo -e "${YELLOW}Deleting image: $target_image${NC}"

    # Check if image exists
    if gcloud artifacts docker images describe "$target_image" &>/dev/null; then
        gcloud artifacts docker images delete "$target_image" --quiet
        echo -e "${GREEN}✓ Deleted image: $target_image${NC}"
    else
        echo -e "${RED}Error: Image not found: $target_image${NC}"
        exit 1
    fi
}

# Function to delete untagged images
delete_untagged() {
    echo -e "${YELLOW}Deleting untagged images...${NC}"

    # List untagged images
    local untagged=$(gcloud artifacts docker images list "${REPO_BASE}/${IMAGE_NAME}" \
        --format="value(IMAGE)" \
        --filter="tags:*" --invert-match 2>/dev/null || true)

    if [ -z "$untagged" ]; then
        echo -e "${GREEN}No untagged images found${NC}"
        return 0
    fi

    local count=0
    echo "$untagged" | while read -r image; do
        if [ -n "$image" ]; then
            echo -e "${BLUE}Deleting untagged: $image${NC}"
            gcloud artifacts docker images delete "$image" --quiet
            ((count++))
        fi
    done

    echo -e "${GREEN}✓ Deleted $count untagged images${NC}"
}

# Main function
main() {
    case "${1:-}" in
        --list|-l)
            list_all_images
            ;;
        --list-old)
            list_old_images
            ;;
        --delete-old)
            echo -e "${RED}This will delete images older than the ${RETENTION_COUNT} most recent${NC}"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                delete_old_images
            else
                echo -e "${YELLOW}Cancelled${NC}"
            fi
            ;;
        --delete-untagged)
            echo -e "${RED}This will delete all untagged images${NC}"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                delete_untagged
            else
                echo -e "${YELLOW}Cancelled${NC}"
            fi
            ;;
        --delete-tag)
            if [ -z "$2" ]; then
                echo -e "${RED}Error: Tag required${NC}"
                echo "Usage: $0 --delete-tag <tag>"
                exit 1
            fi
            echo -e "${RED}This will delete the image with tag: $2${NC}"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                delete_by_tag "$2"
            else
                echo -e "${YELLOW}Cancelled${NC}"
            fi
            ;;
        --force-delete-old)
            delete_old_images
            ;;
        --help|-h|"")
            echo "Portfolio Site Image Cleanup Script"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --list, -l           List all images in repository"
            echo "  --list-old           List images that would be deleted (older than ${RETENTION_COUNT})"
            echo "  --delete-old         Delete images older than ${RETENTION_COUNT} most recent (with confirmation)"
            echo "  --delete-untagged    Delete all untagged images (with confirmation)"
            echo "  --delete-tag <tag>   Delete specific image by tag (with confirmation)"
            echo "  --force-delete-old   Delete old images without confirmation (USE WITH CAUTION)"
            echo "  --help, -h           Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --list                                      # List all images"
            echo "  $0 --list-old                                  # Show what would be cleaned"
            echo "  $0 --delete-old                                # Clean up old images"
            echo "  $0 --delete-tag v1.0.0-20240113-abc1234       # Delete specific version"
            echo ""
            echo "Note: The repository has automatic lifecycle policies that should handle"
            echo "cleanup automatically. Use these commands only if manual intervention is needed."
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
