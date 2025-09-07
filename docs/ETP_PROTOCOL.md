# Energy Trading Protocol (ETP) Specification

## Overview

The Energy Trading Protocol (ETP) defines the communication standards and message flow for the Energy Trading System. **The ETP protocol is fundamentally based on the `SystemEvent` enum**, which defines all message types and their data structures.

This document specifies how auctions are numbered, how interactions are assigned to auction numbers, and the complete message flow.

## Auction Numbering System

### Sequential Counter Implementation

**Auction IDs are generated using a simple sequential counter:**

```rust
let mut auction_id = 1;  // Start at 1
// ... auction processing ...
auction_id += 1;  // Increment after each auction
```

### Key Characteristics

- **Sequential**: 1, 2, 3, 4, 5... (no gaps)
- **Persistent**: Each auction_id is saved to database
- **Unique**: Every auction gets a unique sequential number
- **Traceable**: All events for an auction share the same ID

## Interaction Assignment

### Event-to-Auction Mapping

All interactions within a single auction cycle receive the **same auction_id**:

| Event Type         | Auction ID | Description           |
| ------------------ | ---------- | --------------------- |
| `AuctionStarted`   | ✅ Same ID | Announces new auction |
| `QuerySent`        | ❌ No ID   | Pre-auction queries   |
| `QueryResponse`    | ❌ No ID   | Pre-auction responses |
| `BidPlaced`        | ✅ Same ID | Bids for the auction  |
| `BidAccepted`      | ✅ Same ID | Accepted bids         |
| `BidRejected`      | ✅ Same ID | Rejected bids         |
| `AuctionCompleted` | ✅ Same ID | Final results         |

### Event Flow Per Auction

**Auction #N** follows this sequence:

1. **AuctionStarted** - Announces auction N
2. **QuerySent** - Aggregators query BESS nodes (no auction_id)
3. **QueryResponse** - BESS nodes respond (no auction_id)
4. **BidPlaced** - Multiple bids for auction N
5. **BidAccepted/BidRejected** - Bid results for auction N
6. **AuctionCompleted** - Final results for auction N

## Message Types

The ETP protocol consists of 12 message types defined in the `SystemEvent` enum:

### Core Auction Messages

#### AuctionStarted

```rust
SystemEvent::AuctionStarted {
    auction_id: u64,        // Sequential auction number
    total_energy: f64,      // Available energy in kWh
    reserve_price: f64,     // Minimum price in cents/kWh
}
```

#### BidPlaced

```rust
SystemEvent::BidPlaced {
    auction_id: u64,        // Same as AuctionStarted
    aggregator_id: u64,     // Bidding aggregator
    bess_id: u64,          // Target BESS node
    bid_price: f64,        // Bid price in cents/kWh
    energy_amount: f64,    // Energy amount in kWh
}
```

#### AuctionCompleted

```rust
SystemEvent::AuctionCompleted {
    auction_id: u64,           // Same as AuctionStarted
    winner_aggregator_id: u64, // Winning aggregator
    seller_bess_id: u64,       // Selling BESS node
    energy_sold: f64,          // Energy sold in kWh
    final_price: f64,          // Final price in cents/kWh
    total_value: f64,          // Total value in cents
    auction_duration_ms: u64,  // Duration in milliseconds
}
```

### Pre-Auction Messages

#### QuerySent

```rust
SystemEvent::QuerySent {
    aggregator_id: u64,    // Querying aggregator
    bess_id: u64,         // Target BESS node
    // No auction_id - pre-auction query
}
```

#### QueryResponse

```rust
SystemEvent::QueryResponse {
    aggregator_id: u64,    // Responding aggregator
    bess_id: u64,         // Responding BESS node
    energy_available: f64, // Available energy in kWh
    // No auction_id - pre-auction response
}
```

## Timing and Delays

### Auction Intervals

```rust
// Random delay between 2-10 seconds before next auction
let delay_seconds = 2.0 + ((auction_id * 7) as f64 * 0.13) % 8.0;
```

### Bid Delays

```rust
// Random delay between 5-10 seconds between bids
let bid_delay = 5.0 + ((auction_id * 7 + i as u64 * 13) as f64 * 0.23) % 5.0;
```

### Query Delays

```rust
// Micro-delay between queries (50-200ms)
let micro_delay = 50.0 + ((i * bess_id) as f64 * 0.1) % 150.0;
```

## Database Persistence

### Auction Records

Each auction is persisted to the database:

```rust
let new_auction = NewAuction {
    battery_id: 101,                    // Default BESS node
    energy_amount: BigDecimal::from(total_energy as i64),
    reserve_price: BigDecimal::from(reserve_price as i64),
    status: "active".to_string(),
};
```

