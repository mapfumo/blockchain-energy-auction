#!/bin/bash

# Energy Trading System Stop Script

echo "🛑 Stopping Energy Trading System..."

# Stop Docker containers
echo "🐳 Stopping Docker containers..."
docker-compose down

# Stop background processes
echo "🔄 Stopping background processes..."

# Stop Solana validator
if pgrep -f "solana-test-validator" > /dev/null; then
    echo "🔗 Stopping Solana validator..."
    pkill -f "solana-test-validator"
fi

# Stop gateway
if pgrep -f "simple-gateway" > /dev/null; then
    echo "🌐 Stopping gateway..."
    pkill -f "simple-gateway"
fi

# Stop frontend
if pgrep -f "next dev" > /dev/null; then
    echo "🎨 Stopping frontend..."
    pkill -f "next dev"
fi

echo "✅ System stopped successfully!"
