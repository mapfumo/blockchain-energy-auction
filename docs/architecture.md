# Energy Trading System Architecture

## 1. System Overview

The Energy Trading System is a distributed real-time auction platform enabling competitive bidding between Battery Energy Storage Systems (BESS) and energy aggregators, with blockchain settlement and comprehensive monitoring capabilities.

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Energy Trading Ecosystem                          │
├─────────────────┬───────────────────┬─────────────────┬───────────────────────┤
│   BESS Nodes    │  Aggregator Nodes │  WebSocket      │   Web Dashboard       │
│   (Rust/Tokio)  │   (Rust/Tokio)    │  Gateway        │   (Next.js/React)     │
│                 │                   │  (Axum/Rust)    │                       │
│  ┌───────────┐  │  ┌─────────────┐  │  ┌───────────┐  │  ┌─────────────────┐  │
│  │Battery #1 │◄─┼─►│Aggregator#1 │◄─┼─►│WebSocket  │◄─┼─►│Real-time        │  │
│  │Battery #2 │  │  │Aggregator#2 │  │  │Server     │  │  │Dashboard        │  │
│  │Battery #N │  │  │Aggregator#N │  │  │           │  │  │                 │  │
│  └───────────┘  │  └─────────────┘  │  └───────────┘  │  └─────────────────┘  │
└─────────────────┴───────────────────┴─────────────────┴───────────────────────┘
                               │
                               ▼
                 ┌─────────────────────────────┐
                 │     Solana Blockchain       │
                 │   ┌───────────────────────┐ │
                 │   │ Energy Trading        │ │
                 │   │ Smart Contract        │ │
                 │   │ (Anchor Program)      │ │
                 │   └───────────────────────┘ │
                 └─────────────────────────────┘
                               │
                               ▼
                 ┌─────────────────────────────┐
                 │      Data Layer             │
                 │ ┌──────────┐ ┌────────────┐ │
                 │ │PostgreSQL│ │   Redis    │ │
                 │ │TimescaleDB│ │  (Cache)   │ │
                 │ └──────────┘ └────────────┘ │
                 └─────────────────────────────┘
