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
COPY --from=builder /app/target/release/bess-node /app/bess-node

# Change ownership
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose ports
EXPOSE 8081 8888

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV NODE_ID=001
ENV NODE_TYPE=BESS
ENV CAPACITY_KWH=15
ENV INITIAL_ENERGY=12.5
ENV GATEWAY_HOST=gateway
ENV GATEWAY_PORT=8080
ENV MULTICAST_GROUP=224.0.0.1
ENV MULTICAST_PORT=8888

# Start BESS node
CMD ["./bess-node"]
