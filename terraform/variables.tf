variable "project_id" {
  description = "The GCP project ID"
  type        = string
  default     = "placeholder"
}

variable "region" {
  description = "The GCP region for resources"
  type        = string
  default     = "europe-west9"
}

variable "app_name" {
  description = "Name of the application"
  type        = string
  default     = "firat-portfolio-site"
}

variable "image_name" {
  description = "Docker image name"
  type        = string
  default     = "rust-image-cv-image"
}

variable "repo_name" {
  description = "Artifact Registry repository name"
  type        = string
  default     = "cv-portfolio-repo"
}

variable "custom_domain" {
  description = "Custom domain for the portfolio site (placeholder)"
  type        = string
  default     = "pending.com"
}

variable "subdomain" {
  description = "Subdomain for the portfolio site"
  type        = string
  default     = "portfolio"
}


variable "max_instances" {
  description = "Maximum number of Cloud Run instances"
  type        = number
  default     = 2
}

variable "cpu_limit" {
  description = "CPU limit for Cloud Run service"
  type        = string
  default     = "1000m"
}

variable "memory_limit" {
  description = "Memory limit for Cloud Run service"
  type        = string
  default     = "512Mi"
}

variable "image_retention_count" {
  description = "Number of images to retain in Artifact Registry"
  type        = number
  default     = 2
}

variable "ci_environment" {
  description = "Whether running in CI/CD environment"
  type        = bool
  default     = false
}

variable "auto_build" {
  description = "Whether to automatically build Docker images locally"
  type        = bool
  default     = true
}

variable "cloudflare_api_token" {
  description = "Cloudflare API token for DNS management"
  type        = string
  sensitive   = true
  default     = ""
}
