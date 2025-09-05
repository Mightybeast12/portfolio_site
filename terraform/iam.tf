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

# GitHub Actions service account permissions
# Note: The service account key should be stored in GitHub secrets as GCP_SA_KEY

# Allow GitHub Actions to push to Artifact Registry
resource "google_project_iam_member" "github_artifact_registry_writer" {
  project = var.project_id
  role    = "roles/artifactregistry.writer"
  member  = "serviceAccount:${var.github_actions_sa_email}"
}

# Allow GitHub Actions to manage Cloud Run services
resource "google_project_iam_member" "github_cloud_run_admin" {
  project = var.project_id
  role    = "roles/run.admin"
  member  = "serviceAccount:${var.github_actions_sa_email}"
}

# Allow GitHub Actions to configure Docker authentication
resource "google_project_iam_member" "github_service_account_token_creator" {
  project = var.project_id
  role    = "roles/iam.serviceAccountTokenCreator"
  member  = "serviceAccount:${var.github_actions_sa_email}"
}

# Allow GitHub Actions to access GCS bucket for Terraform state
resource "google_storage_bucket_iam_member" "github_terraform_state_admin" {
  bucket = google_storage_bucket.terraform_state.name
  role   = "roles/storage.admin"
  member = "serviceAccount:${var.github_actions_sa_email}"
}

# Allow GitHub Actions to enable APIs
resource "google_project_iam_member" "github_service_usage_admin" {
  project = var.project_id
  role    = "roles/serviceusage.serviceUsageAdmin"
  member  = "serviceAccount:${var.github_actions_sa_email}"
}

# Allow GitHub Actions to act as the Cloud Run service account
resource "google_project_iam_member" "github_service_account_user" {
  project = var.project_id
  role    = "roles/iam.serviceAccountUser"
  member  = "serviceAccount:${var.github_actions_sa_email}"
}
