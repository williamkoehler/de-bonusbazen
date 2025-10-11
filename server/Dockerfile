# ---- Build stage ----
ARG RUST_VERSION=1.90
FROM rust:${RUST_VERSION}-slim AS builder

WORKDIR /app

# Common native dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy full project and build release
COPY . .
ENV CARGO_TERM_COLOR=always
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime

# Minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates openssl tzdata \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m -u 10001 appuser

WORKDIR /app

COPY --from=builder /app/target/release/server ./server

EXPOSE 8080

# Drop privileges to a non-root user
USER appuser

ENTRYPOINT ["/app/server"]