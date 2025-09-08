# Energy Trading System - Project Overview

## Project Mission

**Goal**: Migrate a Golang energy auction prototype to a production-ready Rust/Solana system that **demonstrates competitive pricing benefits** through multiple aggregator bidding.

**Core Concept**: Prove that when multiple aggregators compete for BESS energy, BESS owners get better prices than the single-utility model.

## Research Foundation

Based on the academic paper "Communication requirements for enabling real-time energy trading among distributed energy storage systems and aggregators" by Antony Mapfumo (Queensland University of Technology).

### Key Research Specifications

- **10 message types** (0-9) for Energy Trading Protocol (ETP)
- **14-field message structure** with binary serialization
- **Strict timing requirements**: ≤500ms critical messages, ≤200ms device failures
- **HTTP registration** + direct communication architecture
- **Competitive pricing model** validation

## System Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Energy Trading Ecosystem                          │
├─────────────────┬───────────────────┬─────────────────┬───────────────────────┤
│   BESS Nodes    │  Aggregator Nodes │  WebSocket      │   Web Dashboard       │
│   (Docker)      │   (Docker)        │  Gateway        │   (Local Host)        │
│                 │                   │  (Local Host)   │                       │
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

### 1. BESS Nodes (Battery Energy Storage Systems)

- **HTTP registration** with gateway for device discovery
- **Direct TCP communication** with aggregators after registration
- **Battery state management** (energy, pricing, health, voltage)
- **Bid evaluation** against reserve price and market conditions
- **AI-powered dynamic pricing** based on market conditions
- **WebSocket client** for real-time monitoring updates
- **Blockchain client** for settlement transactions

### 2. Aggregator Nodes

- **HTTP registration** with gateway for device discovery
- **Direct TCP communication** with BESS nodes after registration
- **Intelligent bidding strategies** (not random increments)
- **Market analysis** for optimal timing
- **Concurrent auction handling** across multiple BESS
- **Reputation tracking** and strategy adjustment

### 3. WebSocket Gateway

- **HTTP registration endpoints** for BESS nodes and aggregators
- **Device registry** to track all registered devices
- **Axum-based WebSocket server** for real-time monitoring
- **Event broadcasting** focused on competitive pricing demonstration
- **Simple in-memory metrics** collection (no external monitoring systems)
- **REST API** for historical data queries and device management
- **Client authentication** and session management

### 4. Dashboard (Next.js/TypeScript)

- **Real-time auction monitoring** with competitive pricing visualization
- **Live bid progression** and price improvement tracking
- **Economic impact dashboard** (reserve vs final prices)
- **System health overview** (connections, performance)
- **Competition metrics** (aggregator count, success rates)
- **Mobile-responsive design** for demonstration purposes

### 5. Solana Smart Contracts

- **Immutable settlement records** for all transactions
- **USDC/SOL payment processing** between aggregators and BESS owners
- **Reputation score management** for batteries and aggregators
- **Event emission** for real-time monitoring
- **Dispute resolution mechanisms**

## Technology Stack

### Backend (Rust)

- **Tokio**: Async runtime for high-performance networking
- **Axum**: Web framework for WebSocket server and REST API
- **Serde**: Serialization/deserialization for message protocol
- **In-memory storage**: HashMap-based data structures for simplicity
- **Tracing**: Structured logging and distributed tracing
- **Solana SDK**: Blockchain integration and transaction handling
- **Anchor**: Solana smart contract framework

### Frontend (TypeScript/React)

- **Next.js**: React framework with SSR/SSG capabilities
- **TypeScript**: Type-safe JavaScript for robust frontend code
- **TailwindCSS**: Utility-first CSS framework for responsive design
- **Recharts**: Data visualization library for trading charts
- **React Query**: Server state management and caching
- **WebSocket API**: Native browser WebSocket for real-time updates

### Database & Infrastructure

- **In-memory data storage**: Simple HashMap-based storage for demonstration purposes
- **Docker + Compose**: Containerization and local development
- **GitHub Actions**: CI/CD pipeline for automated testing and deployment

