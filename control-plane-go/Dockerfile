FROM golang:1.22 as builder

WORKDIR /app
COPY . .

# Build the switchapi binary
RUN go build -o switchapi ./cmd/switchapi

FROM debian:bookworm-slim
WORKDIR /app

# Copy binary into PATH
COPY --from=builder /app/switchapi /usr/local/bin/switchapi

# Run by name (PATH lookup)
CMD ["switchapi"]
