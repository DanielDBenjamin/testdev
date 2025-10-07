FROM rustlang/rust:nightly as builder

# Install system dependencies
RUN apt-get update && apt-get install -y binaryen npm && rm -rf /var/lib/apt/lists/*
RUN npm install -g sass

WORKDIR /app

# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
RUN rustup target add wasm32-unknown-unknown

# Install tools once (cached layer)
RUN cargo install wasm-bindgen-cli --version 0.2.103

# Now copy source code
COPY . .

# Build
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
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

# Make executable
RUN chmod +x /app/clock-it /app/start.sh

# Create directory for SQLite database
RUN mkdir -p /app/data

EXPOSE 3000
CMD ["./start.sh"]