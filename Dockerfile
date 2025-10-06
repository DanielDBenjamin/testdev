# Use a simpler, faster approach with nightly Rust
FROM rustlang/rust:nightly as builder

# Install system dependencies quickly
RUN apt-get update && apt-get install -y nodejs npm && rm -rf /var/lib/apt/lists/*
RUN npm install -g sass
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos directly without complex caching
RUN cargo install cargo-leptos --version 0.2.9

# Set working directory
WORKDIR /app

# Copy everything including .cargo directory
COPY . .

# Build the application with cargo-leptos
RUN cargo leptos build --release

# Verify build artifacts exist
RUN echo "=== Verifying build artifacts ===" && \
    ls -la target/site/ && \
    ls -la target/site/pkg/ && \
    test -f target/site/pkg/clock-it.css || echo "ERROR: CSS missing!" && \
    test -f target/site/pkg/clock-it.js || echo "ERROR: JS missing!" && \
    test -f target/site/pkg/clock-it_bg.wasm || echo "ERROR: WASM missing!"

# Use a smaller base image for the final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built application from the builder stage
COPY --from=builder /app/target/release/clock-it /app/clock-it
COPY --from=builder /app/target/site /app/target/site

# Verify files were copied
RUN ls -la /app/target/site/pkg/ || echo "WARNING: pkg directory not found!"

# Create directory for SQLite database
RUN mkdir -p /app/data