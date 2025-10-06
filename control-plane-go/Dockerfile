# Stage 1: Build
FROM golang:1.22 AS builder
WORKDIR /app

# Copy and build
COPY . .
RUN go mod tidy
RUN go build -o switchapi ./cmd/switchapi

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app

# Install curl for healthcheck
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/switchapi /usr/local/bin/switchapi

# Healthcheck endpoint
HEALTHCHECK --interval=5s --timeout=2s --start-period=5s --retries=5 \
  CMD curl -f http://localhost:8080/health || exit 1

# Start service
CMD ["switchapi"]