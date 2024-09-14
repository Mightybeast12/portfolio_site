#!/bin/bash

# Set variables
APP_NAME = "firat-portfolio-site" 
PROJECT_ID="portfolio-site-434710"
IMAGE_NAME="rust-image-cv-image"
VERSION="latest"
REGION="europe-west9"  
REPO_NAME="cv-portfolio-repo" # Name of the repository in Artifact repository
REPO="${REGION}-docker.pkg.dev/${PROJECT_ID}/${REPO_NAME}/${IMAGE_NAME}:${VERSION}"

# Authenticate Docker with Google Cloud Artifact Registry
gcloud auth configure-docker ${REGION}-docker.pkg.dev

# Build the Docker image
docker build -t ${REPO} .

# Push the Docker image to Google Artifact Registry
docker push ${REPO}

gcloud run deploy $APP_NAME --image "$REPO" --region $REGION \
 
