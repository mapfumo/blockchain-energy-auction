# Enhanced Simulation Timing System

## Overview

The Energy Trading System implements realistic timing delays to create an educational and observable simulation that mirrors real-world energy trading dynamics.

## Timing Components

### 1. Auction Cycle Delays

- **Query Phase**: Random 2-10 second delays between auctions
- **Purpose**: Prevents constant querying, simulates realistic market intervals
- **Implementation**: Random delay calculation based on auction ID

### 2. Bid Placement Delays

- **Duration**: Random 5-10 second delays between aggregator bids
- **Purpose**: Creates sequential bidding for better observation
- **Implementation**: Per-aggregator delay calculation

### 3. Rejection Response Delays

- **Duration**: Random 2-5 second delays after bid rejections
- **Purpose**: Simulates aggregator processing time after rejection
- **Implementation**: Shorter delays for faster recovery

### 4. Critical Recharge Timing

- **Normal Recharge**: 0.05 kWh/second (simulated solar charging)
- **Critical Recharge**: 0.75 kWh/second (5% of 15kWh capacity per second)
- **Trigger**: When BESS energy drops below 10% capacity
- **Purpose**: Emergency energy recovery for depleted batteries

## Implementation Details

### Code Location

- **File**: `energy-trading-rust/src/bin/gateway.rs`
- **Key Functions**:
  - Auction cycle timing (lines 120-130)
  - Bid placement delays (lines 210-215)
  - Rejection delays (lines 220-225)
  - Critical recharge logic (lines 140-150)

### Delay Calculations

#### Auction Cycle Delays

```rust
let auction_delay = 2.0 + ((auction_id * 7) as f64 * 0.23) % 8.0; // 2-10 seconds
```

#### Bid Placement Delays

```rust
let bid_delay = 5.0 + ((auction_id * 7 + i as u64 * 13) as f64 * 0.23) % 5.0; // 5-10 seconds
```

#### Rejection Delays

```rust
let rejection_delay = 2.0 + ((auction_id * 11 + i as u64 * 19) as f64 * 0.31) % 3.0; // 2-5 seconds
```

#### Critical Recharge Rate

```rust
let recharge_rate = if energy_percentage < 10.0 {
    0.75 // 5% of 15kWh = 0.75 kWh per second (fast recharge when critical)
} else {
    0.05 // 0.05 kWh per second (normal solar charging)
};
```

## Timing Benefits

### 1. Educational Value

- **Observable Process**: Users can follow the auction flow step-by-step
- **Realistic Pacing**: Mirrors actual energy market timing
- **Learning Opportunity**: Shows how energy trading works in practice

### 2. System Stability

- **Prevents Overload**: Avoids constant high-frequency operations
- **Resource Management**: Reduces CPU and network usage
- **Smooth Operation**: Creates predictable system behavior

### 3. User Experience

- **Better Monitoring**: Easier to track individual events
- **Clear Progression**: Sequential bidding is easier to follow
- **Realistic Feel**: Creates authentic energy trading atmosphere

## Timing Scenarios

### Normal Auction Cycle

1. **Query Phase**: 2-10 seconds delay between auctions
2. **Bidding Phase**: 5-10 seconds between each aggregator bid
3. **Evaluation Phase**: Immediate bid evaluation
4. **Response Phase**: 2-5 seconds delay after rejections

### Critical Energy Scenario

1. **BESS Depletion**: Energy drops below 10%
2. **Emergency Recharge**: 5% capacity per second (0.75 kWh/sec)
3. **Status Update**: EnergyDepleted event broadcast
4. **Recovery**: Normal recharge resumes above 10%

## Configuration

### Adjustable Parameters

- **Auction Delays**: Modify range in auction cycle logic
- **Bid Delays**: Adjust aggregator waiting times
- **Rejection Delays**: Change post-rejection processing time
- **Recharge Rates**: Modify normal and critical recharge speeds

### Environment Variables

```bash
# Optional: Override default timing (future enhancement)
AUCTION_DELAY_MIN=2
AUCTION_DELAY_MAX=10
BID_DELAY_MIN=5
BID_DELAY_MAX=10
```

## Monitoring and Debugging

### Log Output Examples

```
⏳ Aggregator 1 waiting 9.6 seconds before next action...
⏳ Aggregator 4 waiting 5.0 seconds after rejection...
🔋 BESS Node 101 critical recharge: +0.75 kWh (5.1% → 10.1%)
```

### WebSocket Events

- **Timing Events**: Delays are logged but not broadcast
- **Energy Events**: Critical recharge triggers EnergyRecharged events
- **Bid Events**: All timing affects bid placement and evaluation

## Future Enhancements

### 1. Dynamic Timing

- **Market Conditions**: Adjust delays based on market activity
- **BESS Status**: Modify timing based on energy levels
- **Aggregator Strategy**: Different delays for different strategies

### 2. User Controls

- **Speed Settings**: Allow users to adjust simulation speed
- **Pause/Resume**: Control simulation flow
- **Step Mode**: Single-step through auction cycles

### 3. Advanced Metrics

- **Timing Analytics**: Track delay patterns and effectiveness
- **Performance Impact**: Measure timing effects on system performance
- **User Engagement**: Analyze how timing affects user experience
