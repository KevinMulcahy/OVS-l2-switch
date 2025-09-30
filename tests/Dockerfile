FROM golang:1.22-alpine

WORKDIR /tests
COPY . .

# Install curl in case you want shell-based health probes too
RUN apk add --no-cache curl

CMD ["go", "test", "-v", "./..."]
