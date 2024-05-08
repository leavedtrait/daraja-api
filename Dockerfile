# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the cargo manifests to the container
COPY Cargo.toml Cargo.lock ./

# Cache dependencies by creating a dummy project
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the source code into the container
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Create a new stage to reduce image size
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/your_service_name .

# Expose the port the service runs on
EXPOSE 8000

# Command to run the service
CMD ["./your_service_name"]
