FROM rust:1.82-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates curl openssl

# Create app user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

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
CMD ["./aggregator"]
