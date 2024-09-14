# Use an official Rust image as a parent image
FROM rust:latest

# Install Trunk
RUN cargo install trunk

# Set the working directory
WORKDIR /app

# Copy the Trunk configuration and source code
COPY . .

# Build the Yew app using Trunk
RUN trunk build --release

# Expose the port for the Trunk server
EXPOSE 8080

# Start Trunk server to serve the Yew app
CMD ["trunk", "serve", "--port", "80"]
