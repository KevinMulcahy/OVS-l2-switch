#!/usr/bin/env bash
set -euo pipefail

echo "Starting integration tests..."

# Give services time to boot
sleep 3

# --- Control plane health check ---
echo "Checking control plane health..."
if curl -s http://controlplane:8080/health | grep -q '"status":"ok"'; then
  echo "✅ Control plane health endpoint returned ok"
else
  echo "❌ Control plane health check failed"
  exit 1
fi

# --- Dataplane stub check ---
echo "Checking dataplane process..."
if docker ps | grep -q dataplane; then
  echo "✅ Dataplane container is running"
else
  echo "❌ Dataplane container is not running"
  exit 1
fi

echo "All integration tests passed!"