## Message Protocol (ETP)

### Message Types (0-9)

```
0: Register    - Join trading multicast group
1: Query       - Request BESS availability
2: QueryResponse - BESS availability response
3: Bid         - Aggregator energy bid
4: BidAccept   - BESS accepts bid
5: BidConfirm  - Aggregator confirms transaction
6: BidReject   - BESS rejects bid
7: Terminate   - End trading session
8: DeviceFailure - Critical system failure
9: BESSStatus  - Periodic status updates
```

### Message Structure (14 fields)

```rust
struct ETPMessage {
    message_type: u8,                    // 1 byte
    message_id: u64,                     // 8 bytes
    device_id: u64,                      // 8 bytes
    ttl: u8,                            // 1 byte
    bid_price: f64,                     // 8 bytes - cents/kWh
    sale_price: f64,                    // 8 bytes - final price
    energy_total: f64,                  // 8 bytes - kWh available
    percentage_for_sale: f64,           // 8 bytes - % available
    required_energy_amount: f64,        // 8 bytes
    termination_code: u8,               // 1 byte
    remaining_battery_energy: f64,      // 8 bytes
    battery_health_status_code: u8,     // 1 byte
    battery_voltage: f64,               // 8 bytes
    discharge_rate: f64,                // 8 bytes
}
```

### Timing Requirements

| Message Type              | Priority      | Max Delay (ms) |
| ------------------------- | ------------- | -------------- |
| Device Failure            | 0 (very high) | 200            |
| Bid Accept/Reject/Confirm | 5-15 (high)   | 500            |
| Query Response            | 70 (low)      | 500            |
| BESS Status               | 60 (medium)   | 2000           |
| Register                  | 80 (low)      | 5000           |

## Test-Driven Development (TDD)

### TDD Philosophy

Test-Driven Development is essential for this project due to:

- **Strict timing requirements** (≤500ms critical messages)
- **Complex message protocol** (14-field ETP messages)
- **Concurrent system behavior** (multiple aggregators competing)
- **Blockchain integration** (immutable settlement records)
- **Competitive pricing validation** (proving economic benefits)

### TDD Workflow: Red-Green-Refactor

1. **RED**: Write failing test
2. **GREEN**: Make it pass with minimal code
3. **REFACTOR**: Improve implementation

### TDD Priority Order

1. **Timing Requirements Tests** (≤500ms critical messages)
2. **Message Protocol Tests** (ETP serialization/validation)
3. **Competitive Pricing Tests** (multiple aggregators)
4. **Integration Tests** (end-to-end auction flows)
5. **Performance Tests** (load and throughput)

### Test Categories

- **Unit Tests**: Fast, isolated (message protocol, bid evaluation)
- **Integration Tests**: Medium speed (auction flows, competition)
- **Performance Tests**: Slow, load (timing, throughput)

## Implementation Phases

### Phase 1: Rust Core Implementation (Priority 1)

- **ETP Message Protocol**: Binary serialization with 14 fields
- **BESS Node**: HTTP registration + TCP server with bid evaluation and timing enforcement
- **Aggregator Node**: HTTP registration + TCP client with intelligent bidding strategies
- **Network Architecture**: HTTP registration + direct communication

### Phase 2: Real-time Monitoring (Priority 2)

- **WebSocket Gateway**: Event broadcasting for live monitoring
- **Simple Metrics**: In-memory metrics focused on competitive pricing
- **Dashboard**: Real-time auction monitoring and price visualization

### Phase 3: Blockchain Integration (Priority 3) ✅ COMPLETED

- **Solana Smart Contracts**: Complete Anchor program with auction settlement, USDC payments, and reputation tracking
- **Blockchain Client**: Transaction submission and confirmation with comprehensive error handling
- **USDC Integration**: Automatic token transfers between aggregators and BESS owners
- **Comprehensive Testing**: 20+ test cases covering security, edge cases, and performance

