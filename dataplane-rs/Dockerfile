FROM rust:1.81 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/dataplane-rs /usr/local/bin/dataplane-rs

CMD ["dataplane-rs"]
