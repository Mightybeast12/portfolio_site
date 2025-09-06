# Create Artifact Registry repository for Docker images
resource "google_artifact_registry_repository" "portfolio_images" {
  provider      = google
  location      = var.region
  repository_id = var.repo_name
  description   = "Docker repository for portfolio site images"
  format        = "DOCKER"

  cleanup_policies {
    id     = "keep-recent-versions"
    action = "DELETE"
    condition {
      tag_state  = "TAGGED"
      older_than = "604800s"
    }
  }


  cleanup_policies {
    id     = "delete-untagged"
    action = "DELETE"

    condition {
      tag_state  = "UNTAGGED"
      older_than = "4800s"
    }
  }
}

# IAM binding to allow Cloud Run service account to pull images
resource "google_artifact_registry_repository_iam_binding" "repo_reader" {
  provider   = google
  location   = google_artifact_registry_repository.portfolio_images.location
  repository = google_artifact_registry_repository.portfolio_images.name
  role       = "roles/artifactregistry.reader"

  members = [
    "serviceAccount:${google_service_account.cloud_run_service.email}",
  ]
}

# Output the repository URL for use in build scripts
output "artifact_registry_url" {
  description = "The URL of the Artifact Registry repository"
  value       = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}"
}
