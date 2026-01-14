# Multi-stage build for Docket

# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 docket

# Create data directory
RUN mkdir -p /data && chown docket:docket /data

# Copy the binary from builder
COPY --from=builder /app/target/release/docket /usr/local/bin/docket

# Set user
USER docket

# Set environment variables
ENV DOCKET_DB_PATH=/data/docket.db
ENV DOCKET_PORT=3000

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "sh", "-c", "wget --no-verbose --tries=1 --spider http://localhost:${DOCKET_PORT}/api/projects || exit 1" ]

# Run the server
CMD ["docket", "server"]
