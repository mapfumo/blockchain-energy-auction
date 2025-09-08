# Energy Trading MCP Server

MCP (Model Context Protocol) server for blockchain settlement status in the energy trading system.

## Overview

This MCP server exposes blockchain settlement data and provides tools for querying, verifying, and monitoring energy trading settlements on Solana.

## Features

### Resources

- **Auction Settlements** (`settlement://auctions`) - All auction settlement data
- **Aggregator Status** (`settlement://aggregators`) - Aggregator performance metrics
- **Battery Status** (`settlement://batteries`) - Battery earnings and capacity data
- **Recent Settlements** (`settlement://recent`) - Recently completed settlements

### Tools

- **query_settlement_status** - Query settlement status for a specific auction
- **verify_settlement** - Verify a settlement transaction on blockchain
- **get_aggregator_performance** - Get performance metrics for an aggregator
- **monitor_settlements** - Monitor recent settlement activity

## Usage

### Build and Run

```bash
cd mcp-server
cargo build --release
cargo run
```

### MCP Client Integration

The server communicates via JSON-RPC over stdin/stdout. Example client usage:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/read",
  "params": {
    "uri": "settlement://recent"
  }
}
```

### Example Queries

**Get all auction settlements:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "resources/read",
  "params": {
    "uri": "settlement://auctions"
  }
}
```

**Query specific settlement:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "query_settlement_status",
    "arguments": {
      "auction_id": 1
    }
  }
}
```

**Verify settlement transaction:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "verify_settlement",
    "arguments": {
      "transaction_signature": "settlement_sig_00003039"
    }
  }
}
```

## Data Models

### AuctionSettlement

```rust
{
  "auction_id": 1,
  "energy_amount_kwh": 15.5,
  "final_price_cents": 650,
  "total_value_usd": 10.075,
  "settled": true,
  "settlement_signature": "settlement_sig_00003039",
  "blockchain_url": "https://explorer.solana.com/tx/...",
  "timestamp": 1703123456,
  "winner": "AGG-001",
  "seller": "BESS-001"
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
  "last_settlement": 1703123456
}
```

## Integration with Energy Trading System

This MCP server integrates with your existing energy trading system by:

1. **Connecting to Solana RPC** - Uses the same blockchain client as your gateway
2. **Exposing Settlement Data** - Provides access to auction settlements, aggregator performance, and battery status
3. **Real-time Verification** - Can verify settlement transactions on-chain
4. **Monitoring Tools** - Provides tools for monitoring settlement activity

## Configuration

The server connects to your local Solana validator at `http://127.0.0.1:8899` and uses the program ID from your deployed energy trading contract.

## Development

To extend the MCP server:

1. Add new resources in `main.rs` `resources/list` handler
2. Add new tools in `main.rs` `tools/list` handler
3. Implement tool logic in the corresponding async methods
4. Update data models in `settlement_data.rs` as needed

## Dependencies

- `tokio` - Async runtime
- `serde` - Serialization
- `solana-client` - Solana blockchain integration
- `anyhow` - Error handling
- `tracing` - Logging
