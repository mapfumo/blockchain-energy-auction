# MCP Server Integration Guide

## Overview

The Energy Trading MCP Server provides blockchain settlement status data through the Model Context Protocol (MCP). This allows AI assistants and other clients to query settlement information, verify transactions, and monitor trading activity.

## Quick Start

### Build and Run

```bash
cd mcp-server
cargo build --release
./target/release/energy-trading-mcp-server
```

### Test the Server

```bash
./test_mcp.sh
```

## Available Resources

### 1. Auction Settlements (`settlement://auctions`)

- **Description**: All auction settlement data
- **Data**: Complete settlement records with blockchain signatures
- **Format**: JSON array of `AuctionSettlement` objects

### 2. Aggregator Status (`settlement://aggregators`)

- **Description**: Aggregator performance and settlement data
- **Data**: Reputation scores, settlement counts, trading volumes
- **Format**: JSON array of `AggregatorStatus` objects

### 3. Battery Status (`settlement://batteries`)

- **Description**: Battery settlement and earnings data
- **Data**: Capacity, energy sold, earnings, activity status
- **Format**: JSON array of `BatteryStatus` objects

### 4. Recent Settlements (`settlement://recent`)

- **Description**: Recently completed settlements
- **Data**: Last 3 settlements sorted by timestamp
- **Format**: JSON array of `AuctionSettlement` objects

## Available Tools

### 1. `query_settlement_status`

Query settlement status for a specific auction.

**Parameters:**

- `auction_id` (integer): Auction ID to query

**Example:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "query_settlement_status",
    "arguments": {
      "auction_id": 1
    }
  }
}
```

### 2. `verify_settlement`

Verify a settlement transaction on blockchain.

**Parameters:**

- `transaction_signature` (string): Solana transaction signature to verify

**Example:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "verify_settlement",
    "arguments": {
      "transaction_signature": "settlement_sig_00003039"
    }
  }
}
```

### 3. `get_aggregator_performance`

Get performance metrics for an aggregator.

**Parameters:**

- `aggregator_id` (integer): Aggregator ID to query

**Example:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "get_aggregator_performance",
    "arguments": {
      "aggregator_id": 1
    }
  }
}
```

### 4. `monitor_settlements`

Monitor recent settlement activity.

**Parameters:**

- `limit` (integer, optional): Number of recent settlements to return (default: 10)

**Example:**

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "monitor_settlements",
    "arguments": {
      "limit": 5
    }
  }
}
```

## Data Models

### AuctionSettlement

```rust
{
  "auction_id": 1,
  "energy_amount_kwh": 7.5,
  "final_price_cents": 550,
  "total_value_usd": 41.25,
  "settled": true,
  "settlement_signature": "settlement_sig_00003039",
  "blockchain_url": "https://explorer.solana.com/tx/...",
  "timestamp": 1757316512,
  "winner": "AGG-002",
  "seller": "BESS-002"
}
```

### AggregatorStatus

```rust
{
  "aggregator_id": 1,
  "reputation_score": 85,
  "successful_settlements": 42,
  "total_energy_traded_kwh": 356.8,
  "total_usdc_paid": 12750,
  "last_settlement": 1757316512
}
```

### BatteryStatus

```rust
{
  "battery_id": 1,
  "capacity_kwh": 20.0,
  "total_energy_sold_kwh": 50.0,
  "total_usdc_earned": 3500,
  "last_sale": 1757316512,
  "active": true
}
```

## Integration with Energy Trading System

The MCP server integrates with your existing energy trading system by:

1. **Connecting to Solana RPC** - Uses the same blockchain client as your gateway
2. **Exposing Settlement Data** - Provides access to auction settlements, aggregator performance, and battery status
3. **Real-time Verification** - Can verify settlement transactions on-chain
4. **Monitoring Tools** - Provides tools for monitoring settlement activity

## Configuration

- **Solana RPC**: `http://127.0.0.1:8899` (local validator)
- **Program ID**: `4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb`
- **Protocol**: JSON-RPC over stdin/stdout

## Error Handling

The server returns proper JSON-RPC error responses for:

- Invalid method names (code: -32601)
- Invalid parameters (code: -32602)
- Internal errors (code: -32603)

## Development

To extend the MCP server:

1. Add new resources in the `resources/list` handler
2. Add new tools in the `tools/list` handler
3. Implement tool logic in the corresponding async methods
4. Update data models in `settlement_data.rs` as needed

## Testing

Run the comprehensive test suite:

```bash
./test_mcp.sh
```

This tests all resources and tools to ensure proper functionality.
