# Build stage
FROM rust:1.78-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release -p oa-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/openarcanum-server /usr/local/bin/openarcanum-server

ENV PORT=3000
ENV DATABASE_URL=/data/openarcanum.db

EXPOSE 3000

VOLUME ["/data"]

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["openarcanum-server"]
