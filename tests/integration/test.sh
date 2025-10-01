#!/usr/bin/env bash
set -euo pipefail

echo "Starting integration tests..."
sleep 3

# --- Control plane health check ---
echo "Checking control plane health..."
if curl -s http://controlplane:8080/health | grep -q '"status":"ok"'; then
  echo "✅ Control plane health endpoint returned ok"
else
  echo "❌ Control plane health check failed"
  exit 1
fi

# --- Dataplane liveness check via network ping ---
echo "Checking dataplane container..."
if ping -c1 dataplane >/dev/null 2>&1; then
  echo "✅ Dataplane container is reachable on the network"
else
  echo "❌ Dataplane container not reachable"
  exit 1
fi

echo "All integration tests passed!"

