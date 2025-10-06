# Use a simpler, faster approach with nightly Rust
FROM rustlang/rust:nightly as builder

# Install system dependencies quickly
RUN apt-get update && apt-get install -y nodejs npm && rm -rf /var/lib/apt/lists/*
RUN npm install -g sass
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos directly without complex caching
RUN cargo install cargo-leptos --version 0.2.9

# Set working directory and copy everything
WORKDIR /app
COPY . .

# Build the application with cargo-leptos (should work on nightly)
RUN cargo leptos build --release

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
COPY --from=builder /app/public /app/public
COPY --from=builder /app/style /app/style

# Create directory for SQLite database
RUN mkdir -p /app/data

# Expose the port the app runs on
EXPOSE 3000

# Set environment variables
ENV LEPTOS_SITE_ROOT="/app/target/site"
ENV LEPTOS_SITE_PKG_DIR="pkg" 
ENV DATABASE_URL="sqlite:///app/data/clock_it.db"
ENV CLOCK_IT_USE_TLS="false"

# Make the binary executable
RUN chmod +x /app/clock-it

# Ensure proper working directory and run the application
WORKDIR /app
CMD ["./clock-it"]