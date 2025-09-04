#!/bin/bash

# Enhanced build and deploy script for portfolio site

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

# Generate version tag
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

# Check required tools
check_dependencies() {
    local missing_tools=()
    for tool in docker gcloud git; do
        if ! command -v $tool &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    if [ ${#missing_tools[@]} -ne 0 ]; then
        echo -e "${RED}Error: Missing required tools: ${missing_tools[*]}${NC}"
        echo -e "${YELLOW}Please install the missing tools and try again.${NC}"
        exit 1
    fi
}

# Check if Artifact Registry exists
check_artifact_registry() {
    echo -e "${YELLOW}Checking if Artifact Registry exists...${NC}"
    if ! gcloud artifacts repositories describe "$REPO_NAME" --location="$REGION" &> /dev/null; then
        echo -e "${RED}Error: Artifact Registry '$REPO_NAME' does not exist in region '$REGION'${NC}"
        echo -e "${YELLOW}Please run Terraform first to create the infrastructure:${NC}"
        echo -e "${BLUE}  cd terraform && terraform init && terraform apply${NC}"
        echo ""
        echo -e "${YELLOW}Or if this is your first deployment, make sure to:${NC}"
        echo -e "${BLUE}  1. Run 'terraform apply' to create the Artifact Registry${NC}"
        echo -e "${BLUE}  2. Then run this script to push the initial image${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Artifact Registry exists${NC}"
}

# Check Docker daemon
check_docker_daemon() {
    if ! docker info &> /dev/null; then
        echo -e "${RED}Error: Docker daemon is not running.${NC}"
        echo "Please start Docker Desktop or a Docker daemon (e.g., Colima) and try again."
        exit 1
    fi
}

# Authenticate GCP
authenticate_gcloud() {
    echo -e "${YELLOW}Authenticating with Google Cloud...${NC}"
    if ! gcloud auth list --filter=status:ACTIVE --format="value(account)" | grep -q .; then
        echo -e "${YELLOW}Please authenticate with Google Cloud:${NC}"
        gcloud auth login
    fi
    gcloud auth configure-docker "${REGION}-docker.pkg.dev" --quiet
    gcloud config set project "$PROJECT_ID"
    echo -e "${GREEN}✓ Authentication complete${NC}"
}

build_image() {
    echo -e "${YELLOW}Building Docker image for Cloud Run (linux/amd64)...${NC}"
    cd "$ROOT_DIR"

    # Build the image locally first
    docker build --platform linux/amd64 -t "$IMAGE_LATEST" .
    docker tag "$IMAGE_LATEST" "$IMAGE_VERSIONED"

    echo -e "${GREEN}✓ Image built successfully${NC}"
}

# Push Docker image
push_image() {
    echo -e "${YELLOW}Pushing Docker images to Artifact Registry...${NC}"

    # Use docker push directly (gcloud auth configure-docker already set up)
    docker push "$IMAGE_LATEST"
    docker push "$IMAGE_VERSIONED"

    echo -e "${GREEN}✓ Images pushed successfully${NC}"
}


# Get service status
get_service_status() {
    SERVICE_URL=$(gcloud run services describe "$APP_NAME" \
        --region="$REGION" \
        --format="value(status.url)")
    echo -e "${GREEN}✓ Service deployed successfully${NC}"
    echo -e "${BLUE}Service URL:${NC} $SERVICE_URL"

    if command -v curl &> /dev/null; then
        echo -e "${YELLOW}Testing service endpoint...${NC}"
        if curl -s -o /dev/null -w "%{http_code}" "$SERVICE_URL" | grep -q "200\|301\|302"; then
            echo -e "${GREEN}✓ Service is responding${NC}"
        else
            echo -e "${YELLOW}⚠ Service may still be starting up${NC}"
        fi
    fi
}

# Clean up local images
cleanup_local_images() {
    echo -e "${YELLOW}Cleaning up old local images...${NC}"
    if docker images -f "dangling=true" -q | grep -q .; then
        docker rmi $(docker images -f "dangling=true" -q) 2>/dev/null || true
    fi
    echo -e "${GREEN}✓ Local cleanup complete${NC}"
}

# Main execution
main() {
    echo -e "${BLUE}Starting build and deploy process...${NC}"

    check_dependencies
    check_docker_daemon
    authenticate_gcloud
    check_artifact_registry
    build_image
    push_image
    get_service_status
    cleanup_local_images

    echo ""
    echo -e "${GREEN}=== Deployment Complete ===${NC}"
    echo -e "${GREEN}✓ Image built and pushed: $IMAGE_VERSIONED${NC}"
    echo -e "${GREEN}✓ Cloud Run service updated: $APP_NAME${NC}"
    echo -e "${BLUE}Service URL: $SERVICE_URL${NC}"

    # Save deployment info
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