### Phase 4: Advanced Features (Priority 4)

- **AI and Machine Learning**: Intelligent bidding and dynamic pricing
- **Analytics and Reporting**: Advanced market analysis

### Phase 5: Production Deployment (Priority 5)

- **Testing and Validation**: Comprehensive test coverage
- **Deployment and Operations**: Production readiness

## Performance Requirements

### Critical Performance Targets

- **Message Processing**: ≤500ms for critical messages (Bid Accept/Reject/Confirm)
- **Throughput**: 1000+ messages per second
- **Concurrency**: 100+ concurrent BESS connections per aggregator
- **WebSocket**: 50+ concurrent dashboard connections
- **Availability**: 99.5% uptime for core trading system
- **Data Loss**: <1% message loss rate under normal conditions

### Load Testing Requirements

- **Timing Validation**: All critical messages must complete within 500ms under load
- **Concurrent Connections**: System must handle 100+ simultaneous connections
- **Message Throughput**: Must process 1000+ messages per second
- **Memory Usage**: Efficient memory usage under sustained load
- **Error Recovery**: Graceful handling of network failures and timeouts

## Success Criteria

### Technical Success

- [ ] All timing requirements met under load testing
- [ ] Message timing requirements satisfied (≤500ms for critical messages)
- [ ] 99.5% system availability achieved in production
- [ ] Zero data loss during normal operations
- [ ] 1000+ messages/second throughput achieved

### Business Success

- [ ] **Demonstrate competitive pricing benefits**: Show clear price improvements when multiple aggregators compete
- [ ] **Prove economic viability**: BESS owners get better prices than single-utility model
- [ ] **Show scalability**: System handles multiple concurrent auctions with multiple bidders
- [ ] **Validate real-time performance**: Meet all timing requirements from research paper

### Demonstration Success

- [ ] Live auction monitoring shows price improvements
- [ ] Dashboard clearly shows competitive pricing benefits
- [ ] System handles realistic load scenarios
- [ ] Economic impact is measurable and visible

## Key Features

### Competitive Pricing Demonstration

- **Real-time auction monitoring** with bid progression
- **Price improvement tracking** (reserve vs final prices)
- **Competition metrics** (aggregator count, success rates)
- **Economic impact visualization** (charts and graphs)
- **Historical analysis** of competitive benefits

### Real-time Performance

- **WebSocket monitoring** for live system updates
- **Timing enforcement** for critical message processing
- **Performance metrics** collection and display
- **System health monitoring** with alerts and notifications

### Blockchain Integration

- **Immutable settlement records** on Solana
- **USDC/SOL payment processing** for transactions
- **Transaction monitoring** and confirmation status
- **Audit trail** for regulatory compliance

## Development Guidelines

### Code Quality Standards

- **Write tests first** before implementing any feature
- **Red-Green-Refactor cycle** for all development
- **Test timing requirements first** (≤500ms critical messages)
- **Test competitive pricing scenarios** before implementation
- 80%+ test coverage for core trading logic
- Integration tests for end-to-end auction flows
- Load testing to validate ≤500ms message timing requirements
- Structured logging with tracing spans for observability
- Error handling with circuit breaker patterns

### Technology Preferences

- **Backend**: Rust with Tokio, Axum, SQLx, Serde
- **Frontend**: Next.js, TypeScript, TailwindCSS, Recharts
- **Database**: PostgreSQL + TimescaleDB, Redis
- **Blockchain**: Solana SDK + Anchor
- **Infrastructure**: Docker, GitHub Actions

### Anti-Patterns to Avoid

- Writing code without tests first
- Ignoring timing requirements
- String-based message serialization
- Blocking I/O in async contexts
- Hardcoding network addresses
- Skipping performance tests for critical paths

## Project Status

### Completed ✅

