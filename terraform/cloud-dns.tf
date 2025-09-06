# Cloudflare DNS configuration

# Get the zone ID for your domain
data "cloudflare_zone" "portfolio_zone" {
  name = var.custom_domain
}

# Root domain record for domain mapping
resource "cloudflare_record" "portfolio_root" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = "@"
  content = "ghs.googlehosted.com"
  type    = "CNAME"
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

# CNAME for portfolio subdomain pointing to Google hosted service for domain mapping
resource "cloudflare_record" "portfolio_subdomain" {
  zone_id = data.cloudflare_zone.portfolio_zone.id
  name    = var.subdomain
  content = "ghs.googlehosted.com"
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

# Output domain mapping instructions
output "domain_mapping_instructions" {
  description = "Domain mapping setup instructions"
  value = {
    root_domain = "Point @ to ghs.googlehosted.com (CNAME, not proxied)"
    subdomain   = "Point ${var.subdomain} to ghs.googlehosted.com (CNAME, not proxied)"
    note        = "Domain mapping requires ghs.googlehosted.com target and proxied=false"
  }
}
