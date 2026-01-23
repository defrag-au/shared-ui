#!/bin/bash
set -e

# Shared worker build script for shared-ui workspace
# Detects and builds frontend if present, then builds the worker

# Check if this worker has a frontend directory
if [ -d "frontend" ]; then
    echo "Found frontend directory - building frontend assets..."

    # Install trunk if not already installed
    if ! command -v trunk &> /dev/null; then
        echo "Installing trunk..."
        cargo install trunk --locked
    fi

    # Build frontend
    cd frontend
    echo "Building frontend with trunk..."
    trunk build --release
    cd ..

    echo "Frontend built to dist/"
fi

# Install pinned worker-build version
# Version 0.1.11 is compatible with worker 0.6.6
cargo install -q worker-build --version 0.1.11 --locked

# Build the worker
worker-build --release