```

## 2. Component Architecture

### 2.1 BESS Node Architecture ✅ IMPLEMENTED

```rust
// Core BESS Node Components
┌─────────────────────────────────────────┐
│              BESS Node                  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   TCP Server    │ │  Battery Core │  │
│  │   (Tokio)       │ │   State       │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Message     │ │ │ │ Energy    │ │  │
│  │ │ Handler     │ │ │ │ Manager   │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Connection  │ │ │ │ Bid       │ │  │
│  │ │ Manager     │ │ │ │ Evaluator │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │  WebSocket      │ │  Blockchain   │  │
│  │  Client         │ │  Client       │  │
│  │                 │ │  (Solana)     │  │
│  └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────┘
```

**✅ Implemented Components:**

- **BESSTCPServer**: Production-ready TCP server with concurrent connection handling
- **Message Handler**: Processes all 10 ETP message types with timing constraints
- **Connection Manager**: Handles multiple aggregator connections simultaneously
- **Battery Core State**: Energy levels, pricing, health status management
- **Bid Evaluator**: Reserve price checking and competitive pricing logic

#### Key Components:

- **TCP Server**: Handles incoming connections from aggregators
- **Message Handler**: Processes ETP messages according to protocol spec
- **Connection Manager**: Manages multiple concurrent aggregator connections
- **Battery Core State**: Maintains energy levels, pricing, health status
- **AI Pricing Engine**: Dynamic reserve price calculation
- **WebSocket Client**: Real-time updates to monitoring system
- **Blockchain Client**: Settlement transaction submission

### 2.2 Aggregator Node Architecture ✅ IMPLEMENTED

```rust
// Core Aggregator Node Components
┌─────────────────────────────────────────┐
│           Aggregator Node               │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   TCP Client    │ │ Bidding Core  │  │
│  │   Manager       │ │               │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Connection  │ │ │ │ Strategy  │ │  │
│  │ │ Pool        │ │ │ │ Engine    │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Discovery   │ │ │ │ Market    │ │  │
│  │ │ Service     │ │ │ │ Analyzer  │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │  WebSocket      │ │  Blockchain   │  │
│  │  Client         │ │  Client       │  │
│  │                 │ │  (Solana)     │  │
│  └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────┘
```

**✅ Implemented Components:**

- **AggregatorNode**: Intelligent bidding strategies with historical context
- **Bidding Strategies**: Multiple algorithms (aggressive, conservative, adaptive)
- **Historical Tracking**: Bid success rates and price prediction
- **Market Analysis**: Competition analysis and bid optimization
- **Connection Management**: TCP client with connection pooling

### 2.3 WebSocket Gateway Architecture ✅ IMPLEMENTED

```rust
// WebSocket Gateway Components
┌─────────────────────────────────────────┐
│         WebSocket Gateway               │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   Axum Server   │ │ Event Bus     │  │
│  │                 │ │ (Broadcast)   │  │
│  │ ┌─────────────┐ │ │               │  │
│  │ │ WebSocket   │ │ │ ┌───────────┐ │  │
│  │ │ Handlers    │ │ │ │ Message   │ │  │
│  │ └─────────────┘ │ │ │ Router    │ │  │
│  │                 │ │ └───────────┘ │  │
│  │ ┌─────────────┐ │ │               │  │
│  │ │ REST API    │ │ │ ┌───────────┐ │  │
│  │ │ Endpoints   │ │ │ │ Client    │ │  │
│  │ └─────────────┘ │ │ │ Manager   │ │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │  Data Layer     │ │  Monitoring   │  │
│  │  (PostgreSQL)   │ │  & Metrics    │  │
│  └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────┘
```

**✅ Implemented Components:**

- **WebSocketGateway**: Real-time event broadcasting system
- **Event Bus**: Broadcast channels for system events
- **Message Router**: ETP message routing and processing
- **Client Manager**: WebSocket connection management
- **CORS Support**: Cross-origin resource sharing for frontend integration
- **System Events**: Auction, bid, and performance event types

### 2.4 Web Dashboard Architecture

```typescript
// Frontend Component Hierarchy
┌─────────────────────────────────────────┐
│           Next.js Dashboard             │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │     Pages       │ │   Components  │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Dashboard   │ │ │ │ Battery   │ │  │
│  │ │ Overview    │ │ │ │ Cards     │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Auction     │ │ │ │ Activity  │ │  │
│  │ │ Monitor     │ │ │ │ Feed      │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  │                 │ │               │  │
│  │ ┌─────────────┐ │ │ ┌───────────┐ │  │
│  │ │ Analytics   │ │ │ │ Charts &  │ │  │
│  │ │ Reports     │ │ │ │ Graphs    │ │  │
│  │ └─────────────┘ │ │ └───────────┘ │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │  State Mgmt     │ │  WebSocket    │  │
│  │  (React Query)  │ │  Client       │  │
│  └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────┘
```

**✅ Implemented Frontend Components:**

- **Next.js Dashboard**: Real-time energy trading monitoring interface
- **WebSocket Integration**: Live connection with automatic reconnection
- **Event Processing**: Real-time auction events, bid progression, and system metrics
- **TailwindCSS Styling**: Responsive design with modern UI components
- **Test Components**: WebSocket debugging and connection testing tools
- **CORS Support**: Cross-origin WebSocket connection handling

## 3. Data Flow Architecture

### 3.1 Auction Message Flow

```
Aggregator                BESS Node              WebSocket Gateway        Dashboard
    │                         │                         │                    │
    │──Query Message─────────►│                         │                    │
    │                         │──Battery Status────────►│                    │
    │                         │                         │──Real-time Update──►│
    │◄────QueryResponse───────│                         │                    │
    │                         │                         │                    │
    │──Bid Message───────────►│                         │                    │
    │                         │──Bid Evaluation────────►│                    │
    │                         │                         │──Bid Activity─────►│
    │◄────Bid Accept/Reject───│                         │                    │
    │                         │                         │                    │
    │──Bid Confirmation──────►│                         │                    │
    │                         │──Settlement Trigger────►│                    │
    │                         │                         │──Transaction Update►│
    │                         │                         │                    │
    │              Blockchain Settlement                 │                    │
    │◄─────────────(Solana Program)────────────────────►│                    │
