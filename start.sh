#!/bin/bash

# Set up the database directory
mkdir -p /app/data

# Check if database exists, if not create it and run migrations
if [ ! -f "/app/data/clock_it.db" ]; then
    echo "Database not found, creating new database..."
    
    # Create the database file
    touch /app/data/clock_it.db
    
    # Run migrations (you'll need to install sqlx-cli for this)
    # For now, we'll just create the database file
    echo "Database created at /app/data/clock_it.db"
fi

# Start the application
exec ./clock-it