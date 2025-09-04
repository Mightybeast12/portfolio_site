# Cloudflare provider
terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

# Get the zone ID for your domain
data "cloudflare_zone" "portfolio_zone" {
  name = var.custom_domain
}

# CNAME record pointing to Cloud Run
resource "cloudflare_record" "portfolio_root" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "@"
  value   = replace(google_cloud_run_v2_service.portfolio_site.uri, "https://", "")
  type    = "CNAME"
  proxied = true
  ttl     = 1
}

# CNAME for www subdomain
resource "cloudflare_record" "portfolio_www" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "www"
  value   = var.custom_domain
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