# 🧪 Energy Trading Blockchain Tests

Comprehensive test suite for the Energy Trading System blockchain integration.

## 📋 Test Categories

### ✅ Happy Path Tests (`energy_trading.ts`)

- **Program Initialization**: Basic program setup and initialization
- **Successful Settlement**: Complete auction settlement with USDC payments
- **Reputation Updates**: Aggregator reputation score updates
- **Battery Stats**: BESS owner statistics updates
- **Balance Verification**: USDC balance changes validation

### ❌ Unhappy Path Tests (`energy_trading.ts`)

- **Already Settled**: Prevention of double settlement
- **Insufficient Balance**: USDC balance validation
- **Wrong Authority**: Unauthorized access prevention
- **Invalid Data**: Malformed input rejection

### 🔍 Edge Case Tests (`energy_trading.ts`)

- **Zero Values**: Zero energy amount handling
- **Maximum Values**: Overflow protection
- **Reputation Overflow**: Score capping at 100
- **Concurrent Operations**: Race condition handling

### 🚀 Performance Tests (`performance_tests.ts`)

- **Timing Requirements**: ≤500ms settlement time
- **Concurrent Load**: 100+ simultaneous settlements
- **Memory Usage**: Resource consumption limits
- **Stress Testing**: High-frequency operations

### 🔒 Security Tests (`security_tests.ts`)

- **Access Control**: Unauthorized access prevention
- **Financial Security**: USDC amount validation
- **Input Validation**: Malformed data rejection
- **State Consistency**: Data integrity maintenance

### 🔗 Integration Tests (`integration_tests.ts`)

- **End-to-End Workflow**: Complete auction lifecycle
- **Cross-Component**: USDC and system program integration
- **Data Flow**: Settlement data propagation
- **Network Integration**: Multi-network compatibility

## 🏃‍♂️ Running Tests

### Prerequisites

```bash
# Install dependencies
npm install

# Start local Solana cluster
solana-test-validator

# Build the program
anchor build
```

### Test Commands

```bash
# Run all tests
npm run test:all

# Run specific test categories
npm run test:unit          # Happy/unhappy/edge cases
npm run test:performance   # Performance and load tests
npm run test:security      # Security and attack tests
npm run test:integration   # Integration and workflow tests

# Run with coverage
npm run test:coverage

# Run in watch mode
npm run test:watch

# Run for CI/CD
npm run test:ci
```

## 📊 Test Coverage

### Critical Paths (100% Coverage Required)

- ✅ Auction settlement flow
- ✅ USDC payment processing
- ✅ Reputation score updates
- ✅ Error handling and validation
- ✅ Access control mechanisms

### Performance Benchmarks

- ⚡ Settlement time: ≤500ms
- 🔄 Concurrent operations: 100+ simultaneous
- 💾 Memory usage: ≤50MB increase
- 📈 Success rate: ≥95%

### Security Validation

- 🛡️ Access control: 100% unauthorized access blocked
- 💰 Financial security: All USDC operations validated
- 🔍 Input validation: All malformed data rejected
- 🔄 State consistency: No data corruption

## 🎯 Test Scenarios

### Happy Path Scenarios

1. **Normal Settlement**: Valid auction → USDC transfer → Reputation update
2. **Multiple Settlements**: Sequential settlements with proper state updates
3. **Reputation Growth**: Aggregator reputation increases with successful settlements

### Unhappy Path Scenarios

1. **Double Settlement**: Attempt to settle already settled auction
2. **Insufficient Funds**: Settlement attempt with insufficient USDC balance
3. **Wrong Authority**: Unauthorized user attempting settlement

### Edge Case Scenarios

1. **Zero Values**: Zero energy amount or price
2. **Maximum Values**: Maximum u64 values for overflow testing
3. **Concurrent Access**: Multiple users attempting same operation

### Security Attack Scenarios

1. **Unauthorized Access**: Wrong user attempting settlement
2. **Double Spending**: Attempting to spend same USDC twice
3. **Overflow Attacks**: Maximum values causing overflow
4. **Replay Attacks**: Replaying old transactions

## 🔧 Test Configuration

### Environment Variables

```bash
# Solana cluster URL
SOLANA_URL=http://localhost:8899

# Program ID
PROGRAM_ID=4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb

# Test timeout (ms)
TEST_TIMEOUT=1000000
```

### Test Data

- **USDC Mint**: 6 decimal places
- **Test Accounts**: Generated keypairs for each test
- **Auction Data**: Realistic energy amounts (100 kWh)
- **Price Data**: Australian FiT rates (5-15 c/kWh)

## 📈 Continuous Integration

### GitHub Actions

```yaml
name: Blockchain Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install
      - run: npm run test:ci
```

### Test Reports

- **Coverage Report**: HTML coverage report in `coverage/`
- **Test Results**: JSON results in `test-results.json`
- **Performance Metrics**: Timing and resource usage data

## 🚨 Critical Test Cases

### Must Pass (Blocking)

1. ✅ Settlement within 500ms
2. ✅ USDC balance validation
3. ✅ Unauthorized access prevention
4. ✅ Double settlement prevention
5. ✅ Reputation score accuracy

### Should Pass (Warning)

1. ⚠️ Concurrent operation handling
2. ⚠️ Memory usage optimization
3. ⚠️ Error message clarity
4. ⚠️ Event emission accuracy

### Could Pass (Info)

1. ℹ️ Performance under extreme load
2. ℹ️ Network failure recovery
3. ℹ️ Cross-network compatibility

## 🔍 Debugging Tests

### Common Issues

1. **Account Not Found**: Ensure proper account creation
2. **Insufficient Balance**: Check USDC mint and transfer setup
3. **Timeout**: Increase test timeout for slow operations
4. **Permission Denied**: Verify keypair permissions

### Debug Commands

```bash
# Run single test with verbose output
npx mocha tests/energy_trading.ts --grep "Should settle auction" --timeout 1000000

# Run with debug logging
DEBUG=* npm run test:unit

# Run with coverage and debug
npm run test:coverage -- --grep "settle"
```

## 📚 Test Documentation

### Adding New Tests

1. Create test file in `tests/` directory
2. Follow naming convention: `*_tests.ts`
3. Include comprehensive test cases
4. Add to appropriate test category
5. Update this README

### Test Best Practices

1. **Isolation**: Each test should be independent
2. **Cleanup**: Clean up test data after each test
3. **Assertions**: Use specific, meaningful assertions
4. **Error Messages**: Include helpful error messages
5. **Documentation**: Document complex test scenarios

---

**Remember**: These tests are critical for a financial system handling real USDC payments. Every test must pass before deployment to mainnet! 🚀
