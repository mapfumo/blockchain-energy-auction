#!/bin/bash

echo "🚀 Starting Energy Trading System Services"
echo "=========================================="

# Kill any existing processes
echo "🧹 Cleaning up existing processes..."
pkill -f "target/debug/gateway" 2>/dev/null || true
pkill -f "target/debug/examples/bess_server_demo" 2>/dev/null || true
pkill -f "next dev" 2>/dev/null || true

# Wait a moment
sleep 2

# Start WebSocket Gateway
echo "🌐 Starting WebSocket Gateway..."
cd /home/tony/Desktop/energy-trading/energy-trading-rust
cargo run --bin gateway > gateway.log 2>&1 &
GATEWAY_PID=$!
echo "✅ Gateway started with PID: $GATEWAY_PID"

# Wait for gateway to start
sleep 3

# Start BESS Server Demo
echo "🔋 Starting BESS Server Demo..."
cargo run --example bess_server_demo > bess_demo.log 2>&1 &
BESS_PID=$!
echo "✅ BESS Server started with PID: $BESS_PID"

# Wait a moment
sleep 2

# Start Frontend
echo "🎨 Starting Frontend Dashboard..."
cd /home/tony/Desktop/energy-trading/frontend
npm run dev > frontend.log 2>&1 &
FRONTEND_PID=$!
echo "✅ Frontend started with PID: $FRONTEND_PID"

echo ""
echo "🎯 All services started!"
echo "========================="
echo "📊 Dashboard: http://localhost:3000"
echo "🔌 WebSocket: ws://localhost:8080/ws"
echo "🔋 BESS Server: Running on random port"
echo ""
echo "📝 Logs:"
echo "  Gateway: /home/tony/Desktop/energy-trading/energy-trading-rust/gateway.log"
echo "  BESS:    /home/tony/Desktop/energy-trading/energy-trading-rust/bess_demo.log"
echo "  Frontend: /home/tony/Desktop/energy-trading/frontend/frontend.log"
echo ""
echo "🛑 To stop all services: pkill -f 'gateway|bess_server|next'"
echo ""
echo "⏳ Waiting for services to initialize..."
sleep 5

echo "✅ Services should be ready! Check the dashboard at http://localhost:3000"
