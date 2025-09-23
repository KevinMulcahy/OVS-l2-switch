# INSTALL.md

# Installation Guide: Open-Source Layer 2 Switching Platform

This document provides **installation instructions** for supported Linux distributions, including prerequisites, building from source, and basic setup.

---

## 1. Supported Linux Distributions

The platform is tested and supported on the following distributions:

| Distribution | Minimum Version | Notes |
|-------------|----------------|-------|
| Ubuntu LTS  | 22.04          | Recommended for development and production |
| Debian      | 12             | Stable base, long-term support |
| Fedora      | 39             | Latest features, bleeding-edge kernel |
| CentOS / Rocky Linux | 9      | For enterprise environments |
| Arch Linux  | Rolling        | Latest kernel, developer-focused |

**Note:** Kernel version 5.15+ is recommended for optimal Linux networking stack performance and eBPF/DPDK support.

---

## 2. Prerequisites

Before building and running the platform, ensure the following packages and tools are installed:

### System Packages (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y     build-essential     git     curl     wget     pkg-config     libssl-dev     libelf-dev     clang     cmake     python3     python3-pip     libpcap-dev     iproute2     iputils-ping
```

### System Packages (Fedora)
```bash
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y git curl wget clang cmake libpcap-devel python3 python3-pip     openssl-devel elfutils-libelf-devel iproute iputils
```

### Rust Toolchain (Dataplane)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
rustup default stable
rustup component add rustfmt clippy
```

### Go Toolchain (Control & Management Plane)
```bash
curl -LO https://go.dev/dl/go1.21.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.linux-amd64.tar.gz
export PATH=$PATH:/usr/local/go/bin
go version
```

---

## 3. Repository Setup

Clone the project repository and navigate to the root directory:

```bash
git clone https://github.com/<username>/<repo>.git
cd <repo>
```

**Optional:** Initialize submodules if used:

```bash
git submodule update --init --recursive
```

---

## 4. Building the Platform

### Rust Dataplane
```bash
cd src
cargo build --release
```

**Verify build:**
```bash
cargo check
cargo test --all
```

### Go Control & Management Planes
```bash
cd internal/features
go mod download
go build ./...
go test ./...
```

---

## 5. Running the Platform

### Dataplane
```bash
# Example: start dataplane with default config
sudo ./target/release/dataplane --config configs/dataplane.yaml
```

### Control Plane
```bash
# Example: start control plane
./control-plane --config configs/control.yaml
```

### Management Plane
```bash
# Example: start REST API server
./management-plane --config configs/management.yaml
```

**Logs and monitoring**  
- Syslog integration supported  
- Optional Graylog or Prometheus endpoints for metrics

---

## 6. Systemd Service Setup (Optional)
For automatic startup on system boot:

```bash
sudo cp systemd/*.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable dataplane.service
sudo systemctl enable control-plane.service
sudo systemctl enable management-plane.service
sudo systemctl start dataplane.service
sudo systemctl start control-plane.service
sudo systemctl start management-plane.service
```

**Verify status:**
```bash
sudo systemctl status dataplane.service
```

---

## 7. Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATAPLANE_CONFIG` | Path to dataplane YAML config | `configs/dataplane.yaml` |
| `CTRLPLANE_CONFIG` | Path to control plane YAML config | `configs/control.yaml` |
| `MGMT_CONFIG` | Path to management plane YAML config | `configs/management.yaml` |
| `LOG_LEVEL` | Logging verbosity | `info` |

Example:
```bash
export DATAPLANE_CONFIG=/etc/l2switch/dataplane.yaml
export LOG_LEVEL=debug
```

---

## 8. Docker Setup (Optional)

For a containerized development or demo environment:

```bash
docker build -t l2switch:latest .
docker run --rm --net=host -v $(pwd)/configs:/app/configs l2switch:latest
```

**Notes:**
- Host networking required for packet I/O  
- Mount config directory for customization

---

## 9. Troubleshooting

- **Missing dependencies:** Run `sudo apt install build-essential ...` or equivalent for your distro  
- **Port conflicts:** Ensure no other process is using control/management API ports  
- **Permissions:** Dataplane may require `sudo` for raw socket or eBPF access  

---

## 10. Next Steps

1. Configure switch interfaces and VLANs in `configs/dataplane.yaml`  
2. Start phases according to roadmap ([ROADMAP.md](./ROADMAP.md))  
3. Run test suite to validate installation and packet forwarding  

---

This document ensures consistent **installation across supported Linux distributions**, aligned with the **AIBD-RG coding and architecture guidelines**.
