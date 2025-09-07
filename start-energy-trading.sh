#!/bin/bash

# Energy Trading System Startup Script
# This script starts the entire containerized energy trading system

set -e

echo "🚀 Starting Energy Trading System..."
echo "=================================="

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Check if Docker Compose is available
if ! command -v docker-compose > /dev/null 2>&1; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p logs
mkdir -p data/postgres
mkdir -p data/solana

# Set permissions
chmod +x start-energy-trading.sh
chmod +x stop-energy-trading.sh

# Build and start services
echo "🔨 Building and starting services..."
docker-compose up --build -d

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check service health
echo "🏥 Checking service health..."

# Check Gateway
if curl -f http://localhost:8080/health > /dev/null 2>&1; then
    echo "✅ Gateway is healthy"
else
    echo "⚠️  Gateway health check failed"
fi

# Check Frontend (run locally)
echo "ℹ️  Frontend should be run locally: cd frontend && npm install && npm run dev"

# Check BESS Nodes
for i in 001 002 003; do
    if curl -f http://localhost:8081/health > /dev/null 2>&1; then
        echo "✅ BESS Node $i is healthy"
    else
        echo "⚠️  BESS Node $i health check failed"
    fi
done

# Check Aggregators
for i in 001 002; do
    if curl -f http://localhost:8082/health > /dev/null 2>&1; then
        echo "✅ Aggregator $i is healthy"
    else
        echo "⚠️  Aggregator $i health check failed"
    fi
done

# Check Solana Validator
if curl -f http://localhost:8899 > /dev/null 2>&1; then
    echo "✅ Solana Validator is healthy"
else
    echo "⚠️  Solana Validator health check failed"
fi

echo ""
echo "🎉 Energy Trading System is running!"
echo "=================================="
echo "📊 Frontend Dashboard: http://localhost:3000"
echo "🔌 WebSocket Gateway: ws://localhost:8080"
echo "🗄️  Database: localhost:5432"
echo "⛓️  Solana RPC: http://localhost:8899"
echo ""
echo "📋 Available Services:"
echo "  - 3x BESS Nodes (Energy Storage)"
echo "  - 2x Aggregators (Energy Buyers)"
echo "  - 1x Gateway (Orchestrator)"
echo "  - 1x Frontend (Dashboard) - Run locally"
echo "  - 1x PostgreSQL (Database) - Optional"
echo "  - 1x Solana Validator (Blockchain) - Local"
echo ""
echo "🔍 To view logs: docker-compose logs -f [service_name]"
echo "🛑 To stop: ./stop-energy-trading.sh"
echo ""

# Show running containers
echo "📦 Running containers:"
docker-compose ps
