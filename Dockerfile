# Use stable Rust version
FROM rust:1.81 as builder

# Install Node.js for Leptos frontend build
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && apt-get install -y nodejs

# Install wasm32 target for frontend compilation
RUN rustup target add wasm32-unknown-unknown

# Instead of installing cargo-leptos, we'll build directly with cargo
# This avoids the cargo-leptos dependency conflicts entirely

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./
COPY Cargo.lock ./

# Copy the source code
COPY . .

# Build the WASM frontend first
RUN cargo build --target wasm32-unknown-unknown --no-default-features --features=hydrate --release

# Install wasm-bindgen-cli for WASM processing  
RUN cargo install wasm-bindgen-cli --version 0.2.95

# Create target directories
RUN mkdir -p target/site/pkg

# Process the WASM file (the lib will be named clock_it.wasm)
RUN wasm-bindgen --out-dir target/site/pkg --target web --no-typescript target/wasm32-unknown-unknown/release/clock_it.wasm

# Build the server
RUN cargo build --release --no-default-features --features=ssr

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

# Create directory for SQLite database
RUN mkdir -p /app/data

# Expose the port the app runs on
EXPOSE 3000

# Set environment variables
ENV LEPTOS_SITE_ROOT="target/site"
ENV DATABASE_URL="sqlite:///app/data/clock_it.db"

# Make startup script executable
RUN chmod +x /app/start.sh

# Run the application
CMD ["./start.sh"]