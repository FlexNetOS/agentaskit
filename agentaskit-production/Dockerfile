# ─────────────────────────────────────────────────────────────────────────────
# AgentAskit Production Dockerfile
# Multi-stage build for optimal image size and security
# ─────────────────────────────────────────────────────────────────────────────

# Stage 1: Build
FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY . .

# Touch main.rs to ensure rebuild with actual code
RUN touch src/main.rs 2>/dev/null || true

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 -s /bin/false agentaskit

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/agentaskit* /app/ 2>/dev/null || true

# Copy configuration files
COPY --from=builder /app/configs /app/configs 2>/dev/null || true

# Set ownership
RUN chown -R agentaskit:agentaskit /app

# Switch to non-root user
USER agentaskit

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose default port
EXPOSE 8080

# Default command
CMD ["./agentaskit"]

# Labels
LABEL org.opencontainers.image.source="https://github.com/FlexNetOS/agentaskit"
LABEL org.opencontainers.image.description="AgentAskit Production Runtime"
LABEL org.opencontainers.image.licenses="MIT"
