# Cloud Run service for the portfolio site
resource "google_cloud_run_v2_service" "portfolio_site" {
  name     = var.app_name
  location = var.region
  ingress  = "INGRESS_TRAFFIC_ALL"

  template {
    service_account = google_service_account.cloud_run_service.email

    scaling {
      max_instance_count = var.max_instances
    }

    containers {
      image = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:${var.image_tag}"

      ports {
        container_port = 8080
      }

      resources {
        limits = {
          cpu    = var.cpu_limit
          memory = var.memory_limit
        }
      }

      # Environment variables (add more as needed)
      # env {
      #   name  = "PORT"
      #   value = "8080"
      # }

      env {
        name  = "ENVIRONMENT"
        value = "production"
      }
    }
  }

  traffic {
    percent = 100
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    # Explicitly no tag
  }


  depends_on = [
    google_artifact_registry_repository.portfolio_images,
    google_service_account.cloud_run_service,
    null_resource.docker_build
  ]
}

# Output the Cloud Run service URL
output "cloud_run_url" {
  description = "URL of the Cloud Run service"
  value       = google_cloud_run_v2_service.portfolio_site.uri
}

# Output the service name for use in load balancer
output "cloud_run_service_name" {
  description = "Name of the Cloud Run service"
  value       = google_cloud_run_v2_service.portfolio_site.name
}
