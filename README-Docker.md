# Energy Trading System - Docker Setup

## 🐳 Quick Start

### Prerequisites

- Docker & Docker Compose
- 8GB+ RAM (for multiple containers)

### Start the System

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

### Services

**Core Energy Trading:**

- **3x BESS Nodes**: Battery energy storage systems with different capacities and reserve prices
- **3x Aggregators**: Different bidding strategies (Intelligent, Aggressive, Conservative)
- **WebSocket Gateway**: Real-time event broadcasting on port 8080

**Infrastructure:**

- **PostgreSQL**: Database on port 5432
- **Redis**: Cache on port 6379
- **Frontend**: Next.js dashboard on port 3000

**Optional Monitoring:**

- **Grafana**: Advanced monitoring on port 3001 (use `--profile monitoring`)

### Network Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   BESS Node 1   │    │   BESS Node 2   │    │   BESS Node 3   │
│   100kWh @ $15  │    │   80kWh @ $16   │    │  120kWh @ $14.5 │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌────────────┼────────────┐
                    │            │            │
            ┌───────▼───┐  ┌─────▼─────┐  ┌───▼──────┐
            │Aggregator1│  │Aggregator2│  │Aggregator3│
            │Intelligent│  │Aggressive │  │Conservative│
            └───────┬───┘  └─────┬─────┘  └───┬──────┘
                    │            │            │
                    └────────────┼────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │   WebSocket Gateway     │
                    │      Port 8080          │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │   Frontend Dashboard    │
                    │      Port 3000          │
                    └─────────────────────────┘
```

### Real-Time Monitoring

**WebSocket Events:**

- `AuctionStarted`: New energy auction begins
- `BidPlaced`: Aggregator places bid
- `BidAccepted`: BESS accepts bid
- `BidRejected`: BESS rejects bid
- `SystemMetrics`: Live performance data

**Dashboard Features:**

- Live auction visualization
- BESS node energy levels
- Aggregator bidding strategies
- Price improvement tracking
- Network performance metrics

### Testing the System

```bash
# Run all tests
docker-compose exec gateway cargo test

# Run specific test suites
docker-compose exec gateway cargo test --test network_architecture_tests
docker-compose exec gateway cargo test --test aggregator_node_tests
docker-compose exec gateway cargo test --test bess_node_tests

# View system logs
docker-compose logs gateway
docker-compose logs aggregator-1
docker-compose logs bess-node-1
```

### Performance Testing

```bash
# Load test with multiple aggregators
docker-compose up --scale aggregator-1=3 --scale aggregator-2=3

# Monitor performance
docker stats
```

### Troubleshooting

**Common Issues:**

1. **Port conflicts**: Change ports in docker-compose.yml
2. **Memory issues**: Increase Docker memory limit
3. **Network issues**: Check if ports 8080, 3000 are available

**Logs:**

```bash
# View all logs
docker-compose logs

# View specific service logs
docker-compose logs gateway
docker-compose logs postgres
```

### Development

**Hot Reload:**

```bash
# For development with hot reload
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

**Database Access:**

```bash
# Connect to PostgreSQL
docker-compose exec postgres psql -U energy_user -d energy_trading
```

### Production Deployment

**Environment Variables:**

```bash
# Set production environment
export RUST_LOG=warn
export DATABASE_URL=postgresql://energy_user:energy_pass@postgres:5432/energy_trading
export REDIS_URL=redis://redis:6379
```

**Scaling:**

```bash
# Scale aggregators for load testing
docker-compose up --scale aggregator-1=5 --scale aggregator-2=5
```

---

**Ready to trade energy!** 🚀

The system demonstrates competitive pricing benefits through real-time bidding between multiple aggregators and BESS nodes, with live monitoring via WebSocket events.
