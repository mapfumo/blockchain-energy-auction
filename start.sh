#!/bin/bash

# Energy Trading System Startup Script
# Automatically detects the correct host IP for Docker containers

echo "🚀 Starting Energy Trading System..."

# Get the Docker bridge gateway IP (most reliable method)
DOCKER_GATEWAY=$(docker run --rm alpine ip route show default | awk '/default/ {print $3}')

if [ -z "$DOCKER_GATEWAY" ]; then
    echo "❌ Could not detect Docker gateway IP"
    exit 1
fi

echo "📍 Detected Docker gateway IP: $DOCKER_GATEWAY"

# Update docker-compose.yml with the detected IP
sed -i "s/GATEWAY_HOST=.*/GATEWAY_HOST=$DOCKER_GATEWAY/" docker-compose.yml

echo "✅ Updated docker-compose.yml with gateway IP: $DOCKER_GATEWAY"

# Start Solana validator if not running
if ! pgrep -f "solana-test-validator" > /dev/null; then
    echo "🔗 Starting Solana validator..."
    solana-test-validator --reset --quiet &
    sleep 5
else
    echo "✅ Solana validator already running"
fi

# Start the gateway if not running
if ! pgrep -f "simple-gateway" > /dev/null; then
    echo "🌐 Starting gateway..."
    cd simple-gateway && cargo run &
    cd ..
    sleep 3
else
    echo "✅ Gateway already running"
fi

# Start the frontend if not running
if ! pgrep -f "next dev" > /dev/null; then
    echo "🎨 Starting frontend..."
    cd frontend && npm run dev &
    cd ..
    sleep 3
else
    echo "✅ Frontend already running"
fi

# Start Docker containers
echo "🐳 Starting BESS nodes and aggregators..."
docker-compose up -d bess-001 bess-002 bess-003 aggregator-001 aggregator-002

# Wait for containers to start
sleep 5

# Check system status
echo ""
echo "📊 System Status:"
echo "=================="

# Check BESS nodes
BESS_COUNT=$(curl -s http://localhost:8080/api/bess-list | jq '.count // 0' 2>/dev/null || echo "0")
echo "🔋 BESS Nodes: $BESS_COUNT"

# Check aggregators  
AGG_COUNT=$(curl -s http://localhost:8080/api/aggregator-list | jq '.count // 0' 2>/dev/null || echo "0")
echo "⚡ Aggregators: $AGG_COUNT"

# Check services
echo "🌐 Gateway: $(curl -s http://localhost:8080/health > /dev/null && echo "✅ Running" || echo "❌ Down")"
echo "🎨 Frontend: $(curl -s http://localhost:3000 > /dev/null && echo "✅ Running" || echo "❌ Down")"
echo "🔗 Solana: $(pgrep -f "solana-test-validator" > /dev/null && echo "✅ Running" || echo "❌ Down")"

echo ""
echo "🎉 System startup complete!"
echo "📱 Frontend: http://localhost:3000"
echo "🔧 Gateway API: http://localhost:8080"
echo "📊 Health: http://localhost:8080/health"
