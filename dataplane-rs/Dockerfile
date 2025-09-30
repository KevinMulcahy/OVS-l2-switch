FROM rust:1.81 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# Copy correct binary
COPY --from=builder /app/target/release/dataplane_rs /usr/local/bin/dataplane_rs

# Run the binary by name (it's in PATH)
CMD ["dataplane_rs"]

