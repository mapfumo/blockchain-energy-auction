# Energy Trading System

A production-ready Rust/Solana implementation of a real-time energy auction platform for Distributed Battery Energy Storage Systems (D-BESS) and energy aggregators.

## 🎯 Project Goal

Demonstrate that competitive bidding among multiple energy aggregators for D-BESS energy results in **fairer prices for BESS owners** compared to a single-utility model.

## ✅ Current Status

### 🎉 **WebSocket Connection Issues RESOLVED (2024-09-08)**

**Status**: ✅ **RESOLVED** - WebSocket connection working perfectly

**Technical Status**:

- ✅ **Gateway**: Healthy and broadcasting real-time events
- ✅ **WebSocket Server**: Working perfectly with proper connection lifecycle handling
- ✅ **CORS Configuration**: Properly configured
- ✅ **Frontend Connection**: Fixed callback dependency issues, now connecting successfully
- ✅ **Real-time Data**: BESSNodeStatus, AuctionCompleted, and SystemMetrics events flowing

## ✨ Latest Features

### 🚀 **NEW: Complete Blockchain Integration (2024)**

- **Real-time Blockchain Settlements**: Live auction settlements with Solana transaction links
- **Test-Driven Development**: Comprehensive TDD implementation for blockchain components
- **Docker Containerization**: Full system containerization with Ubuntu 22.04 compatibility
- **WebSocket Stability**: Enhanced error handling and reconnection logic
- **React Performance**: Fixed warnings and optimized component rendering

### 🔧 **Enhanced System Features**

- **Enhanced Simulation Timing**: Realistic 5-10 second delays between aggregator bids
- **Critical Energy Management**: Emergency 5%/second recharge when BESS energy drops below 10%
- **Sequential Bidding**: Aggregators wait between actions for more realistic simulation pace
- **Smart Rejection Logic**: Intelligent bid evaluation with capacity and safety constraints
- **Real-time Energy Events**: Live monitoring of energy depletion and recharge cycles
- **High Contrast UI**: Colorful auction details (blue/green/purple/orange) for excellent readability
- **PostgreSQL Database**: Real data persistence with automatic migrations and graceful fallback

## 📚 Documentation

Detailed technical documentation is available in the [`docs/`](./docs/) folder:

- **[Rejection Logic](./docs/REJECTION_LOGIC.md)** - Intelligent bid evaluation system
- **[Simulation Timing](./docs/SIMULATION_TIMING.md)** - Enhanced timing and delay systems
- **[Database Implementation](./docs/DATABASE_IMPLEMENTATION.md)** - PostgreSQL integration and persistence
- **[Documentation Index](./docs/README.md)** - Complete documentation overview

## 🏗️ Architecture

### Core Components

- **ETP Message Protocol**: Binary serialization with 14 fields, 10 message types, strict timing requirements
- **BESS Nodes**: Battery energy storage systems that evaluate bids and manage energy sales
- **Aggregator Nodes**: Energy buyers with intelligent bidding strategies
- **WebSocket Gateway**: Real-time monitoring and event broadcasting
- **Blockchain Integration**: Solana-based settlement, USDC/SOL payments, and reputation tracking

### Technology Stack

**Backend (Rust)**

- Tokio (async runtime)
- Axum (WebSocket/REST API)
- Serde + Bincode (binary serialization)
- In-memory data structures
- Solana SDK (blockchain integration)
- Tracing (structured logging)

**Frontend (Next.js/TypeScript)**

- React + TailwindCSS
- Recharts (data visualization)
- React Query (state management)
- Native WebSocket (real-time updates)

**Infrastructure**

- Docker + Docker Compose
- In-memory data storage (no database)
- GitHub Actions (CI/CD)

**Blockchain (Solana)**

- Anchor Framework (Rust smart contracts)
- Solana SDK (Rust client integration)
- USDC/SOL Token Support
- Wallet Integration (Phantom, Solflare)
- Multi-network Deployment (localnet/devnet/mainnet)

## 🔗 Blockchain Integration

### Smart Contract Architecture

**Settlement Contract**

```rust
#[program]
pub mod energy_trading {
    pub fn settle_auction(
        ctx: Context<SettleAuction>,
        auction_id: u64,
        energy_amount: u64,
        final_price: u64,
    ) -> Result<()> {
        // Transfer USDC from aggregator to BESS owner
        // Update reputation scores
        // Emit settlement event
    }
}
```

**Key Features**

- **Immutable Settlement**: All energy trades recorded on-chain
- **Automatic Payments**: USDC transferred directly to BESS owner wallets
- **Reputation System**: On-chain performance tracking for aggregators
- **Dispute Resolution**: Smart contract hooks for automated conflict resolution
- **Multi-token Support**: USDC (primary) and SOL (secondary) payments

### Integration Flow

