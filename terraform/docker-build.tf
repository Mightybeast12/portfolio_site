# Smart Docker build that checks for existing images
locals {
  # Image URL for checking existence
  image_url = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:latest"

  # Command to check if image exists
  image_check_cmd = "gcloud artifacts docker images list ${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name} --filter='image:${var.image_name}' --format='value(image)' --limit=1 2>/dev/null | grep -q '${var.image_name}' || echo 'no-image'"
}

# Check if Docker image exists in registry
resource "null_resource" "check_image_exists" {
  count = var.auto_build ? 1 : 0

  triggers = {
    always_check = timestamp()
  }

  provisioner "local-exec" {
    command = <<-EOT
      echo "Checking if image exists in registry..."
      if ${local.image_check_cmd} | grep -q "no-image"; then
        echo "IMAGE_EXISTS=false" > /tmp/terraform_image_check
      else
        echo "IMAGE_EXISTS=true" > /tmp/terraform_image_check
        echo "✓ Image already exists in registry"
      fi
    EOT
  }

  depends_on = [
    google_artifact_registry_repository.portfolio_images
  ]
}

# Build Docker image only if it doesn't exist and auto_build is enabled
resource "null_resource" "docker_build" {
  count = var.auto_build && !var.ci_environment ? 1 : 0

  triggers = {
    # Rebuild when source files change or when we detect no image exists
    dockerfile_hash = filemd5("${path.module}/../Dockerfile")
    source_hash     = sha256(join("", [for f in fileset("${path.module}/../src", "**") : filesha256("${path.module}/../src/${f}")]))
    registry_ready  = google_artifact_registry_repository.portfolio_images.id
  }

  provisioner "local-exec" {
    command = <<-EOT
      echo "🔍 Checking if initial Docker image build is needed..."

      # Check if image exists in registry
      if ${local.image_check_cmd} | grep -q "no-image"; then
        echo "📦 No image found in registry. Building initial image..."
        echo "🚀 Running build script: ${path.module}/../scripts/build-and-deploy.sh"
        ${path.module}/../scripts/build-and-deploy.sh
        echo "✅ Initial image build completed"
      else
        echo "✅ Image already exists in registry. Skipping build."
        echo "💡 To rebuild manually, run: ./scripts/build-and-deploy.sh"
      fi
    EOT
  }

  depends_on = [
    google_artifact_registry_repository.portfolio_images,
    null_resource.check_image_exists
  ]
}

# Output helpful information about the build process
output "docker_build_info" {
  description = "Information about Docker build process"
  value = {
    auto_build_enabled = var.auto_build
    ci_environment     = var.ci_environment
    image_url          = local.image_url
    build_script_path  = "${path.module}/../scripts/build-and-deploy.sh"
  }
}
