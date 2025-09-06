# Multi-stage build for Rust energy trading system
FROM rust:1.75 as builder

# Set working directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false energy

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/energy-trading /app/energy-trading

# Change ownership
RUN chown energy:energy /app/energy-trading

# Switch to non-root user
USER energy

# Expose port (will be overridden by docker-compose)
EXPOSE 8080

# Default command (will be overridden by docker-compose)
CMD ["./energy-trading"]
