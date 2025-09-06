# Intelligent Bid Rejection System

## Overview

The Energy Trading System implements a sophisticated bid rejection system that simulates real-world energy market constraints and safety requirements. This system ensures BESS nodes maintain operational reserves while allowing competitive bidding.

## Rejection Logic Hierarchy

The system evaluates bids in a specific order, rejecting at the first condition met:

```
1. Price Check: bid_price < 8.0¢/kWh → "Bid price below reserve price"
2. Capacity Check: energy_requested > available_energy → "BESS node capacity exceeded"
3. Safety Buffer: energy_requested > (available_energy - 0.5) → "BESS node capacity exceeded"
4. General Check: energy_requested > 10.0 kWh → "Insufficient energy available"
5. Default: → "Bid not competitive enough"
```

## Rejection Reasons Explained

### 1. "Bid price below reserve price"

- **Trigger**: Bid price is less than 8¢/kWh
- **Purpose**: Ensures minimum viable pricing for BESS owners
- **Real-world equivalent**: Below market rate protection

### 2. "BESS node capacity exceeded"

- **Trigger**: Energy request is within 0.5 kWh of available energy
- **Purpose**: Maintains safety buffer for BESS operations
- **Real-world equivalent**: Operational reserve requirements

### 3. "Insufficient energy available"

- **Trigger**: Energy request exceeds available energy
- **Purpose**: Prevents over-commitment of energy resources
- **Real-world equivalent**: Physical energy constraints

### 4. "Bid not competitive enough"

- **Trigger**: All other checks pass but bid isn't competitive
- **Purpose**: Encourages better pricing in competitive market
- **Real-world equivalent**: Market competition dynamics

## Example Scenarios

### Scenario 1: Safety Buffer Trigger

- **BESS State**: 6.1 kWh available
- **Aggregator Request**: 6.0 kWh at 15.0¢/kWh
- **Result**: "BESS node capacity exceeded"
- **Reason**: Request too close to available energy (within 0.5 kWh buffer)

### Scenario 2: Price Below Reserve

- **BESS State**: 8.1 kWh available
- **Aggregator Request**: 5.0 kWh at 7.5¢/kWh
- **Result**: "Bid price below reserve price"
- **Reason**: Price below minimum 8¢/kWh threshold

### Scenario 3: Competitive Bidding

- **BESS State**: 12.1 kWh available
- **Aggregator Request**: 7.0 kWh at 25.1¢/kWh
- **Result**: "Bid not competitive enough"
- **Reason**: Price acceptable but not competitive enough

## Safety Buffer Logic

The 0.5 kWh safety buffer serves several purposes:

1. **Operational Reserve**: BESS nodes need energy for their own operations
2. **System Stability**: Prevents complete energy depletion
3. **Realistic Trading**: Mirrors real-world energy market practices
4. **Emergency Capacity**: Maintains reserve for critical loads

## Implementation Details

### Code Location

- **File**: `energy-trading-rust/src/bin/gateway.rs`
- **Function**: Auction evaluation logic (lines 183-194)
- **Key Variables**:
  - `available_energy`: Current BESS energy level
  - `energy_requested`: Aggregator's energy request
  - `bid_price`: Aggregator's bid price

### Debug Logging

The system includes comprehensive debug logging:

```
❌ Rejecting Aggregator 4: BESS node capacity exceeded (bid: 17.1¢/kWh, requested: 6.0 kWh, available: 6.1 kWh)
```

## Benefits

1. **Realistic Simulation**: Mirrors actual energy market constraints
2. **Educational Value**: Shows why bids get rejected in real markets
3. **System Safety**: Prevents BESS nodes from being completely drained
4. **Competitive Dynamics**: Encourages better pricing strategies
5. **Operational Reserves**: Maintains system stability

## Future Enhancements

- **Dynamic Buffer Sizing**: Adjust safety buffer based on BESS status
- **Market Conditions**: Modify rejection thresholds based on market state
- **BESS Preferences**: Allow BESS nodes to set custom rejection criteria
- **Historical Learning**: Use past rejection data to improve logic
