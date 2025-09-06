# Domain mapping for Cloud Run service with Cloudflare DNS

# Domain mapping for custom domain (if supported in region)
resource "google_cloud_run_domain_mapping" "portfolio_domain" {
  count    = var.custom_domain != "pending.com" ? 1 : 0
  location = var.region
  name     = var.custom_domain

  metadata {
    namespace = var.project_id
  }

  spec {
    route_name = google_cloud_run_v2_service.portfolio_site.name
  }

  depends_on = [google_cloud_run_v2_service.portfolio_site]
}

# Output domain mapping status
output "domain_mapping_status" {
  description = "Status of the domain mapping"
  value = var.custom_domain != "pending.com" ? {
    domain = var.custom_domain
    status = length(google_cloud_run_domain_mapping.portfolio_domain) > 0 ? google_cloud_run_domain_mapping.portfolio_domain[0].status[0].conditions[0].status : "Not created"
  } : null
}
