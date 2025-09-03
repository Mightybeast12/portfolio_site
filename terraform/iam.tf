# Service account for Cloud Run service
resource "google_service_account" "cloud_run_service" {
  account_id   = "${var.app_name}-service"
  display_name = "Cloud Run Service Account for ${var.app_name}"
  description  = "Service account used by Cloud Run service"
}

# Grant the Cloud Run service account the necessary permissions
resource "google_project_iam_member" "cloud_run_invoker" {
  project = var.project_id
  role    = "roles/run.invoker"
  member  = "serviceAccount:${google_service_account.cloud_run_service.email}"
}

# Allow all users to invoke the Cloud Run service (can be restricted later)
resource "google_cloud_run_service_iam_member" "public_access" {
  service  = google_cloud_run_v2_service.portfolio_site.name
  location = google_cloud_run_v2_service.portfolio_site.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# Service account for load balancer (if needed for advanced configurations)
resource "google_service_account" "load_balancer_service" {
  account_id   = "${var.app_name}-lb"
  display_name = "Load Balancer Service Account for ${var.app_name}"
  description  = "Service account used by Load Balancer components"
}
