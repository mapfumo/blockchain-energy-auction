FROM rust:1.82 as builder

# Install build dependencies for Ubuntu target
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application for Ubuntu target
RUN cargo build --release

# Runtime stage
FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/aggregator /app/aggregator

# Change ownership
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose ports
EXPOSE 8082 8888

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8082/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV AGGREGATOR_ID=001
ENV NODE_TYPE=AGGREGATOR
ENV STRATEGY=CONSERVATIVE
ENV MAX_BID_PRICE=800
ENV GATEWAY_HOST=gateway
ENV GATEWAY_PORT=8080
ENV MULTICAST_GROUP=224.0.0.1
ENV MULTICAST_PORT=8888

# Start Aggregator
CMD ["/app/aggregator"]