1. **Auction Completion** → Generate settlement transaction
2. **Wallet Connection** → Connect aggregator and BESS owner wallets
3. **Smart Contract Call** → Submit settlement to Solana blockchain
4. **Payment Processing** → USDC automatically transferred
5. **Database Update** → Store blockchain transaction hash
6. **Event Monitoring** → Listen for on-chain settlement events

### Benefits

**For BESS Owners**

- Immutable transaction records
- Direct USDC payments to wallet
- Transparent pricing history

**For Aggregators**

- On-chain reputation building
- Trustless trading environment
- Competitive advantage through performance

**For System**

- Decentralized architecture
- Complete audit trail
- High-throughput settlement

## 🚀 Current Status

### ✅ Completed (Phase 1 & 2)

**ETP Message Protocol**

- [x] 14-field message structure with binary serialization
- [x] All 10 message types (Register, Query, Response, Bid, Accept, Confirm, Reject, Terminate, DeviceFailure, BESSStatus)
- [x] Timing constraints validation (≤500ms critical messages, ≤200ms device failures)
- [x] **Real Query Flow**: Query/QueryResponse events following research paper specification
- [x] **Energy Management**: BESS energy depletion and recharge simulation with smart pricing
- [x] **Enhanced Recharge**: Critical 5%/second recharge when energy drops below 10%
- [x] **Realistic Timing**: Random 2-10 second delays between auctions (no constant querying)
- [x] **Bid Delays**: Random 5-10 second delays between aggregator bids for realistic simulation
- [x] **Smart Rejections**: Intelligent bid evaluation with capacity and safety constraints
- [x] **Energy Events**: EnergyDepleted and EnergyRecharged events for real-time monitoring
- [x] Message priority handling and TTL management
- [x] Comprehensive test coverage (13 tests)

**BESS Node Implementation**

- [x] Battery energy storage system with capacity tracking
- [x] Bid evaluation logic with reserve price handling
- [x] Energy availability calculation and validation
- [x] Message generation (status, query response)
- [x] BESSNodeManager for concurrent node management
- [x] **BESSTCPServer**: Production-ready TCP server with concurrent connection handling
- [x] **Smart Rejection Logic**: Hierarchical bid evaluation system (see `docs/REJECTION_LOGIC.md`)
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

- [x] HTTP registration for BESS and aggregator discovery
- [x] Direct TCP communication for bidding (no multicast)
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

### ✅ Completed (Phase 3)

**Blockchain Integration**

- [x] **Solana Smart Contracts**: Complete Anchor framework implementation with comprehensive testing
- [x] **Payment Processing**: USDC automatic token transfers with balance validation
- [x] **Auction Settlement**: Immutable transaction records with event emission
- [x] **Reputation Tracking**: On-chain performance scoring with overflow protection
- [x] **Security Features**: Comprehensive error handling and access control
- [x] **Multi-network Support**: localnet, devnet, mainnet deployment ready
- [x] **Comprehensive Testing**: 20+ test cases covering security, edge cases, and performance
- [x] **Account Management**: PDA-based account creation with authority validation
- [x] **Event Monitoring**: Settlement event emission for real-time monitoring

### 🔄 In Progress (Phase 4)

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

## 🚀 Getting Started

### System Requirements

**Minimum Requirements:**

- **OS**: Linux (Ubuntu 20.04+), macOS (10.15+), or Windows 10+ with WSL2
- **RAM**: 8GB minimum, 16GB recommended
- **CPU**: 4 cores minimum, 8 cores recommended
- **Storage**: 10GB free space
- **Network**: Ports 3000, 8080, 8899, 8900 available

### Prerequisites Installation

#### 1. Install Rust (Required)

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell or run:
source ~/.cargo/env

# Verify installation
rustc --version  # Should be 1.70+
cargo --version
```

#### 2. Install Docker & Docker Compose (Required)

**Ubuntu/Debian:**

```bash
# Update package index
sudo apt update

# Install Docker
sudo apt install -y docker.io docker-compose

# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, then verify
docker --version
docker-compose --version
```

**macOS:**

```bash
# Install using Homebrew
brew install --cask docker

# Or download Docker Desktop from: https://www.docker.com/products/docker-desktop
```

**Windows:**

- Download Docker Desktop from: https://www.docker.com/products/docker-desktop
- Enable WSL2 integration if using WSL2

#### 3. Install Node.js 18+ (Required)

**Ubuntu/Debian:**

```bash
# Install Node.js 18 LTS
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node --version  # Should be 18+
npm --version
```

**macOS:**

```bash
# Install using Homebrew
brew install node@18

# Or download from: https://nodejs.org/
```

**Windows:**

- Download from: https://nodejs.org/ (LTS version)

#### 4. Install Solana CLI (Required)

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation
solana --version  # Should be 1.17.0+
```

