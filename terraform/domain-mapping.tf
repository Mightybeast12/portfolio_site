# Domain mapping for the portfolio site
resource "google_cloud_run_domain_mapping" "portfolio_domain" {
  location = var.region
  name     = "${var.subdomain}.${var.custom_domain}"

  metadata {
    namespace = var.project_id
  }

  spec {
    route_name = google_cloud_run_v2_service.portfolio_site.name
  }

  depends_on = [
    google_cloud_run_v2_service.portfolio_site
  ]
}
