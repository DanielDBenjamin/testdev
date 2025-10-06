# Use nightly Rust since Leptos 0.8.x requires future Rust versions
FROM rustlang/rust:nightly as dependencies

# Install Node.js for Leptos frontend build and SASS for CSS compilation
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && apt-get install -y nodejs
RUN npm install -g sass

# Install wasm32 target for frontend compilation
RUN rustup target add wasm32-unknown-unknown

# Set the working directory
WORKDIR /app

# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to build dependencies
RUN mkdir src && echo 'pub fn dummy() {}' > src/lib.rs && echo 'fn main() {}' > src/main.rs

# Update Cargo.lock to prevent version drift, then build dependencies
RUN cargo update
RUN cargo build --release --target wasm32-unknown-unknown --lib
RUN cargo build --release --bin clock-it

# Stage 2: Build the actual application
FROM dependencies as builder

# Install a simple version of cargo-leptos
RUN cargo install cargo-leptos --version 0.2.9

# Copy the actual source code
COPY . .

# Touch main.rs to ensure rebuild
RUN touch src/main.rs

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