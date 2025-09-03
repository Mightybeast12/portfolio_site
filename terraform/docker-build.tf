# Docker build and push using Terraform
resource "null_resource" "docker_build" {
  triggers = {
    # Rebuild when source files change
    dockerfile_hash = filemd5("${path.module}/../Dockerfile")
    source_hash     = sha256(join("", [for f in fileset("${path.module}/../src", "**") : filesha256("${path.module}/../src/${f}")]))
  }

  provisioner "local-exec" {
    command = <<-EOT
      cd ${path.module}/..
      gcloud auth configure-docker ${var.region}-docker.pkg.dev --quiet
      docker buildx build --platform linux/amd64 \
        -t ${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:latest \
        --push .
    EOT
  }

  depends_on = [
    google_artifact_registry_repository.portfolio_images
  ]
}