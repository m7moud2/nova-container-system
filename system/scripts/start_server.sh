#!/bin/bash

# Kill any existing process on port 3000
echo "🧹 Cleaning up old processes..."
lsof -t -i :3000 | xargs kill -9 2>/dev/null || true

# Rebuild (fast check)
echo "🔨 Checking build..."
cargo build --release --quiet

# Start Server
echo "🚀 Starting Nova Cloud Dashboard..."
echo "👉 Open: http://localhost:3000/login.html"
./target/release/nova_container_system dashboard --port 3000
