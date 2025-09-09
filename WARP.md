# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a **decentralized energy marketplace** built with Rust/Solana that enables real-time energy auctions between Battery Energy Storage Systems (BESS) and energy aggregators, with blockchain settlement. The system demonstrates that competitive bidding among multiple aggregators results in better prices for BESS owners compared to single-utility models.

## Core Architecture

The system uses a **hybrid deployment approach** with multiple interconnected components:

### Key Components
- **simple-gateway/** - Main Rust WebSocket gateway (runs locally on port 8080)
- **frontend/** - Next.js dashboard (runs locally on port 3000)
- **energy-trading-rust/** - Core Rust backend with ETP protocol implementation
- **energy_trading/** - Solana smart contracts (Anchor framework)
- **containers/** - Docker containers for BESS nodes and aggregators
- **energy-trading-golang/** - Alternative Go implementation (legacy)

### Network Architecture
- **Local Services**: Gateway (8080), Frontend (3000), Solana validator (8899)
- **Docker Network**: BESS nodes and aggregators communicate via `energy-trading_energy_network`
- **Container-to-Host**: Containers connect to local gateway using detected Docker bridge IP

## Essential Commands

### System Startup (Recommended)
```bash
# Automated startup (detects Docker IP automatically)
./start.sh
```

### Manual Development Commands

#### Build & Run Core Services
```bash
# Build Rust components
cd simple-gateway && cargo build --release

# Install frontend dependencies
cd frontend && npm install

# Start Solana validator (required for blockchain features)
solana-test-validator --reset --quiet &

# Start gateway (main orchestrator)
cd simple-gateway && cargo run &

# Start frontend dashboard
cd frontend && npm run dev &

# Start containerized nodes
docker-compose up -d bess-001 bess-002 bess-003 aggregator-001 aggregator-002
```

#### Testing Commands
```bash
# Run Rust tests (53 passing tests)
cargo test

# Run specific test suites
cargo test --test etp_message_tests
cargo test --test bess_node_tests
cargo test --test bess_tcp_server_tests

# Frontend testing
cd frontend && npm run test
cd frontend && npm run test:coverage

# Blockchain tests (Anchor/Solana)
cd energy_trading && anchor test
```

#### Development Utilities
```bash
# Check system health
curl http://localhost:8080/health

# View running containers
docker ps

# Monitor gateway logs
docker logs energy-gateway

# Check Solana validator status
solana cluster-version

# Build Docker images
docker-compose build
```

### Individual Component Commands

#### Blockchain Development
```bash
cd energy_trading

# Build smart contracts
anchor build

# Deploy to localnet
anchor deploy

# Run comprehensive blockchain tests
anchor test
```

#### Gateway Development
```bash
cd simple-gateway

# Run with debug logging
RUST_LOG=debug cargo run

# Build release binary
cargo build --release
```

#### Frontend Development
```bash
cd frontend

# Development with hot reload
npm run dev

# Production build
npm run build && npm run start

# Type checking
npm run type-check
```

## System Architecture Deep Dive

### Message Protocol (ETP)
The system implements a **14-field Energy Trading Protocol** with strict timing requirements:
- **Binary serialization** (122-128 bytes per message)  
- **10 message types** (Register, Query, Response, Bid, Accept, Confirm, Reject, Terminate, DeviceFailure, BESSStatus)
- **Timing constraints**: ≤500ms for critical messages, ≤200ms for device failures
- **Priority-based processing** with circuit breaker patterns

### Real-time Auction Flow
1. **Registration**: BESS nodes and aggregators register with gateway via HTTP
2. **Discovery**: Gateway provides device lists for direct TCP communication  
3. **Query Phase**: Aggregators query BESS availability using ETP protocol
4. **Bidding Phase**: Multiple aggregators submit competitive bids
5. **Evaluation**: BESS nodes evaluate bids against reserve prices
6. **Settlement**: Winning bids trigger blockchain settlement via Solana
7. **Monitoring**: All events broadcast via WebSocket for real-time dashboard

### Blockchain Integration (Solana)
- **Smart Contracts**: Anchor-based programs for immutable settlement
- **Payment Processing**: Automatic USDC transfers between wallets
- **Reputation System**: On-chain performance tracking
- **Multi-network Support**: Localnet, devnet, mainnet deployment ready

### Data Flow Patterns
- **WebSocket Broadcasting**: Real-time events to frontend dashboard
- **Direct TCP Communication**: BESS ↔ Aggregator bidding after HTTP registration
- **In-memory Storage**: HashMap-based state management (no external database required)
- **Event-driven Architecture**: Async message processing with Tokio runtime

## Test-Driven Development (TDD) Approach

This project follows strict TDD methodology due to critical timing requirements and complex message protocols:

### Test Categories
- **Unit Tests**: Message protocol validation, bid evaluation logic
- **Integration Tests**: End-to-end auction flows, competitive pricing scenarios  
- **Performance Tests**: ≤500ms timing validation, 1000+ msg/sec throughput
- **Blockchain Tests**: Smart contract security, settlement validation

### Running Tests by Category
```bash
# All tests (53 passing)
cargo test

# Message protocol tests
cargo test etp_message

# BESS node functionality
cargo test bess_node

# Network communication
cargo test network

# Performance validation
cargo test --release performance
```

## Performance Requirements

### Critical Metrics
- **Message Processing**: ≤500ms for critical messages (Bid Accept/Reject/Confirm)
- **Throughput**: 1000+ messages/second
- **WebSocket Latency**: <100ms for real-time dashboard updates
- **Blockchain Settlement**: <10s confirmation time on Solana

### Load Testing Validation
```bash
# Performance tests under load
cargo test --release test_critical_message_timing_under_load
cargo test --release test_message_throughput_1000_per_second

# Concurrent connection testing
cargo test --release test_concurrent_aggregator_connections
```

## Key Implementation Files

### Core Rust Implementation
- `energy-trading-rust/src/etp_message.rs` - Binary message protocol
- `energy-trading-rust/src/bess_node.rs` - Battery energy storage system logic
- `energy-trading-rust/src/aggregator_node.rs` - Energy buyer/bidding strategies
- `energy-trading-rust/src/network/` - WebSocket gateway and TCP servers
- `energy-trading-rust/src/bess_tcp_server.rs` - Direct BESS communication

### WebSocket Gateway  
- `simple-gateway/src/main.rs` - Main gateway application
- `simple-gateway/src/blockchain.rs` - Solana integration
- `simple-gateway/src/lib.rs` - Event broadcasting and client management

### Frontend Dashboard
- `frontend/src/components/` - React components for real-time monitoring
- `frontend/src/hooks/` - Custom WebSocket hooks and state management
- `frontend/src/types/` - TypeScript definitions for ETP messages

### Blockchain Smart Contracts
- `energy_trading/programs/` - Anchor smart contracts
- `energy_trading/tests/` - Comprehensive blockchain test suite

### Container Orchestration
- `containers/bess-node/` - Dockerized BESS node implementations
- `containers/aggregator/` - Dockerized aggregator implementations  
- `docker-compose.yml` - Multi-container deployment configuration

## Common Development Patterns

### Adding New ETP Message Types
1. Update `ETPMessage` struct in `etp_message.rs`
2. Add serialization/deserialization logic
3. Implement timing constraints validation
4. Add comprehensive test coverage
5. Update WebSocket event broadcasting

### Implementing New Bidding Strategies
1. Extend `AggregatorNode` in `aggregator_node.rs`
2. Add strategy-specific bid evaluation logic
3. Implement historical performance tracking
4. Add competitive analysis features
5. Write integration tests with multiple aggregators

### Adding Blockchain Settlement Features
1. Update Anchor program in `energy_trading/programs/`
2. Add new instruction handlers
3. Update client integration in `simple-gateway/src/blockchain.rs`
4. Implement event monitoring
5. Add comprehensive security tests

## Network Configuration

### Port Usage
- **8080**: WebSocket gateway (HTTP/WebSocket API)
- **3000**: Frontend dashboard (Next.js development)
- **8899**: Solana validator (RPC endpoint)
- **8900**: Solana validator (WebSocket endpoint)
- **5432**: PostgreSQL (optional, not actively used)

### Docker Network Setup
- **Network**: `energy-trading_energy_network` (bridge driver)
- **Gateway Access**: Containers use detected Docker bridge IP for host communication
- **Environment Variables**: `GATEWAY_HOST` automatically configured by `start.sh`

## Critical System Requirements

### Prerequisites
- **Rust**: 1.75+ with Cargo
- **Node.js**: 18+ with npm
- **Docker**: Latest with Docker Compose
- **Solana CLI**: 1.17.0+ for blockchain features
- **Anchor CLI**: 0.31.1+ for smart contract development

### System Resources
- **Minimum**: 8GB RAM, 4 CPU cores, 10GB storage
- **Recommended**: 16GB RAM, 8 CPU cores, 20GB storage
- **Network**: Ports 3000, 8080, 8899, 8900 available

## Troubleshooting Common Issues

### Frontend Node Name Display Issues

**Problem**: Dashboard shows "BESS Node undefined" or "Aggregator undefined" in live events, or "BESS-Undefined"/"AGG-AGG-002" in dropdowns.

**Solution**: This has been resolved with enhanced frontend validation:

```bash
# Verify frontend build is successful
cd frontend && npm run build

# Check browser console for JavaScript errors
# Look for device_id validation warnings

# Restart frontend with latest fixes
cd frontend && npm run dev
```

**Root Causes Fixed**:
- Duplicate prefix handling (AGG-AGG-002 → AGG-002)
- Missing device_id validation and fallback logic  
- Inconsistent field extraction from WebSocket events
- Enhanced TypeScript interfaces for flexible data formats

### WebSocket Connection Problems
```bash
# Check gateway status
curl http://localhost:8080/health

# Verify Docker network connectivity
docker exec bess-001 ping gateway

# Restart gateway with fresh state
cd simple-gateway && cargo run
```

### Container Communication Issues
```bash
# Detect Docker bridge IP
docker run --rm alpine ip route show default

# Update docker-compose configuration
./start.sh  # Auto-detects and updates IP

# Check container logs
docker logs bess-001
docker logs aggregator-001
```

### Solana Validator Problems
```bash
# Kill existing validator
pkill -f solana-test-validator

# Start fresh validator
solana-test-validator --reset --quiet

# Verify validator health
solana cluster-version
```

### Performance Degradation
```bash
# Run performance tests
cargo test --release performance

# Check timing constraints
cargo test test_critical_message_timing_under_load

# Monitor system resources
docker stats
```

## Production Considerations

### Battery Physics Constraints
The current system uses simplified energy trading. For production deployment:

- **Implement C-Rate Limits**: Add power rating (kW) constraints to BESS specifications
- **Realistic Discharge Times**: Calculate minimum discharge duration based on battery capacity
- **Power-Limited Auctions**: Enforce discharge rate limits (typically 0.25-1C for home batteries)

Example: 10kWh battery @ 0.5C = 5kW maximum discharge rate = 2-hour minimum discharge time

### Scalability Enhancements
- **Load Balancing**: Multiple WebSocket gateway instances
- **State Synchronization**: Redis for shared session management  
- **Database Migration**: PostgreSQL + TimescaleDB for persistence
- **Multi-region Deployment**: Geographic distribution for lower latency

This system demonstrates competitive energy pricing through multi-aggregator auctions, validated by comprehensive testing and real-time monitoring capabilities.
