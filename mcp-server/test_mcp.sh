#!/bin/bash

# Test script for Energy Trading MCP Server
echo "🧪 Testing Energy Trading MCP Server for Blockchain Settlement Status"
echo "=================================================================="

# Test 1: Initialize
echo -e "\n1️⃣ Testing initialize..."
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize"}' | ./target/release/energy-trading-mcp-server

# Test 2: List resources
echo -e "\n2️⃣ Testing resources/list..."
echo '{"jsonrpc": "2.0", "id": 2, "method": "resources/list"}' | ./target/release/energy-trading-mcp-server

# Test 3: Read recent settlements
echo -e "\n3️⃣ Testing resources/read (recent settlements)..."
echo '{"jsonrpc": "2.0", "id": 3, "method": "resources/read", "params": {"uri": "settlement://recent"}}' | ./target/release/energy-trading-mcp-server

# Test 4: List tools
echo -e "\n4️⃣ Testing tools/list..."
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/list"}' | ./target/release/energy-trading-mcp-server

# Test 5: Query settlement status
echo -e "\n5️⃣ Testing tools/call (query_settlement_status)..."
echo '{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "query_settlement_status", "arguments": {"auction_id": 1}}}' | ./target/release/energy-trading-mcp-server

# Test 6: Verify settlement
echo -e "\n6️⃣ Testing tools/call (verify_settlement)..."
echo '{"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "verify_settlement", "arguments": {"transaction_signature": "settlement_sig_00003039"}}}' | ./target/release/energy-trading-mcp-server

# Test 7: Monitor settlements
echo -e "\n7️⃣ Testing tools/call (monitor_settlements)..."
echo '{"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "monitor_settlements", "arguments": {"limit": 3}}}' | ./target/release/energy-trading-mcp-server

echo -e "\n✅ All tests completed!"
