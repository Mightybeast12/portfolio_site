# Configure the Google Cloud Provider
provider "google" {
  project = var.project_id
  region  = var.region
}

# Configure the Google Cloud Provider Beta (for some newer features)
provider "google-beta" {
  project = var.project_id
  region  = var.region
}
