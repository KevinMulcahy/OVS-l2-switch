FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    curl \
    netcat-openbsd \
    bash \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# ✅ Ensure test runner is executable
RUN chmod +x ./run_tests.sh

CMD ["./run_tests.sh"]