# Stage 1: Build
FROM rust:1.81 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/dataplane_rs /usr/local/bin/dataplane_rs

HEALTHCHECK --interval=5s --timeout=2s --start-period=5s --retries=5 \
  CMD curl -f http://localhost:8080 || exit 1

CMD ["dataplane_rs"]