```

### 3.2 Real-time Data Propagation

```
System Event Generation:
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ BESS Node   │────│ Event Bus   │────│ WebSocket   │
│ Updates     │    │ (Channels)  │    │ Broadcast   │
└─────────────┘    └─────────────┘    └─────────────┘
                           │                   │
┌─────────────┐            │                   │
│ Aggregator  │────────────┘                   │
│ Updates     │                                │
└─────────────┘                                │
                                               ▼
                                   ┌─────────────────┐
                                   │ Connected       │
                                   │ Dashboard       │
                                   │ Clients         │
                                   └─────────────────┘
```

## 4. Database Schema Design

### 4.1 Why PostgreSQL is Essential

PostgreSQL is critical for this project despite our simplified monitoring approach:

**Blockchain Integration Requirements:**

- **Immutable Settlement Records**: Store blockchain transaction hashes and settlement data
- **Audit Trail**: Complete history of all auctions and transactions for regulatory compliance
- **State Recovery**: Resume system state after restarts without losing active auctions
- **Historical Analysis**: Track price improvements over time to demonstrate competitive benefits

**Service Persistence:**

- **Battery Registration**: Persistent device IDs and configurations
- **Aggregator Management**: Store aggregator credentials and reputation scores
- **Auction State**: Save in-progress auctions to survive service restarts
- **Configuration Data**: System settings and parameters

**Demonstration Data:**

- **Price Improvement Tracking**: Historical data showing competitive pricing benefits
- **Competition Metrics**: Long-term analysis of aggregator participation
- **Performance Analytics**: System performance over time for optimization

### 4.2 Core Entities

```sql
-- Battery Energy Storage Systems
CREATE TABLE batteries (
    id SERIAL PRIMARY KEY,
    device_id INTEGER UNIQUE NOT NULL,
    owner_pubkey VARCHAR(44) NOT NULL,
    energy_total DECIMAL(10,2) NOT NULL,
    percentage_for_sale DECIMAL(5,2) NOT NULL,
    reserve_price DECIMAL(8,2) NOT NULL,
    health_status INTEGER NOT NULL,
    voltage DECIMAL(6,2) NOT NULL,
    discharge_rate DECIMAL(6,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Energy Aggregators
CREATE TABLE aggregators (
    id SERIAL PRIMARY KEY,
    device_id INTEGER UNIQUE NOT NULL,
    owner_pubkey VARCHAR(44) NOT NULL,
    max_bid_price DECIMAL(8,2) NOT NULL,
    energy_requirements DECIMAL(10,2) NOT NULL,
    reputation_score INTEGER DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Auction Records
CREATE TABLE auctions (
    id BIGSERIAL PRIMARY KEY,
    battery_id INTEGER NOT NULL REFERENCES batteries(id),
    aggregator_id INTEGER REFERENCES aggregators(id),
    energy_amount DECIMAL(10,2) NOT NULL,
    reserve_price DECIMAL(8,2) NOT NULL,
    final_price DECIMAL(8,2),
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    blockchain_tx_hash VARCHAR(88),
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settled_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Message Log (TimescaleDB for time-series)
CREATE TABLE message_log (
    id BIGSERIAL,
    timestamp TIMESTAMPTZ NOT NULL,
    message_type INTEGER NOT NULL,
    source_device_id INTEGER NOT NULL,
    target_device_id INTEGER,
    message_data JSONB NOT NULL,
    processing_time_ms DECIMAL(8,3),
    success BOOLEAN NOT NULL DEFAULT true,
    error_details TEXT
);

-- Convert to hypertable for time-series optimization
SELECT create_hypertable('message_log', 'timestamp');
```

### 4.2 Performance Metrics Tables

```sql
-- System Performance Metrics
CREATE TABLE performance_metrics (
    id BIGSERIAL,
    timestamp TIMESTAMPTZ NOT NULL,
    metric_type VARCHAR(50) NOT NULL,
    component VARCHAR(50) NOT NULL,
    value DECIMAL(12,4) NOT NULL,
    metadata JSONB
);

SELECT create_hypertable('performance_metrics', 'timestamp');

-- Trading Statistics
CREATE TABLE trading_stats (
    id BIGSERIAL,
    timestamp TIMESTAMPTZ NOT NULL,
    total_active_batteries INTEGER,
    total_active_aggregators INTEGER,
    active_auctions INTEGER,
    avg_settlement_time_ms DECIMAL(10,2),
    total_volume_kwh DECIMAL(12,2),
    avg_price_cents_kwh DECIMAL(8,2)
);

SELECT create_hypertable('trading_stats', 'timestamp');
```

## 5. API Design

### 5.1 WebSocket API

```typescript
// WebSocket Message Types
interface WebSocketMessage {
  type: MessageType;
  timestamp: number;
  data: any;
}

enum MessageType {
  // Server -> Client
  SYSTEM_SNAPSHOT = "system_snapshot",
  BATTERY_STATUS = "battery_status",
  AUCTION_STARTED = "auction_started",
  BID_PLACED = "bid_placed",
  BID_ACCEPTED = "bid_accepted",
  BID_REJECTED = "bid_rejected",
  AUCTION_SETTLED = "auction_settled",
  SYSTEM_ALERT = "system_alert",
  PERFORMANCE_METRIC = "performance_metric",

  // Client -> Server
  SUBSCRIBE = "subscribe",
  UNSUBSCRIBE = "unsubscribe",
  CONTROL_COMMAND = "control_command",
  REQUEST_SNAPSHOT = "request_snapshot",
}
```

### 5.2 REST API Endpoints

```rust
// REST API Routes
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: i64,
}

// Battery Management
GET    /api/batteries              // List all batteries
GET    /api/batteries/{id}         // Get battery details
PUT    /api/batteries/{id}/config  // Update battery configuration
GET    /api/batteries/{id}/history // Get trading history

// Aggregator Management
GET    /api/aggregators            // List all aggregators
GET    /api/aggregators/{id}       // Get aggregator details
GET    /api/aggregators/{id}/bids  // Get bidding history

// Auction Management
GET    /api/auctions               // List auctions (with filters)
GET    /api/auctions/{id}          // Get auction details
POST   /api/auctions/{id}/settle   // Manual settlement trigger

// Analytics & Reporting
GET    /api/analytics/overview     // System overview metrics
GET    /api/analytics/performance  // Performance metrics
GET    /api/analytics/market       // Market analysis data
GET    /api/reports/trading        // Generate trading reports

// System Health
GET    /api/health                 // System health check
GET    /api/metrics                // Prometheus metrics endpoint
```

## 6. Security Architecture

### 6.1 Authentication & Authorization

```rust
// Multi-layer Security Model
┌─────────────────────────────────────────┐
│              Security Layers            │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   TLS/SSL       │ │  API Keys     │  │
│  │   Encryption    │ │  Management   │  │
│  │                 │ │               │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   Input         │ │  Rate         │  │
│  │   Validation    │ │  Limiting     │  │
│  │                 │ │               │  │
│  └─────────────────┘ └───────────────┘  │
├─────────────────────────────────────────┤
│  ┌─────────────────┐ ┌───────────────┐  │
│  │   Blockchain    │ │  Audit        │  │
│  │   Signatures    │ │  Logging      │  │
│  │                 │ │               │  │
│  └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────┘
```

### 6.2 Network Security

```rust
// Security Configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub tls_cert_path: PathBuf,
    pub tls_key_path: PathBuf,
    pub api_key_salt: String,
    pub rate_limit_requests_per_minute: u32,
    pub max_connections_per_ip: u32,
    pub allowed_origins: Vec<String>,
    pub audit_log_level: LogLevel,
}
```

## 7. Scalability & Performance

### 7.1 Horizontal Scaling Strategy

```
Load Balancer (HAProxy/Nginx)
           │
    ┌──────┼──────┐
    ▼      ▼      ▼
┌─────┐ ┌─────┐ ┌─────┐
│WS   │ │WS   │ │WS   │  WebSocket Gateway Instances
│GW#1 │ │GW#2 │ │GW#N │  (Auto-scaling based on load)
└─────┘ └─────┘ └─────┘
    │      │      │
    └──────┼──────┘
           ▼
    ┌─────────────┐
    │   Redis     │      Shared State & Session Store
    │   Cluster   │      (Message routing, client sessions)
    └─────────────┘
           │
           ▼
    ┌─────────────┐
    │ PostgreSQL  │      Primary Database
    │ + Read      │      (with read replicas)
    │ Replicas    │
    └─────────────┘
```

### 7.2 Performance Optimization

```rust
// Performance Monitoring Points
pub struct PerformanceMetrics {
    // Message processing
    pub message_processing_time: Histogram,
    pub messages_per_second: Counter,
    pub failed_message_count: Counter,

    // Network performance
    pub connection_pool_size: Gauge,
    pub websocket_client_count: Gauge,
    pub tcp_connection_duration: Histogram,

    // Database performance
    pub db_query_duration: Histogram,
    pub db_connection_pool_usage: Gauge,

    // Blockchain performance
    pub blockchain_tx_confirmation_time: Histogram,
    pub blockchain_tx_success_rate: Counter,
}
```

## 8. Deployment Architecture

### 8.1 Container Strategy

```dockerfile
# Multi-stage Rust build for optimal container size
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 libpq5
COPY --from=builder /app/target/release/energy-trading /usr/local/bin/
EXPOSE 8080 9090
CMD ["energy-trading"]
```

### 8.2 Docker Compose Development

```yaml
version: "3.8"
services:
  # Database services
  postgres:
    image: timescale/timescaledb:latest-pg15
    environment:
      POSTGRES_DB: energy_trading
      POSTGRES_USER: energy_user
      POSTGRES_PASSWORD: secure_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  # Application services
  websocket-gateway:
    build:
      context: .
      dockerfile: Dockerfile.websocket-gateway
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://energy_user:secure_password@postgres:5432/energy_trading
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis

  dashboard:
    build:
      context: ./dashboard
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - WEBSOCKET_URL=ws://websocket-gateway:8080/ws
    depends_on:
      - websocket-gateway
```

## 9. Real-time Monitoring & Observability

### 9.1 WebSocket Event Broadcasting

```rust
// Real-time event system for competitive pricing demonstration
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub enum SystemEvent {
    AuctionStarted {
        battery_id: u32,
        reserve_price: f64,
        timestamp: u64,
    },
    BidPlaced {
        battery_id: u32,
        aggregator_id: u32,
        bid_price: f64,
        timestamp: u64,
    },
    BidAccepted {
        battery_id: u32,
        final_price: f64,
        aggregator_id: u32,
        price_improvement: f64,
        timestamp: u64,
    },
    BidRejected {
        battery_id: u32,
        aggregator_id: u32,
        reason: String,
        timestamp: u64,
    },
    AuctionCompleted {
        battery_id: u32,
        final_price: f64,
        reserve_price: f64,
        price_improvement_percent: f64,
        aggregator_count: usize,
        timestamp: u64,
    },
    SystemMetrics {
        active_connections: usize,
        active_auctions: usize,
        avg_processing_time_ms: u64,
        total_energy_traded_kwh: f64,
    },
}

// WebSocket gateway with event broadcasting
pub struct WebSocketGateway {
    event_tx: broadcast::Sender<SystemEvent>,
    active_auctions: Arc<DashMap<u32, AuctionState>>,
    metrics: Arc<SystemMetrics>,
}

impl WebSocketGateway {
    pub async fn broadcast_event(&self, event: SystemEvent) {
        let _ = self.event_tx.send(event);
    }
}
```

### 9.2 Structured Logging with Focus on Economic Impact

```rust
// Logging focused on demonstrating competitive pricing benefits
use tracing::{info, warn, error, instrument};

#[instrument(skip(self), fields(battery_id = %battery_id, aggregator_id = %aggregator_id))]
pub async fn process_bid(&self, battery_id: u32, aggregator_id: u32, bid: BidMessage) -> Result<BidResponse> {
    let start_time = Instant::now();

    info!(
        battery_id = %battery_id,
        aggregator_id = %aggregator_id,
        bid_price = bid.price,
        reserve_price = self.reserve_price,
        energy_amount = bid.energy_amount,
        "Processing competitive bid"
    );

    let response = self.evaluate_bid(bid).await?;
    let processing_time = start_time.elapsed().as_millis();

    if processing_time > 500 {
        warn!("Bid processing exceeded 500ms threshold: {}ms", processing_time);
    }

    // Log economic impact
    if response.accepted {
        let price_improvement = ((bid.price - self.reserve_price) / self.reserve_price) * 100.0;
        info!(
            battery_id = %battery_id,
            final_price = bid.price,
            reserve_price = self.reserve_price,
            price_improvement_percent = price_improvement,
            aggregator_id = %aggregator_id,
            "Bid accepted - demonstrating competitive pricing benefit"
        );
    }

    Ok(response)
}
```

### 9.3 Simple Metrics Collection

```rust
// In-memory metrics for real-time monitoring
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Debug, Clone, Serialize)]
pub struct SystemMetrics {
    // Economic impact metrics (core demonstration focus)
    pub total_auctions: AtomicUsize,
    pub avg_price_improvement_percent: AtomicU64, // stored as basis points
    pub total_energy_traded_kwh: AtomicU64, // stored as 1000ths
    pub max_price_improvement: AtomicU64, // best competitive outcome

