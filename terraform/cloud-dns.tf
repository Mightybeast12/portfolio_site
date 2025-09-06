# Cloudflare DNS configuration

# Get the zone ID for your domain
data "cloudflare_zone" "portfolio_zone" {
  name = var.custom_domain
}

# Root domain record managed manually in Cloudflare dashboard
# Point @ record to: replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")
# Type: CNAME, Proxied: Yes

# CNAME for www subdomain
resource "cloudflare_record" "portfolio_www" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "www"
  content = var.custom_domain
  type    = "CNAME"
  proxied = true
  ttl     = 1
}

# CNAME for portfolio subdomain pointing to Cloud Run
resource "cloudflare_record" "portfolio_subdomain" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = var.subdomain
  content = replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")
  type    = "CNAME"
  proxied = true
  ttl     = 1
}

# SSL settings
resource "cloudflare_zone_settings_override" "portfolio_ssl" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  settings {
    ssl                      = "full"
    always_use_https         = "on"
    automatic_https_rewrites = "on"
    min_tls_version          = "1.2"
  }
}

# Output Cloudflare nameservers
output "cloudflare_nameservers" {
  description = "Cloudflare nameservers to configure in Porkbun"
  value       = data.cloudflare_zone.portfolio_zone.name_servers
}

# Output Cloud Run URL for manual DNS configuration
output "cloud_run_dns_target" {
  description = "Cloud Run URL to use as CNAME target in Cloudflare (remove https://)"
  value       = replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")
}