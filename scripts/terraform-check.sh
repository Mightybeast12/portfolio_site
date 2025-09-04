#!/bin/bash

# Terraform validation and safety check script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TERRAFORM_DIR="$(dirname "$SCRIPT_DIR")/terraform"

echo -e "${BLUE}=== Terraform Safety Check ===${NC}"

# Check if we're in the right directory
if [ ! -f "$TERRAFORM_DIR/terraform.tfvars" ]; then
    echo -e "${RED}Error: terraform.tfvars not found in $TERRAFORM_DIR${NC}"
    exit 1
fi

cd "$TERRAFORM_DIR"

# Check Terraform installation
if ! command -v terraform &> /dev/null; then
    echo -e "${RED}Error: Terraform is not installed${NC}"
    exit 1
fi

# Check if Terraform is initialized
if [ ! -d ".terraform" ]; then
    echo -e "${YELLOW}Terraform not initialized. Running terraform init...${NC}"
    terraform init
fi

# Validate Terraform configuration
echo -e "${YELLOW}Validating Terraform configuration...${NC}"
if terraform validate; then
    echo -e "${GREEN}✓ Terraform configuration is valid${NC}"
else
    echo -e "${RED}✗ Terraform configuration has errors${NC}"
    exit 1
fi

# Format check
echo -e "${YELLOW}Checking Terraform formatting...${NC}"
if terraform fmt -check; then
    echo -e "${GREEN}✓ Terraform files are properly formatted${NC}"
else
    echo -e "${YELLOW}⚠ Some files need formatting. Run 'terraform fmt' to fix${NC}"
    echo -e "${BLUE}Auto-formatting files...${NC}"
    terraform fmt
    echo -e "${GREEN}✓ Files formatted${NC}"
fi

# Security check - look for potential issues
echo -e "${YELLOW}Running security checks...${NC}"

# Check for hardcoded secrets
if grep -r "password\|secret\|key" --include="*.tf" --include="*.tfvars" . | grep -v "# " | grep -v "description"; then
    echo -e "${RED}⚠ Potential hardcoded secrets found. Please review.${NC}"
else
    echo -e "${GREEN}✓ No obvious hardcoded secrets found${NC}"
fi

# Check for public access
if grep -r "0.0.0.0/0\|*" --include="*.tf" .; then
    echo -e "${YELLOW}⚠ Found potential public access configurations. Please review.${NC}"
else
    echo -e "${GREEN}✓ No obvious public access configurations found${NC}"
fi

# Plan check
echo -e "${YELLOW}Running terraform plan...${NC}"
if terraform plan -out=tfplan -detailed-exitcode; then
    PLAN_EXIT_CODE=$?
    case $PLAN_EXIT_CODE in
        0)
            echo -e "${GREEN}✓ No changes needed${NC}"
            ;;
        2)
            echo -e "${YELLOW}⚠ Changes detected. Review the plan above.${NC}"
            echo -e "${BLUE}To apply changes, run: terraform apply tfplan${NC}"
            ;;
    esac
else
    echo -e "${RED}✗ Terraform plan failed${NC}"
    exit 1
fi

# Clean up
rm -f tfplan

echo -e "${GREEN}=== Safety check complete ===${NC}"

# Provide helpful commands
echo ""
echo -e "${BLUE}Helpful commands:${NC}"
echo -e "${BLUE}  terraform apply          ${NC}# Apply changes"
echo -e "${BLUE}  terraform destroy        ${NC}# Destroy infrastructure"
echo -e "${BLUE}  terraform show           ${NC}# Show current state"
echo -e "${BLUE}  terraform output         ${NC}# Show outputs"