    // System performance metrics
    pub active_connections: AtomicUsize,
    pub messages_processed: AtomicUsize,
    pub avg_bid_processing_time_ms: AtomicU64,

    // Competition metrics (key for demonstration)
    pub avg_aggregators_per_auction: AtomicU64, // stored as 100ths
    pub successful_auctions: AtomicUsize,
    pub auctions_with_competition: AtomicUsize, // multiple bidders
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            total_auctions: AtomicUsize::new(0),
            avg_price_improvement_percent: AtomicU64::new(0),
            total_energy_traded_kwh: AtomicU64::new(0),
            max_price_improvement: AtomicU64::new(0),
            active_connections: AtomicUsize::new(0),
            messages_processed: AtomicUsize::new(0),
            avg_bid_processing_time_ms: AtomicU64::new(0),
            avg_aggregators_per_auction: AtomicU64::new(0),
            successful_auctions: AtomicUsize::new(0),
            auctions_with_competition: AtomicUsize::new(0),
        }
    }

    pub fn record_auction_completion(&self, price_improvement: f64, energy_kwh: f64, aggregator_count: usize) {
        self.total_auctions.fetch_add(1, Ordering::Relaxed);
        self.successful_auctions.fetch_add(1, Ordering::Relaxed);

        if aggregator_count > 1 {
            self.auctions_with_competition.fetch_add(1, Ordering::Relaxed);
        }

        // Update running averages and track maximum improvement
        let improvement_basis_points = (price_improvement * 100.0) as u64;
        self.max_price_improvement.fetch_max(improvement_basis_points, Ordering::Relaxed);

        // Convert energy to 1000ths for storage
        let energy_1000ths = (energy_kwh * 1000.0) as u64;
        self.total_energy_traded_kwh.fetch_add(energy_1000ths, Ordering::Relaxed);
    }

    pub fn get_competition_benefit_summary(&self) -> CompetitionSummary {
        CompetitionSummary {
            total_auctions: self.total_auctions.load(Ordering::Relaxed),
            auctions_with_competition: self.auctions_with_competition.load(Ordering::Relaxed),
            avg_price_improvement: self.avg_price_improvement_percent.load(Ordering::Relaxed) as f64 / 100.0,
            max_price_improvement: self.max_price_improvement.load(Ordering::Relaxed) as f64 / 100.0,
            total_energy_traded: self.total_energy_traded_kwh.load(Ordering::Relaxed) as f64 / 1000.0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CompetitionSummary {
    pub total_auctions: usize,
    pub auctions_with_competition: usize,
    pub avg_price_improvement: f64,
    pub max_price_improvement: f64,
    pub total_energy_traded: f64,
}
```

## 10. Test-Driven Development (TDD) Architecture

### 10.1 TDD Philosophy for Energy Trading System

Test-Driven Development is essential for this project due to:

- **Strict timing requirements** (≤500ms critical messages)
- **Complex message protocol** (14-field ETP messages)
- **Concurrent system behavior** (multiple aggregators competing)
- **Blockchain integration** (immutable settlement records)
- **Competitive pricing validation** (proving economic benefits)

### 10.2 TDD Workflow: Red-Green-Refactor

```rust
// 1. RED: Write a failing test
#[tokio::test]
async fn test_bid_processing_within_500ms() {
    let battery = Battery::new(100, 20.0, 15.0);
    let bid = BidMessage::new(1, 18.0, 5.0);

    let start = Instant::now();
    let response = battery.process_bid(bid).await;
    let elapsed = start.elapsed();

    assert!(elapsed <= Duration::from_millis(500));
    assert!(response.is_ok());
}

// 2. GREEN: Make it pass with minimal code
impl Battery {
    pub async fn process_bid(&self, bid: BidMessage) -> Result<BidResponse> {
        let timeout = Duration::from_millis(500);
        tokio::time::timeout(timeout, self.evaluate_bid(bid)).await?
    }
}

// 3. REFACTOR: Improve the implementation
impl Battery {
    pub async fn process_bid(&self, bid: BidMessage) -> Result<BidResponse> {
        let start = Instant::now();
        let timeout = Duration::from_millis(500);

        let result = tokio::time::timeout(timeout, self.evaluate_bid(bid)).await?;

        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(500) {
            warn!("Bid processing exceeded 500ms: {:?}", elapsed);
        }

        Ok(result)
    }
}
```

### 10.3 Test Categories and Examples

#### 10.3.1 Unit Tests (Fast, Isolated)

```rust
// Message Protocol Tests
#[test]
fn test_etp_message_serialization_roundtrip() {
    let original = ETPMessage::new_bid(123, 15.5, 10.0);
    let serialized = original.serialize().unwrap();
    let deserialized = ETPMessage::deserialize(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_message_validation() {
    let valid_msg = ETPMessage::new_bid(123, 15.5, 10.0);
    assert!(valid_msg.validate().is_ok());

    let invalid_msg = ETPMessage::new_invalid();
    assert!(invalid_msg.validate().is_err());
}

// Bid Evaluation Tests
#[test]
fn test_bid_evaluation_above_reserve() {
    let battery = Battery::new(100, 20.0, 15.0); // $15 reserve
    let bid = BidMessage::new(1, 18.0, 5.0); // $18 bid

    let result = battery.evaluate_bid(bid);
    assert!(result.accepted);
    assert_eq!(result.final_price, 18.0);
}

#[test]
fn test_bid_evaluation_below_reserve() {
    let battery = Battery::new(100, 20.0, 15.0); // $15 reserve
    let bid = BidMessage::new(1, 12.0, 5.0); // $12 bid

    let result = battery.evaluate_bid(bid);
    assert!(!result.accepted);
    assert_eq!(result.reason, "Bid below reserve price");
}
```

#### 10.3.2 Integration Tests (Medium Speed)

```rust
// Full Auction Flow Tests
#[tokio::test]
async fn test_complete_auction_flow() {
    let battery = Battery::new(100, 20.0, 15.0);
    let aggregator = Aggregator::new(1, 18.0);

    // Query phase
    let query = QueryMessage::new(1);
    let response = battery.handle_query(query).await.unwrap();
    assert_eq!(response.energy_available, 20.0);

    // Bid phase
    let bid = BidMessage::new(1, 18.0, 5.0);
    let bid_response = battery.process_bid(bid).await.unwrap();
    assert!(bid_response.accepted);

    // Confirmation phase
    let confirm = ConfirmMessage::new(1, 18.0, 5.0);
    let confirm_response = battery.handle_confirmation(confirm).await.unwrap();
    assert!(confirm_response.success);
}

// Competitive Pricing Tests
#[tokio::test]
async fn test_multiple_aggregators_competition() {
    let battery = Battery::new(100, 20.0, 15.0); // $15 reserve
    let aggregators = vec![
        Aggregator::new(1, 16.0), // Low bid
        Aggregator::new(2, 18.0), // Medium bid
        Aggregator::new(3, 20.0), // High bid
    ];

    let auction = Auction::new(battery, aggregators);
    let result = auction.run().await;

    // Should get price above reserve due to competition
    assert!(result.final_price > 15.0);
    assert_eq!(result.final_price, 20.0); // Highest bid wins
    assert!(result.aggregator_count > 1); // Multiple bidders
    assert!(result.price_improvement > 0.0); // Above reserve
}
```

#### 10.3.3 Performance Tests (Slow, Load)

```rust
// Timing Requirements Tests
#[tokio::test]
async fn test_critical_message_timing_under_load() {
    let battery = Battery::new(100, 20.0, 15.0);
    let mut handles = vec![];

    // Spawn 100 concurrent bid processing tasks
    for i in 0..100 {
        let battery = battery.clone();
        let handle = tokio::spawn(async move {
            let bid = BidMessage::new(i, 18.0, 1.0);
            let start = Instant::now();
            let result = battery.process_bid(bid).await;
            let elapsed = start.elapsed();
            (result.is_ok(), elapsed)
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results = futures::future::join_all(handles).await;

    // Verify all completed within 500ms
    for (success, elapsed) in results {
        assert!(success);
        assert!(elapsed <= Duration::from_millis(500));
    }
}

// Throughput Tests
#[tokio::test]
async fn test_message_throughput_1000_per_second() {
    let battery = Battery::new(100, 20.0, 15.0);
    let start = Instant::now();
    let mut handles = vec![];

    // Process 1000 messages
    for i in 0..1000 {
        let battery = battery.clone();
        let handle = tokio::spawn(async move {
            let bid = BidMessage::new(i, 18.0, 1.0);
            battery.process_bid(bid).await
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    let elapsed = start.elapsed();

    // Should complete within 1 second
    assert!(elapsed <= Duration::from_secs(1));

    // All should succeed
    for result in results {
        assert!(result.is_ok());
    }
}
```

### 10.4 TDD Test Structure

```rust
// Test module organization
#[cfg(test)]
mod tests {
    use super::*;

    mod unit_tests {
        mod message_tests {
            // ETP message serialization/deserialization
        }

        mod battery_tests {
            // Bid evaluation logic
        }

        mod aggregator_tests {
            // Bidding strategy tests
        }
    }

    mod integration_tests {
        mod auction_flow_tests {
            // End-to-end auction scenarios
        }

        mod competitive_pricing_tests {
            // Multiple aggregator competition
        }

        mod websocket_tests {
            // Real-time monitoring
        }
    }

    mod performance_tests {
        mod timing_tests {
            // Critical message timing
        }

        mod load_tests {
            // Throughput and concurrency
        }
    }
}
```

### 10.5 TDD Benefits for This Project

**1. Timing Requirements Validation**

- Tests enforce ≤500ms critical message processing
- Performance regression detection
- Load testing under realistic conditions

**2. Competitive Pricing Verification**

- Tests prove multiple aggregators lead to better prices
- Economic benefit validation
- Market behavior simulation

**3. Message Protocol Reliability**

- Binary serialization roundtrip validation
- Protocol compliance verification
- Error handling coverage

**4. System Integration Confidence**

- End-to-end auction flow validation
- WebSocket real-time monitoring tests
- Blockchain settlement integration tests

This architecture provides a solid foundation for building your energy trading system with all the components we discussed. The design emphasizes performance, scalability, and maintainability while preserving the core auction mechanics from your research.