#### 5. Install Anchor Framework (Required)

```bash
# Install Anchor CLI
npm install -g @coral-xyz/anchor-cli

# Verify installation
anchor --version  # Should be 0.31.1+
```

### First-Time Setup

#### 1. Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd energy-trading

# Build Rust components
cd simple-gateway
cargo build --release
cd ..

# Install frontend dependencies
cd frontend
npm install
cd ..

# Build Docker images
docker-compose build
```

#### 2. Generate Solana Keypair

```bash
# Generate a new keypair for testing
solana-keygen new --outfile ~/.config/solana/id.json

# Set Solana cluster to localnet
solana config set --url localhost

# Verify configuration
solana config get
```

#### 3. Start Solana Validator

```bash
# Start Solana test validator (in background)
solana-test-validator --reset --quiet &

# Wait for validator to start
sleep 5

# Check validator status
solana cluster-version
```

### Running the System

#### Option 1: Automated Setup (Recommended)

```bash
# Start everything automatically
./start.sh
```

This script will:

- Detect Docker gateway IP
- Start Solana validator
- Start gateway service
- Start frontend dashboard
- Start BESS nodes and aggregators
- Display system status

#### Option 2: Manual Setup

```bash
# Terminal 1: Start Solana validator
solana-test-validator --reset --quiet &

# Terminal 2: Start gateway
cd simple-gateway
cargo run &

# Terminal 3: Start frontend
cd frontend
npm run dev &

# Terminal 4: Start BESS nodes and aggregators
docker-compose up -d bess-001 bess-002 bess-003 aggregator-001 aggregator-002
```

### Access the System

Once running, access these URLs:

- **🎨 Frontend Dashboard**: http://localhost:3000
- **🔧 Gateway API**: http://localhost:8080
- **📊 Health Check**: http://localhost:8080/health
- **🔗 Solana Explorer**: http://localhost:8899
- **📈 Solana RPC**: http://localhost:8899

### Verification

#### Check System Status

```bash
# Check all services
curl http://localhost:8080/health
curl http://localhost:3000
curl http://localhost:8899

# Check Docker containers
docker ps

# Check Solana validator
solana cluster-version
```

#### Expected Output

- **Frontend**: Should show "Messages received: X" (increasing number)
- **Gateway**: Should return `{"status":"healthy","timestamp":"..."}`
- **Solana**: Should return cluster version (e.g., "1.17.0")

### Troubleshooting

#### Common Issues

**1. Port Already in Use**

```bash
# Check what's using the port
sudo lsof -i :3000
sudo lsof -i :8080
sudo lsof -i :8899

# Kill the process
sudo kill -9 <PID>
```

**2. Docker Permission Denied**

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in
# Or run: newgrp docker
```

**3. Solana Validator Won't Start**

```bash
# Check if port 8899 is free
sudo lsof -i :8899

# Kill any existing validator
pkill -f solana-test-validator

# Start fresh
solana-test-validator --reset --quiet
```

**4. Frontend Won't Connect to WebSocket**

```bash
# Check gateway logs
docker logs energy-gateway

# Check if gateway is healthy
curl http://localhost:8080/health

# Restart gateway
docker restart energy-gateway
```

**5. BESS Nodes Not Connecting**

```bash
# Check Docker network
docker network ls
docker network inspect energy-trading_energy_network

# Check container logs
docker logs bess-001
docker logs aggregator-001
```

### Network Configuration

The system uses these network configurations:

- **Frontend**: `localhost:3000` → `localhost:8080` (WebSocket)
- **BESS Nodes**: Docker network → `gateway:8080` (HTTP)
- **Aggregators**: Docker network → `gateway:8080` (HTTP)
- **Solana**: `localhost:8899` (RPC), `localhost:8900` (WebSocket)

### Development Workflow

```bash
# Run tests
cargo test

# Run specific test suites
cargo test --test etp_message_tests
cargo test --test bess_node_tests
cargo test --test bess_tcp_server_tests

# Frontend development
cd frontend
npm run dev
npm run build
npm run test

# Blockchain development
cd energy_trading
anchor test
anchor build
anchor deploy
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
├── energy_trading/               # Solana smart contracts ✅ COMPLETE
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

## 🔋 Production Considerations

**Battery Discharge Rate (C-Rate) Constraints**

- **Current System**: Simplified energy trading without power limitations
- **Production Requirement**: Implement C-rate constraints for realistic battery behavior
  - Add power rating (kW) to BESS node specifications
  - Enforce discharge rate limits (typically 0.25-1C for home batteries)
  - Implement power-limited auctions based on battery capacity and C-rate
  - Example: 10kWh battery @ 0.5C = 5kW maximum discharge rate
- **Impact**: More realistic energy trading with proper battery physics constraints

---

_Built with ❤️ using Rust, Solana, TypeScript and Test-Driven Development_
