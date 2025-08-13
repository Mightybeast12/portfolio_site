# Multi-stage Dockerfile optimized for Rust/WASM builds
# Optimizations: build caching, security, smaller images, faster builds

# Build arguments for flexibility
ARG RUST_VERSION=slim-bookworm
ARG TRUNK_VERSION=0.18.8

# Stage 1: Cache trunk installation (rarely changes)
FROM rust:${RUST_VERSION} AS trunk-installer
ARG TRUNK_VERSION
RUN cargo install trunk --version ${TRUNK_VERSION} --locked

# Stage 2: Build dependencies (cached when Cargo.toml unchanged)
FROM rust:${RUST_VERSION} AS dependencies
WORKDIR /app
# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
# Create dummy main to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Build dependencies in release mode
RUN cargo build --release
# Clean up dummy source
RUN rm -rf src

# Stage 3: Build application
FROM rust:${RUST_VERSION} AS builder
# Install wasm target
RUN rustup target add wasm32-unknown-unknown
# Copy trunk from installer stage
COPY --from=trunk-installer /usr/local/cargo/bin/trunk /usr/local/cargo/bin/trunk
# Set working directory
WORKDIR /app
# Copy cached dependencies
COPY --from=dependencies /app/target target
# Copy source code
COPY . .
# Clean any previous builds and build for release
RUN trunk clean && trunk build --release

# Stage 4: Production nginx server
FROM nginx:alpine

# Add metadata labels
LABEL maintainer="firat@firathonca.online"
LABEL version="1.0.0"
LABEL description="Portfolio site built with Rust/WASM"
LABEL org.opencontainers.image.source="https://github.com/Mightybeast12/portfolio_site"

# Create non-root user for security
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy built assets from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Change ownership to non-root user
RUN chown -R appuser:appgroup /usr/share/nginx/html /var/cache/nginx /var/run /var/log/nginx

# Switch to non-root user
USER appuser

# Expose port 8080 (Cloud Run requirement)
EXPOSE 8080

# Start nginx in foreground
CMD ["nginx", "-g", "daemon off;"]
