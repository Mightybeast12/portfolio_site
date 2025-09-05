# Import existing GCS bucket
import {
  to = google_storage_bucket.terraform_state
  id = "portfolio-site-434710-terraform-state"
}

# Import existing Artifact Registry repository
import {
  to = google_artifact_registry_repository.portfolio_images
  id = "projects/portfolio-site-434710/locations/europe-west9/repositories/cv-portfolio-repo"
}

# Import existing Cloud Run service
import {
  to = google_cloud_run_v2_service.portfolio_site
  id = "projects/portfolio-site-434710/locations/europe-west9/services/firat-portfolio-site"
}

# Import existing service account
import {
  to = google_service_account.cloud_run_service
  id = "projects/portfolio-site-434710/serviceAccounts/firat-portfolio-site-service@portfolio-site-434710.iam.gserviceaccount.com"
}