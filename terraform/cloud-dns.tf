# Cloudflare DNS configuration

# Get the zone ID for your domain
data "cloudflare_zone" "portfolio_zone" {
  name = var.custom_domain
}

# A record for root domain (proxied through Cloudflare)
resource "cloudflare_record" "portfolio_root" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "@"
  content = "192.0.2.1"  # Placeholder IP, Cloudflare will proxy to Cloud Run
  type    = "A"
  proxied = true
  ttl     = 1
}

# CNAME for www subdomain
resource "cloudflare_record" "portfolio_www" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "www"
  content = var.custom_domain
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