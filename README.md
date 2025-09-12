# Open Source Layer 2 Switch

An open-source Layer 2 switching platform designed to run on commodity PC hardware.  
This project aims to provide a modular, extensible, and community-driven virtual switch suitable for labs, edge deployments, and NFV environments.

---

## ✨ Vision
Build a lightweight, extensible, community-driven Layer 2 software switch that:
- Runs on standard x86_64 hardware
- Provides modular datapaths (kernel, DPDK, XDP/eBPF)
- Includes simple CLI, REST API, and GUI management
- Offers VLAN, MAC learning, and forwarding features out of the box
- Is easy to build, test, and contribute to

---

## 🛠️ Features (Planned)

### MVP
- Basic Layer 2 forwarding (MAC learning, flooding, unicast)
- VLAN support (802.1Q tagging/untagging)
- Simple management (CLI + REST API)
- Containerized deployment (Docker/Podman)
- Automated tests and CI workflows

### Later Phases
- DPDK and/or AF_XDP acceleration
- Spanning Tree Protocol (RSTP)
- Link Aggregation (LACP)
- QoS and traffic shaping
- ACLs and port mirroring
- Monitoring and metrics (Prometheus, Graylog)
- GUI and mobile management apps
- SDKs in Python, Go, Node.js

---

## 📐 Architecture

- **Datapath Layer**  
  Handles packet I/O and forwarding. Modular design:
  - Kernel networking stack (AF_PACKET/TAP) for MVP
  - DPDK backend for high throughput
  - eBPF/XDP backend for kernel-accelerated performance

- **Control Plane**  
  - MAC learning, VLAN table, STP/LACP logic
  - Flow handling policies

- **Management Plane**  
  - CLI (`switchctl`) for interactive management
  - REST API with OpenAPI spec
  - Future: GUI and mobile frontends

---

## 📅 Roadmap

See [ROADMAP.md](ROADMAP.md) for the full phased plan.  
Highlights:
- **Phase 1:** Repo setup, CI/CD, minimal datapath  
- **Phase 2:** Core switching (MAC learning, VLANs, STP)  
- **Phase 3:** Management APIs + persistence  
- **Phase 4:** CLI/GUI/mobile interfaces  
- **Phase 5:** Performance datapaths (DPDK, XDP)  
- **Phase 6:** Ecosystem, community, and v1.0 release  

---

## 🚀 Getting Started

### Prerequisites
- Linux (Ubuntu, Debian, or Fedora recommended)
- Git, Make, GCC/Clang, Python3
- (Optional) Docker/Podman for containerized testing

### Quick Start (MVP Datapath)
```bash
# Clone repo
git clone https://github.com/your-org/ovs-layer2-switch.git
cd ovs-layer2-switch

# Build
make build

# Run basic test harness
make test

Example (veth test)

# Create veth pair
sudo ip link add veth0 type veth peer name veth1
sudo ip link set veth0 up
sudo ip link set veth1 up

# Start switch
sudo ./bin/switchd --add-port veth0 --add-port veth1


⸻

🤝 Contributing

We welcome contributions!
	•	See CONTRIBUTING.md for guidelines
	•	Good first issues are labeled in GitHub
	•	Discussions and feature requests welcome in Issues/Discussions

⸻

🧪 Testing
	•	Unit tests with make test
	•	Integration tests with Docker Compose harness
	•	Benchmarking suite (iperf3, pktgen, TRex) planned for perf datapaths

⸻

📜 License

This project is licensed under Apache 2.0.
See LICENSE for details.

⸻

🌍 Community & Governance
	•	Transparent roadmap and planning in GitHub
	•	Maintainers and contributors listed in MAINTAINERS.md
	•	Governance model will evolve with community size (Linux Foundation patterns as reference)

⸻

🔮 Long-Term Goals
	•	VXLAN/GRE tunneling for overlays
	•	SDN controller integration (ONOS, OpenDaylight)
	•	Hardware offload (FPGA/NIC support)
	•	Programmable data plane with P4

---