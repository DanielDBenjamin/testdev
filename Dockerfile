# Use stable Rust version with good compatibility  
FROM rust:1.81 as builder

# Install Node.js for Leptos frontend build
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && apt-get install -y nodejs

# Install wasm32 target for frontend compilation
RUN rustup target add wasm32-unknown-unknown

# Try to install the same version that works locally, but with locked dependencies
RUN cargo install cargo-leptos --version 0.2.44 --locked

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./
COPY Cargo.lock ./

# Copy the source code
COPY . .

# Build the application with cargo-leptos
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
COPY --from=builder /app/start.sh /app/start.sh

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