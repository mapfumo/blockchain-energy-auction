# Energy Trading System

A production-ready Rust/Solana implementation of a real-time energy auction platform for Distributed Battery Energy Storage Systems (D-BESS) and energy aggregators.

## 🎯 Project Goal

Demonstrate that competitive bidding among multiple energy aggregators for D-BESS energy results in **fairer prices for BESS owners** compared to a single-utility model.

## 🏗️ Architecture

### Core Components

- **ETP Message Protocol**: Binary serialization with 14 fields, 10 message types, strict timing requirements
- **BESS Nodes**: Battery energy storage systems that evaluate bids and manage energy sales
- **Aggregator Nodes**: Energy buyers with intelligent bidding strategies
- **WebSocket Gateway**: Real-time monitoring and event broadcasting
- **Blockchain Integration**: Solana-based settlement and reputation tracking

### Technology Stack

**Backend (Rust)**

- Tokio (async runtime)
- Axum (WebSocket/REST API)
- Serde + Bincode (binary serialization)
- SQLx (PostgreSQL database)
- Solana SDK (blockchain integration)
- Tracing (structured logging)

**Frontend (Next.js/TypeScript)**

- React + TailwindCSS
- Recharts (data visualization)
- React Query (state management)
- Native WebSocket (real-time updates)

**Infrastructure**

- Docker + Docker Compose
- PostgreSQL + TimescaleDB
- Redis (caching)
- GitHub Actions (CI/CD)

## 🚀 Current Status

### ✅ Completed (Phase 1 & 2)

**ETP Message Protocol**

- [x] 14-field message structure with binary serialization
- [x] All 10 message types (Register, Query, Response, Bid, Accept, Confirm, Reject, Terminate, DeviceFailure, BESSStatus)
- [x] Timing constraints validation (≤500ms critical messages, ≤200ms device failures)
- [x] **Real Query Flow**: Query/QueryResponse events following research paper specification
- [x] **Realistic Timing**: Random 2-10 second delays between auctions (no constant querying)
- [x] Message priority handling and TTL management
- [x] Comprehensive test coverage (13 tests)

**BESS Node Implementation**

- [x] Battery energy storage system with capacity tracking
- [x] Bid evaluation logic with reserve price handling
- [x] Energy availability calculation and validation
- [x] Message generation (status, query response)
- [x] BESSNodeManager for concurrent node management
- [x] **BESSTCPServer**: Production-ready TCP server with concurrent connection handling
- [x] Comprehensive test coverage (28 tests total)

**Aggregator Node Implementation**

- [x] TCP client with intelligent bidding strategies
- [x] Historical context and price prediction
- [x] Multi-BESS bidding coordination
- [x] Bid optimization algorithms
- [x] **Enhanced Metrics**: Successful bids, total energy bought, detailed performance tracking
- [x] **Smart Rejection Logic**: Rejections based on actual energy availability
- [x] Comprehensive test coverage (6 tests)

**Network Architecture**

- [x] Multicast discovery for BESS registration
- [x] Unicast communication for bidding
- [x] Message routing and delivery guarantees
- [x] Comprehensive test coverage (4 tests)

**WebSocket Gateway**

- [x] Real-time event broadcasting
- [x] In-memory metrics collection
- [x] Competitive pricing visualization
- [x] CORS support for cross-origin WebSocket connections
- [x] Comprehensive test coverage (2 tests)

**Dashboard Frontend (Production-Ready)**

- [x] Next.js dashboard with real-time monitoring
- [x] WebSocket integration with automatic reconnection
- [x] Live auction feed with bid progression
- [x] Real-time event processing and state management
- [x] Responsive UI with TailwindCSS
- [x] **Professional UI/UX**: Custom logo, dark/light themes, keyboard shortcuts
- [x] **Advanced Features**: Node selectors, detailed popups, live events panel
- [x] **Error Handling**: Graceful error recovery with user-friendly messages
- [x] **Help System**: Interactive help modal with shortcuts guide

**Competitive Pricing System**

- [x] **Expanded Bidding Range**: 5-30¢/kWh (vs 5-15¢/kWh typical FiT rates)
- [x] **Australian Market Integration**: Realistic solar battery voltages (12V/24V/48V)
- [x] **Dynamic Pricing**: Market-driven price discovery vs fixed FiT rates
- [x] **Economic Impact**: Clear demonstration of auction system advantages

### 🔄 In Progress (Phase 3)

**Blockchain Integration**

- [ ] Solana smart contracts for settlement
- [ ] USDC/SOL payment processing
- [ ] Reputation tracking and dispute resolution

**Performance Optimization**

- [ ] React components optimization
- [ ] WebSocket performance tuning
- [ ] Mobile responsiveness enhancements

### 📋 Upcoming (Phases 4-5)

**Advanced Features**

- [ ] AI-powered pricing algorithms
- [ ] Machine learning for market analysis
- [ ] Advanced analytics and reporting
- [ ] Predictive modeling

## 🧪 Testing Strategy

**Test-Driven Development (TDD)**

