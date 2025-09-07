#!/bin/bash

# Energy Trading System Stop Script
# This script stops the entire containerized energy trading system

set -e

echo "🛑 Stopping Energy Trading System..."
echo "=================================="

# Check if Docker Compose is available
if ! command -v docker-compose > /dev/null 2>&1; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Stop and remove containers
echo "🔄 Stopping containers..."
docker-compose down

# Remove volumes (optional - uncomment if you want to reset data)
# echo "🗑️  Removing volumes..."
# docker-compose down -v

# Remove images (optional - uncomment if you want to clean up)
# echo "🧹 Removing images..."
# docker-compose down --rmi all

echo ""
echo "✅ Energy Trading System stopped successfully!"
echo "=================================="
echo ""
echo "📋 To start again: ./start-energy-trading.sh"
echo "🧹 To clean up everything: docker-compose down -v --rmi all"
echo ""

# Show remaining containers
echo "📦 Remaining containers:"
docker ps -a