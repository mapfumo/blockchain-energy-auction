# Energy Trading Dashboard

A real-time dashboard for monitoring energy trading auctions between BESS (Battery Energy Storage Systems) nodes and aggregators.

## Features

- **Live Auctions**: Real-time visualization of energy trading auctions
- **BESS Node Map**: Monitor battery energy levels, health, and availability
- **Price Analytics**: Charts and statistics for competitive pricing analysis
- **System Metrics**: Health monitoring and performance metrics
- **WebSocket Integration**: Real-time updates via WebSocket connection

## Technology Stack

- **Next.js 15.0.0**: Latest stable React framework with App Router support
- **TypeScript 5.6**: Type-safe development with latest features
- **TailwindCSS 3.4**: Utility-first CSS framework
- **Recharts 2.8**: Data visualization library
- **Jest & React Testing Library**: Comprehensive testing suite

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch

# Type checking
npm run type-check

# Linting
npm run lint
```

### Environment Variables

Create a `.env.local` file:

```env
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
```

## Testing

The project includes comprehensive tests with 80%+ coverage:

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end workflow testing
- **WebSocket Tests**: Real-time communication testing
- **Mock Tests**: Isolated component testing

### Test Structure

```
src/
├── components/__tests__/     # Component unit tests
├── hooks/__tests__/         # Custom hook tests
└── __tests__/               # Integration tests
```

### Running Tests

```bash
# Run all tests
npm test

# Run specific test file
npm test Dashboard.test.tsx

# Run tests with coverage report
npm run test:coverage

# Run tests in watch mode for development
npm run test:watch
```

## Components

### Dashboard

Main application component with tab navigation and WebSocket integration.

### AuctionView

Displays live auction information, bidding progress, and participant details.

### BESSNodeMap

Shows battery energy levels, health status, and availability for sale.

### PriceAnalytics

Charts and statistics for price trends and competitive analysis.

### SystemMetrics

System health monitoring and performance metrics.

### ConnectionStatus

WebSocket connection status indicator with reconnection handling.

## WebSocket Events

The dashboard listens for the following events:

- `AuctionStarted`: New auction begins
- `BidPlaced`: New bid placed
- `BidAccepted`: Bid accepted by BESS
- `BidRejected`: Bid rejected by BESS
- `BESSNodeStatus`: BESS node status update
- `AggregatorStatus`: Aggregator status update
- `SystemMetrics`: System performance metrics

## Development

### Project Structure

```
src/
├── components/          # React components
├── hooks/              # Custom React hooks
├── types/              # TypeScript type definitions
├── styles/             # Global styles
└── pages/              # Next.js pages
```

### Code Quality

- **TypeScript**: Strict type checking enabled
- **ESLint**: Code linting with Next.js rules
- **Prettier**: Code formatting (configured via .prettierrc)
- **Jest**: Comprehensive testing framework

### Performance

- **Code Splitting**: Automatic code splitting by Next.js
- **Image Optimization**: Next.js Image component
- **Bundle Analysis**: Use `npm run build` to analyze bundle size

## Docker

The frontend is containerized and can be run with Docker Compose:

```bash
# From project root
docker-compose up frontend
```

## Contributing

1. Write tests for new features
2. Ensure all tests pass
3. Maintain 80%+ test coverage
4. Follow TypeScript best practices
5. Use meaningful commit messages

## License

This project is part of the Energy Trading System demonstration.
