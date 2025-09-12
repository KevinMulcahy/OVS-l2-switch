# Roadmap

## Vision
Build an open-source Layer 2 switching platform that runs on PC architecture, with modular components for dataplane, control plane, management APIs, and user interfaces (CLI, GUI, and mobile).

---

## Phase 1: Foundations (Months 0–3)
- [ ] Define core requirements and architecture  
- [ ] Set up GitHub repository, licensing, and contribution guidelines  
- [ ] Implement basic packet forwarding with Linux networking stack (DPDK/eBPF or OVS baseline)  
- [ ] Create Makefile and CI/CD pipelines (linting, unit tests, integration tests)  
- [ ] Documentation:
  - [ ] `README.md`  
  - [ ] `INSTALL.md` for supported Linux distros  
  - [ ] Developer setup guide  

---

## Phase 2: Core Switching Features (Months 3–6)
- [ ] MAC learning table implementation (aging, flood/forward logic)  
- [ ] VLAN support (802.1Q tagging/untagging)  
- [ ] Spanning Tree Protocol (basic RSTP)  
- [ ] Integrate systemd service for persistence  
- [ ] Create test suite for:
  - [ ] Interface discovery  
  - [ ] Forwarding logic validation  
  - [ ] VLAN separation tests  

---

## Phase 3: Management & APIs (Months 6–9)
- [ ] Northbound **REST API**  
- [ ] Southbound integration with Linux netlink and OVSDB  
- [ ] Role-based access control for API  
- [ ] Logging and monitoring hooks (syslog, Graylog, Prometheus)  
- [ ] Documentation:
  - [ ] `API.md`  
  - [ ] Examples of API usage with curl and Python SDK  

---

## Phase 4: User Interfaces (Months 9–12)
- [ ] CLI tool for switch management (config, show, debug)  
- [ ] Web-based GUI  
- [ ] Mobile app prototype for monitoring  
- [ ] Theme and layout customization for GUI  
- [ ] Build Docker images for quick deployment  

---

## Phase 5: Advanced Switching & Performance (Months 12–18)
- [ ] Link aggregation (LACP)  
- [ ] QoS and traffic shaping  
- [ ] ACLs (packet filtering)  
- [ ] Performance optimizations:
  - [ ] DPDK acceleration  
  - [ ] eBPF offload for flow handling  
- [ ] Benchmarking suite with traffic generators (iperf, TRex)  

---

## Phase 6: Ecosystem & Community (Months 18–24)
- [ ] Documentation website (MkDocs or Sphinx)  
- [ ] Release **v1.0.0** with stable feature set  
- [ ] Publish SDKs (Python, Go, Node.js)  
- [ ] Build demo labs (VMware, Proxmox, Docker Compose)  
- [ ] Community:
  - [ ] Contributor guidelines  
  - [ ] Issue templates  
  - [ ] Feature request discussions  
- [ ] Security review and hardening  

---

## Long-Term Goals
- [ ] Multi-chassis link aggregation  
- [ ] VXLAN/GRE tunneling for overlays  
- [ ] Integration with SDN controllers (ONOS, OpenDaylight)  
- [ ] Hardware offload (FPGA/NIC support)  
- [ ] Pluggable policy engine for ICS/enterprise use cases  