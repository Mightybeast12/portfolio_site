# Domain mapping for portfolio subdomain only
resource "google_cloud_run_domain_mapping" "portfolio_subdomain" {
  count    = var.custom_domain != "pending.com" ? 1 : 0
  location = var.region
  name     = "${var.subdomain}.${var.custom_domain}"

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
  description = "Status of the portfolio subdomain mapping"
  value = var.custom_domain != "pending.com" ? {
    domain = "${var.subdomain}.${var.custom_domain}"
    status = length(google_cloud_run_domain_mapping.portfolio_subdomain) > 0 ? "Created" : "Not created"
  } : null
}
