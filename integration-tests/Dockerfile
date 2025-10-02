FROM debian:bookworm-slim

# Install necessary tools
RUN apt-get update && apt-get install -y \
    curl \
    netcat-openbsd \
    bash \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Default command (replace with your actual test runner)
CMD ["./run_tests.sh"]