- Write failing tests first (RED)
- Implement minimal code to pass (GREEN)
- Refactor and optimize (REFACTOR)

**Test Categories**

- **Unit Tests**: Individual component functionality
- **Integration Tests**: Component interaction and message flow
- **Performance Tests**: Timing constraints and throughput validation

**Current Test Coverage**

- **53 total tests passing**
- **0 failures**
- **ETP Message Protocol**: 13 tests
- **BESS Node**: 20 tests
- **Aggregator Node**: 6 tests
- **Network Architecture**: 4 tests
- **WebSocket Gateway**: 2 tests
- **BESS TCP Server**: 7 tests
- **Unit Tests**: 1 test

**Frontend Test Coverage**

- **WebSocket Integration**: Real-time connection testing
- **Event Processing**: Live data handling validation
- **UI Components**: Responsive design testing
- **Cross-Origin Support**: CORS validation
- **Competitive Pricing**: 5-30¢/kWh bidding range validation
- **Australian Market**: Realistic FiT rates and battery standards

## 🏃‍♂️ Quick Start

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- PostgreSQL 14+

### Development Setup

```bash
# Clone repository
git clone <repository-url>
cd energy-trading

# Start database
docker-compose up -d postgres redis

# Run tests
cd energy-trading-rust
cargo test

# Run specific test suites
cargo test --test etp_message_tests
cargo test --test bess_node_tests
```

### Running the System

```bash
# Start WebSocket gateway (backend)
cd energy-trading-rust
cargo run --bin gateway

# Start frontend dashboard (in new terminal)
cd frontend
npm run dev

# Run all tests
cargo test

# Run specific test suites
cargo test --test etp_message_tests
cargo test --test bess_node_tests
cargo test --test bess_tcp_server_tests
```

### Frontend Dashboard

The dashboard is now **production-ready** with:

- **Real-time Monitoring**: Live auction data and bid progression
- **Competitive Pricing**: 5-30¢/kWh bidding range demonstration
- **Professional UI**: Custom logo, themes, keyboard shortcuts
- **Advanced Features**: Node selectors, detailed popups, live events
- **Australian Integration**: Realistic FiT rates and battery standards
- **ETP Query Events**: QuerySent/QueryResponse event display and filtering
- **Enhanced Metrics**: Successful bids, total energy bought, detailed performance tracking

**Access**: http://localhost:3000 (after running `npm run dev`)

## 📊 Performance Requirements

**Timing Constraints**

- Device Failure: ≤200ms (priority 0)
- Bid Accept/Reject/Confirm: ≤500ms (priority 5)
- Query Response: ≤500ms (priority 50)
- BESSStatus: ≤2000ms (priority 60)
- Register: ≤5000ms (priority 80)

**Throughput**

- ≥1000 messages/second
- <500ms latency for critical messages
- Binary serialization <10ms for 1000 messages

## 🔬 Research Foundation

Based on the research paper: _"Communication requirements for enabling real-time energy trading among distributed energy storage systems and aggregators"_ by Antony Mapfumo (QUT).

**Key Specifications**

- 14-field ETP message structure
- 10 message types with specific priorities
- Binary serialization (122-128 bytes per message)
- Multicast discovery + unicast bidding
- Real-time performance requirements

## 📁 Project Structure

```
energy-trading/
├── energy-trading-rust/          # Rust backend ✅ COMPLETE
│   ├── src/
│   │   ├── etp_message.rs       # ETP protocol implementation
│   │   ├── bess_node.rs         # BESS node implementation
│   │   ├── aggregator_node.rs   # Aggregator implementation
│   │   ├── network/             # WebSocket gateway & TCP servers
│   │   └── error.rs             # Error handling
│   ├── tests/                   # Integration tests (53 tests passing)
│   └── Cargo.toml
├── frontend/                     # Next.js dashboard ✅ PRODUCTION-READY
│   ├── src/
│   │   ├── components/          # React components
│   │   ├── hooks/               # Custom React hooks
│   │   ├── types/               # TypeScript definitions
│   │   └── pages/               # Next.js pages
│   ├── public/                  # Static assets (logo, favicon)
│   └── package.json
├── programs/                     # Solana smart contracts (upcoming)
├── infrastructure/               # Docker & deployment (upcoming)
├── docs/                        # Documentation
│   ├── architecture.md
│   ├── requirements.md
│   └── project_overview.md
├── TODO.md                      # Project roadmap
├── STATUS.md                    # Current status overview
└── README.md                    # This file
```

## 🤝 Contributing

1. Follow TDD approach: Write tests first
2. Ensure all tests pass before submitting
3. Follow Rust best practices and error handling
4. Update documentation for new features

## 📄 License

MIT License - see LICENSE file for details

## 🎯 Success Metrics

**Technical**

- All timing requirements met
- 100% test coverage for critical paths
- Binary serialization performance validated

**Business**

- Demonstrate competitive pricing benefits
- Show price improvement vs single-utility model
- Prove economic viability for BESS owners

---

_Built with ❤️ using Rust, Solana, and Test-Driven Development_