- [x] Project architecture and requirements analysis
- [x] Research paper analysis and technical specifications
- [x] Documentation updates (simplified monitoring approach)
- [x] Technology stack selection and validation
- [x] TDD approach documentation and guidelines
- [x] **Phase 1**: Complete Rust core implementation (ETP, BESS, Aggregator, Network)
- [x] **Phase 2.1**: WebSocket Gateway with real-time event broadcasting
- [x] **Phase 2.2**: Metrics collection and competitive pricing tracking
- [x] **Phase 2.3**: Next.js Dashboard with WebSocket integration
- [x] **WebSocket CORS Fix**: Cross-origin connection support
- [x] **Real-time Event Processing**: Live auction data and bid progression
- [x] **Phase 3**: Complete Solana blockchain integration with Anchor smart contracts
- [x] **USDC Payment Processing**: Automatic token transfers for auction settlements
- [x] **Comprehensive Blockchain Testing**: 20+ test cases covering security and edge cases
- [x] **Architecture Modernization**: HTTP registration + direct communication (no multicast)

### In Progress 🔄

- [ ] **Phase 2.4**: Frontend UI/UX improvements and TailwindCSS enhancements

### Next Steps 🎯

1. **Complete Phase 2.4**: Enhance frontend with advanced styling and animations
2. **Integration Testing**: End-to-end auction flows with multiple aggregators
3. **Performance Testing**: Load testing for 1000+ messages/second
4. **Blockchain Integration**: Connect Rust backend to Solana smart contracts

## Deployment Architecture

### Hybrid Development Setup

The system uses a **hybrid deployment approach** optimized for development efficiency:

#### Local Components (Host Machine)

- **Gateway**: Runs locally on port 8080 for fast development iteration
- **Frontend**: Runs locally on port 3000 for immediate UI updates
- **Solana Validator**: Runs locally for blockchain testing

#### Docker Components (Containers)

- **BESS Nodes**: 3 containers (001, 002, 003) with different capacities
- **Aggregators**: 2 containers (001, 002) with conservative trading strategies

#### Network Configuration

- **Docker Network**: `energy-trading_energy_network` for container communication
- **Host Connectivity**: Containers connect to local gateway via host IP (`192.168.0.250:8080`)
- **Environment Variables**: `GATEWAY_HOST=192.168.0.250` for container-to-host communication

#### Benefits of Hybrid Approach

- **Fast Development**: No Docker rebuilds for gateway changes
- **Real-time Updates**: Immediate frontend and gateway updates
- **Container Isolation**: BESS nodes and aggregators remain containerized
- **Network Simplicity**: Avoids Docker Compose networking issues

#### Production Considerations

For production deployment, all components should run in containers with:

- Proper service discovery
- Load balancing
- Health checks
- Monitoring and logging

## Future Considerations

### Scalability Enhancements

- Multi-region deployment for geographic distribution
- Event-driven architecture with message queues
- Advanced caching strategies
- Horizontal scaling capabilities

### Feature Expansions

- **Battery Discharge Rate (C-Rate) Constraints**: Implement realistic battery physics
  - Add power rating (kW) to BESS node specifications
  - Enforce discharge rate limits (typically 0.25-1C for home batteries)
  - Implement power-limited auctions based on battery capacity and C-rate
  - Example: 10kWh battery @ 0.5C = 5kW maximum discharge rate
- Machine learning models for market prediction
- Integration with additional blockchain networks
- Advanced auction mechanisms (combinatorial, multi-attribute)
- Integration with smart grid infrastructure protocols

### Research Extensions

- Validation of competitive pricing benefits
- Performance analysis under various market conditions
- Economic impact assessment
- Integration with renewable energy forecasting

---

## Notes

- **Project Focus**: Demonstrate competitive pricing benefits through multiple aggregator bidding
- **Monitoring Approach**: Simple WebSocket monitoring, no enterprise monitoring tools
- **Priority**: Real-time performance and economic impact visualization
- **Timeline**: Phases 1-2 are critical for core demonstration
- **Success Criteria**: Clear proof that multiple aggregators lead to better prices for BESS owners
