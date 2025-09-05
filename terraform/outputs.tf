# Output values for important resources

output "project_id" {
  description = "The GCP project ID"
  value       = var.project_id
}

output "region" {
  description = "The GCP region"
  value       = var.region
}

# Custom domain outputs (using Cloudflare DNS)
output "domain_mapping_url" {
  description = "The custom domain URL"
  value       = "https://${var.custom_domain}"
}

output "cloud_run_service_url" {
  description = "Direct URL of the Cloud Run service (for testing)"
  value       = google_cloud_run_v2_service.portfolio_site.uri
}

output "custom_domain_url" {
  description = "Custom domain URL (configure DNS to point here)"
  value       = "https://${var.custom_domain}"
}

output "subdomain_url" {
  description = "Subdomain URL (configure DNS to point here)"
  value       = "https://${var.subdomain}.${var.custom_domain}"
}

output "artifact_registry_repository" {
  description = "Full path to the Artifact Registry repository"
  value       = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}"
}

output "docker_image_url" {
  description = "Current Docker image URL"
  value       = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:latest"
}

output "service_account_email" {
  description = "Email of the Cloud Run service account"
  value       = google_service_account.cloud_run_service.email
}

# DNS Configuration Instructions
output "dns_configuration" {
  description = "DNS records needed for custom domain (direct routing to Cloud Run)"
  value = {
    message      = "Add this CNAME record to your DNS provider (Cloudflare)"
    type         = "CNAME"
    host         = var.subdomain
    target       = replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")
    full_domain  = "${var.subdomain}.${var.custom_domain}"
    instructions = "Create CNAME record: ${var.subdomain}.${var.custom_domain} -> ${replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")}"
  }
}

# Deployment information
output "deployment_info" {
  description = "Important information for deployment"
  value = {
    build_command       = "cd ${path.root}/.. && ./scripts/build-and-deploy.sh"
    terraform_workspace = path.root
    current_image       = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}:latest"
  }
}
