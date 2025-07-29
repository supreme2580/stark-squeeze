#!/bin/bash

# 🚀 Railway Deployment Script for Stark Squeeze Backend
# This script helps you deploy your backend to Railway

set -e

echo "🚀 Starting Railway deployment for Stark Squeeze Backend..."

# Check if Railway CLI is installed
if ! command -v railway &> /dev/null; then
    echo "❌ Railway CLI not found. Installing..."
    curl -fsSL https://railway.app/install.sh | sh
    echo "✅ Railway CLI installed"
fi

# Check if user is logged in
if ! railway whoami &> /dev/null; then
    echo "🔐 Please login to Railway..."
    railway login
fi

# Build the project
echo "🔨 Building project..."
cargo build --release

# Check if Dockerfile exists
if [ ! -f "Dockerfile.backend" ]; then
    echo "❌ Dockerfile.backend not found!"
    exit 1
fi

# Check if railway.json exists
if [ ! -f "railway.json" ]; then
    echo "❌ railway.json not found!"
    exit 1
fi

# Deploy to Railway
echo "🚀 Deploying to Railway..."
railway up

echo "✅ Deployment complete!"
echo "🌐 Your API should be available at the URL shown above"
echo "📊 Check Railway dashboard for logs and monitoring"

# Test the deployment
echo "🧪 Testing deployment..."
sleep 10  # Wait for deployment to complete

# Get the service URL
SERVICE_URL=$(railway status --json | jq -r '.service.url' 2>/dev/null || echo "")

if [ -n "$SERVICE_URL" ]; then
    echo "🔍 Testing health endpoint..."
    curl -s "$SERVICE_URL/health" | jq . || echo "⚠️ Health check failed"
else
    echo "⚠️ Could not get service URL. Check Railway dashboard."
fi

echo "🎉 Deployment script completed!"
echo "📝 Remember to:"
echo "   - Set environment variables in Railway dashboard"
echo "   - Update your frontend to use the new API URL"
echo "   - Test all endpoints" 