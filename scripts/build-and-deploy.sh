#!/bin/bash

# Enhanced build and deploy script for portfolio site
# Replaces the original build.sh with improved versioning and error handling

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration (these will be read from terraform.tfvars or environment)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TERRAFORM_DIR="$ROOT_DIR/terraform"

# Source configuration from terraform.tfvars
if [ -f "$TERRAFORM_DIR/terraform.tfvars" ]; then
    # Extract values from terraform.tfvars (simple parsing)
    PROJECT_ID=$(grep '^project_id' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    REGION=$(grep '^region' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    APP_NAME=$(grep '^app_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    IMAGE_NAME=$(grep '^image_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
    REPO_NAME=$(grep '^repo_name' "$TERRAFORM_DIR/terraform.tfvars" | cut -d'"' -f2)
else
    echo -e "${RED}Error: terraform.tfvars not found in $TERRAFORM_DIR${NC}"
    exit 1
fi

# Generate version tag with timestamp and git commit
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
VERSION="v1.0.0-${TIMESTAMP}-${GIT_COMMIT}"

# Construct image URLs
REPO_BASE="${REGION}-docker.pkg.dev/${PROJECT_ID}/${REPO_NAME}"
IMAGE_LATEST="${REPO_BASE}/${IMAGE_NAME}:latest"
IMAGE_VERSIONED="${REPO_BASE}/${IMAGE_NAME}:${VERSION}"

echo -e "${BLUE}=== Portfolio Site Build and Deploy ===${NC}"
echo -e "${BLUE}Project ID:${NC} $PROJECT_ID"
echo -e "${BLUE}Region:${NC} $REGION"
echo -e "${BLUE}App Name:${NC} $APP_NAME"
echo -e "${BLUE}Version:${NC} $VERSION"
echo -e "${BLUE}Image (Latest):${NC} $IMAGE_LATEST"
echo -e "${BLUE}Image (Versioned):${NC} $IMAGE_VERSIONED"
echo ""

# Function to check if required tools are installed
check_dependencies() {
    local missing_tools=()

    if ! command -v docker &> /dev/null; then
        missing_tools+=("docker")
    fi

    if ! command -v gcloud &> /dev/null; then
        missing_tools+=("gcloud")
    fi

    if ! command -v git &> /dev/null; then
        missing_tools+=("git")
    fi

    if [ ${#missing_tools[@]} -ne 0 ]; then
        echo -e "${RED}Error: Missing required tools: ${missing_tools[*]}${NC}"
        exit 1
    fi
}

# Function to authenticate with Google Cloud
authenticate_gcloud() {
    echo -e "${YELLOW}Authenticating with Google Cloud...${NC}"

    # Check if already authenticated
    if ! gcloud auth list --filter=status:ACTIVE --format="value(account)" | grep -q .; then
        echo -e "${YELLOW}Please authenticate with Google Cloud:${NC}"
        gcloud auth login
    fi

    # Configure Docker authentication
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet

    echo -e "${GREEN}✓ Authentication complete${NC}"
}

# Function to build Docker image
build_image() {
    echo -e "${YELLOW}Building Docker image...${NC}"
    cd "$ROOT_DIR"

    # Build with both latest and versioned tags
    docker build -t "$IMAGE_LATEST" -t "$IMAGE_VERSIONED" .

    echo -e "${GREEN}✓ Image built successfully${NC}"
}

# Function to push Docker image
push_image() {
    echo -e "${YELLOW}Pushing Docker images to Artifact Registry...${NC}"

    # Push both tags
    docker push "$IMAGE_LATEST"
    docker push "$IMAGE_VERSIONED"

    echo -e "${GREEN}✓ Images pushed successfully${NC}"
    echo -e "${BLUE}Latest image:${NC} $IMAGE_LATEST"
    echo -e "${BLUE}Versioned image:${NC} $IMAGE_VERSIONED"
}

# Function to update Cloud Run service
update_cloud_run() {
    echo -e "${YELLOW}Updating Cloud Run service...${NC}"

    # Update the service with the new image
    gcloud run deploy "$APP_NAME" \
        --image "$IMAGE_LATEST" \
        --region "$REGION" \
        --platform managed \
        --quiet

    echo -e "${GREEN}✓ Cloud Run service updated${NC}"
}

# Function to get service status
get_service_status() {
    echo -e "${YELLOW}Getting service status...${NC}"

    # Get the service URL
    SERVICE_URL=$(gcloud run services describe "$APP_NAME" \
        --region="$REGION" \
        --format="value(status.url)")

    echo -e "${GREEN}✓ Service deployed successfully${NC}"
    echo -e "${BLUE}Service URL:${NC} $SERVICE_URL"

    # Test if service is responding
    if command -v curl &> /dev/null; then
        echo -e "${YELLOW}Testing service endpoint...${NC}"
        if curl -s -o /dev/null -w "%{http_code}" "$SERVICE_URL" | grep -q "200\|301\|302"; then
            echo -e "${GREEN}✓ Service is responding${NC}"
        else
            echo -e "${YELLOW}⚠ Service may still be starting up${NC}"
        fi
    fi
}

# Function to clean up old local images
cleanup_local_images() {
    echo -e "${YELLOW}Cleaning up old local images...${NC}"

    # Remove dangling images
    if docker images -f "dangling=true" -q | grep -q .; then
        docker rmi $(docker images -f "dangling=true" -q) 2>/dev/null || true
    fi

    echo -e "${GREEN}✓ Local cleanup complete${NC}"
}

# Main execution
main() {
    echo -e "${BLUE}Starting build and deploy process...${NC}"

    check_dependencies
    authenticate_gcloud
    build_image
    push_image
    update_cloud_run
    get_service_status
    cleanup_local_images

    echo ""
    echo -e "${GREEN}=== Deployment Complete ===${NC}"
    echo -e "${GREEN}✓ Image built and pushed: $IMAGE_VERSIONED${NC}"
    echo -e "${GREEN}✓ Cloud Run service updated: $APP_NAME${NC}"
    echo -e "${BLUE}Service URL: $SERVICE_URL${NC}"

    # Save deployment info for rollback script
    echo "$VERSION" > "$SCRIPT_DIR/.last_version"
    echo "$IMAGE_VERSIONED" > "$SCRIPT_DIR/.last_image"
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [options]"
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --version, -v  Show version information"
        echo ""
        echo "This script builds and deploys the portfolio site to Google Cloud Run."
        exit 0
        ;;
    --version|-v)
        echo "Portfolio Site Build Script"
        echo "Version: $VERSION"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        echo -e "${RED}Error: Unknown option $1${NC}"
        echo "Use --help for usage information"
        exit 1
        ;;
esac
