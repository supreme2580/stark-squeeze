#!/bin/bash

# Docker build script for Stark Squeeze backend
set -e

echo "🐳 Building Stark Squeeze Backend Docker Image..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker and try again."
    exit 1
fi

# Build the backend image
echo "📦 Building backend image..."
docker build -f Dockerfile.backend -t starksqueeze-backend:latest .

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Backend image built successfully!"
    echo "📊 Image details:"
    docker images starksqueeze-backend:latest
else
    echo "❌ Build failed!"
    exit 1
fi

echo "🚀 Ready to deploy with: docker-compose up -d" 