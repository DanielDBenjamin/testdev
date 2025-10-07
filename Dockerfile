FROM rustlang/rust:nightly as builder

# Install system dependencies
RUN apt-get update && apt-get install -y binaryen && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
RUN cargo install wasm-bindgen-cli --version 0.2.103

WORKDIR /app

# Copy everything
COPY . .

# Use our custom build script instead of cargo-leptos
RUN chmod +x build.sh && ./build.sh

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built artifacts
COPY --from=builder /app/clock-it /app/clock-it
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/start.sh /app/start.sh

# Make executable
RUN chmod +x /app/clock-it /app/start.sh

# Create directory for SQLite database
RUN mkdir -p /app/data

# Expose port
EXPOSE 3000

# Start the application
CMD ["./start.sh"]