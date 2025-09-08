FROM rust:latest as builder

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

# Runtime stage - use Ubuntu 22.04 for glibc compatibility
FROM ubuntu:22.04

# Install minimal runtime dependencies
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
COPY --from=builder /app/target/release/simple-gateway /app/simple-gateway

# Change ownership
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose ports
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Set environment variables
ENV RUST_LOG=info

# Run the application
CMD ["/app/simple-gateway"]
