#!/bin/bash

# Detect if we're running locally or in Railway
if [ -d "/app" ]; then
    # Railway environment
    DATA_DIR="/app/data"
    BINARY="./clock-it"
else
    # Local environment
    DATA_DIR="./data"
    BINARY="./target/release/clock-it"
fi

# Set up the database directory
mkdir -p "$DATA_DIR"

# Check if database exists, if not create it
if [ ! -f "$DATA_DIR/clock_it.db" ]; then
    echo "Database not found, creating new database..."
    touch "$DATA_DIR/clock_it.db"
    echo "Database created at $DATA_DIR/clock_it.db"
fi

# Start the application
exec "$BINARY"