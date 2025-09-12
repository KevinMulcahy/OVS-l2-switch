# OVS Layer 2 Switch

A Linux-based Layer 2 managed switch using Open vSwitch (OVS) with automatic interface detection, VLAN support, port mirroring, traffic shaping, and advanced switching features.

---

## Quick Start

Follow these steps to get the OVS Layer 2 switch up and running:

1. Install prerequisites on your Linux machine
2. Run the interface discovery script
3. Deploy OVS with dynamic bridge setup
4. Configure VLANs, LACP, QoS, and OpenFlow as needed
5. Use management scripts for monitoring, backups, and troubleshooting

### Example Commands

```bash
# Discover interfaces (excluding management)
sudo ./discover-interfaces.sh

# Deploy the OVS bridge and add interfaces
sudo ./deploy-ovs-switch.sh

# Configure the switch interactively
sudo ./configure-switch.sh

Usage Notes
	•	You can specify a custom management interface:

MANAGEMENT_INTERFACE=ens18 sudo ./discover-interfaces.sh

	•	Scripts automatically detect and handle any number of interfaces
	•	Generated management scripts are located in /usr/local/bin/ovs-scripts/

Key Benefits
	1.	Automatic Detection: Discovers NICs dynamically
	2.	Flexible Configuration: Supports 2, 5, 10, or 50+ interfaces
	3.	Safety: Preserves the management interface
	4.	Scalable: Performance tuning adjusts to interface count
	5.	Management Tools: Generates scripts tailored to your setup
	6.	Recovery: Provides backup and restore capabilities

Example Scenarios

# Machine with 8 interfaces
sudo ./discover-interfaces.sh
sudo ./deploy-ovs-switch.sh

# Machine with 20 interfaces and custom management
MANAGEMENT_INTERFACE=ens18 sudo ./discover-interfaces.sh
sudo ./deploy-ovs-switch.sh

# Minimal 2-interface setup
sudo ./discover-interfaces.sh
sudo ./deploy-ovs-switch.sh

Prerequisites
	•	Linux machine with multiple physical network interfaces (2 or more)
	•	Root access to the system
	•	Basic understanding of networking concepts
	•	At least one interface reserved for management (recommended)

Architecture Overview

    ┌───────────────────────────────┐
    │         Linux Host            │
    │ ┌─────────────┐               │
    │ │   Open vSwitch (OVS)        │
    │ │ ┌─────────┐                 │
    │ │ │ br0     │                 │
    │ │ │ eth1…N │                 │
    │ │ └─────────┘                 │
    │ └─────────────┘               │
    └───────────────────────────────┘

Installation and Setup

Step 1: Discover Interfaces

sudo ./discover-interfaces.sh

	•	Prompts for confirmation before proceeding
	•	Validates that minimum interface requirements are met

Step 2: Install OVS

sudo ./install-ovs.sh

	•	Detects your Linux distribution (Ubuntu, Debian, RHEL, CentOS, Fedora)
	•	Installs required packages and dependencies
	•	Enables and starts OVS services

Step 3: Setup OVS Bridge

sudo ./setup-ovs-bridge.sh

	•	Creates the main bridge (br0)
	•	Adds all discovered interfaces (excluding management)
	•	Configures persistent bridge setup via systemd service

Step 4: Configure Switch

sudo ./configure-switch.sh

	•	Interactive menu for:
	•	VLANs
	•	Port mirroring
	•	QoS
	•	Link aggregation (LACP)
	•	OpenFlow controller
	•	Exporting management scripts

Management and Maintenance

View Current Configuration

sudo ./show-network-config.sh

	•	Lists OVS bridges
	•	Displays interface status
	•	Shows port statistics

Backup and Restore

sudo ./backup-ovs-config.sh
sudo ./restore-ovs-config.sh

	•	Backs up current OVS configuration
	•	Restores from backup when needed

Generated Scripts Location
	•	/usr/local/bin/ovs-scripts/
Contains scripts for common tasks like monitoring, VLAN changes, port resets, and interface diagnostics.

⸻

Contributing
	•	Fork the repository
	•	Make changes on a feature branch
	•	Submit pull requests with clear descriptions
	•	Ensure scripts pass validation and lint checks

License

This project is licensed under the MIT License.
