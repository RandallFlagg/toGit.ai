#!/bin/bash

# Default value for DAYS
DEFAULT_DAYS=30

# Check if a command-line argument is provided; if so, use it; otherwise, use the default
DAYS=${1:-$DEFAULT_DAYS}

cd src-tauri

# Remove all build artifacts older than a certain number of days
cargo sweep -t $DAYS

echo "Deleted build artifacts older than $DAYS days."

# Path to the incremental directory
INCREMENTAL_DIR="target/debug/incremental"

# Find and delete files older than X days
find "$INCREMENTAL_DIR" -type f -mtime +$DAYS -exec rm -f {} \;

echo "Deleted incremental files older than $DAYS days."