### Event Tracking

- **AuctionStarted** → Creates database record
- **BidPlaced** → Logged but not persisted (future enhancement)
- **AuctionCompleted** → Updates database record with results

## ETP Protocol Implementation

### Message Definition

The ETP protocol is implemented as a Rust enum:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    // Core auction messages
    AuctionStarted { auction_id: u64, total_energy: f64, reserve_price: f64 },
    BidPlaced { auction_id: u64, aggregator_id: u64, bess_id: u64, bid_price: f64, energy_amount: f64 },
    BidAccepted { auction_id: u64, aggregator_id: u64, bess_id: u64, final_price: f64, energy_amount: f64 },
    BidRejected { aggregator_id: u64, bess_id: u64, reason: String },
    AuctionCompleted { auction_id: u64, winner_aggregator_id: u64, seller_bess_id: u64, energy_sold: f64, final_price: f64, total_value: f64, auction_duration_ms: u64 },

    // Pre-auction messages
    QuerySent { aggregator_id: u64, bess_id: u64 },
    QueryResponse { bess_id: u64, energy_available: f64, percentage_for_sale: f64 },

    // System status messages
    EnergyDepleted { bess_id: u64, final_energy: f64, energy_percentage: f64 },
    EnergyRecharged { bess_id: u64, energy_added: f64, new_total: f64, energy_percentage: f64 },
    SystemMetrics { total_auctions: u64, total_bids: u64, avg_price_improvement_percent: f64, active_bess_nodes: u64, active_aggregators: u64 },
    BESSNodeStatus { device_id: u64, energy_available: f64, battery_health: u8, is_online: bool },
    AggregatorStatus { device_id: u64, strategy: String, success_rate: f64, total_bids: u64, successful_bids: u64, total_energy_bought: f64, average_bid_price: f64, is_online: bool },
}
```

### Message Flow

1. **Definition** → `SystemEvent` enum in Rust
2. **Serialization** → Binary format via WebSocket
3. **Transmission** → Real-time broadcasting
4. **Deserialization** → TypeScript interfaces
5. **Processing** → Frontend event handlers

## Protocol Benefits

### Traceability

- **Complete Audit Trail**: Every auction can be traced from start to finish
- **Event Correlation**: All related events share the same auction_id
- **Database Queries**: Easy to query all events for a specific auction

### Simplicity

- **Sequential Numbering**: No complex ID generation logic
- **Consistent Assignment**: Same ID for all auction-related events
- **Predictable Flow**: Clear event sequence per auction

### Scalability

- **Unique IDs**: No collision risk with sequential numbering
- **Database Indexing**: Efficient queries on auction_id
- **Event Filtering**: Easy to filter events by auction

## Implementation Details

### Gateway Event Generation

```rust
// Main auction loop
loop {
    // 1. Generate auction
    let auction_event = SystemEvent::AuctionStarted {
        auction_id: auction_id as u64,
        total_energy,
        reserve_price,
    };

    // 2. Process bids
    for aggregator in aggregators {
        let bid_event = SystemEvent::BidPlaced {
            auction_id: auction_id as u64,  // Same ID
            // ... bid details
        };
    }

    // 3. Complete auction
    let completion_event = SystemEvent::AuctionCompleted {
        auction_id: auction_id as u64,  // Same ID
        // ... completion details
    };

    // 4. Increment for next auction
    auction_id += 1;
}
```

### Frontend Event Processing

```typescript
// Frontend groups events by auction_id
const handleSystemEvent = (event: SystemEvent) => {
  if (event.AuctionStarted) {
    // Start new auction tracking
    setAuctions((prev) => [
      ...prev,
      {
        id: event.auction_id,
        // ... auction details
      },
    ]);
  }

  if (event.BidPlaced) {
    // Add bid to existing auction
    updateAuctionBids(event.auction_id, event);
  }

  if (event.AuctionCompleted) {
    // Mark auction as completed
    completeAuction(event.auction_id, event);
  }
};
```

## Conclusion

The ETP auction numbering system provides:

- ✅ **Simple Sequential Numbering**: Easy to understand and implement
- ✅ **Complete Traceability**: Every auction can be fully reconstructed
- ✅ **Consistent Event Assignment**: All related events share the same ID
- ✅ **Database Integration**: Seamless persistence and querying
- ✅ **Scalable Design**: Handles multiple concurrent auctions efficiently

This protocol ensures that anyone can understand the complete flow of any auction by following the sequential auction_id through all related events.
