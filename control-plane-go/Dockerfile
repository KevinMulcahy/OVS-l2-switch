FROM golang:1.22 as builder

WORKDIR /app
COPY . .

# Build switchapi (the API with /health)
RUN go build -o switchapi ./cmd/switchapi

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/switchapi /usr/local/bin/switchapi

CMD ["switchapi"]
