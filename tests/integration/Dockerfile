FROM debian:bookworm-slim

# Install curl for HTTP checks
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY test.sh .

CMD ["./test.sh"]
