# MOONSHIPS — Installation

The current release uses a compatibility layer that retains Pier's on-disk and systemd identifiers. This allows existing deployments to be upgraded without moving application state.

## Requirements
- Linux with systemd (Ubuntu/Debian recommended)
- Docker Engine and Docker Compose plugin
- Git, curl, Rust 1.93+, and sudo/root access

## Build from source

    sudo apt-get update
    sudo apt-get install -y curl git build-essential pkg-config libssl-dev
    git clone https://github.com/BRIDGE-ALLAH/MOONSHIPS.git
    cd MOONSHIPS
    cargo build --release --package pier-core
    sudo bash scripts/install.sh --binary target/release/pier

The installer checks Docker and Compose, installs the compatible binary and service units, and starts the service. It reuses the existing data directory when upgrading. Before replacing an existing Pier installation, it automatically backs up /opt/pier/data (including SQLite WAL files), /opt/pier/.env, /opt/pier/bin, and the Pier systemd units to a root-only directory under /var/backups/moonships/. If the backup cannot be completed, the upgrade stops before replacing the old installation. Docker volumes remain in Docker's storage and are not copied by this backup, so take a provider snapshot as well.

## First-time setup
Open https://SERVER_IP:8443/setup (or your configured port) and create the administrator account. The initial certificate may be self-signed; configure a domain and trusted TLS after setup.

Allow SSH (22), HTTP (80), HTTPS (443), and the dashboard port (8443 by default) through your firewall.

## Upgrade an existing Pier installation
1. Take a VPS/provider snapshot and verify a recent application/database backup.
2. Save a root-only copy of /opt/pier/.env; it contains secrets.
3. Build the MOONSHIPS-compatible binary from this repository.
4. Run sudo bash scripts/install.sh --binary target/release/pier.
5. Verify systemctl status pier and inspect journalctl -u pier -n 100 --no-pager.
6. Sign in and verify projects, deployments, domains, TLS, agents, backups, and application health.

### Preserved runtime contracts
- /opt/pier, including .env and data/
- pier system account and group
- pier.service, pier-net-helper.service, and updater units
- /run/pier/net.sock and existing helper binaries
- Existing Docker resources and persistent volumes

These names are intentionally retained because the current runtime, embedded helper unit, updater, and service files refer to them. The visible product identity is MOONSHIPS; internal compatibility identifiers remain Pier until a coordinated runtime migration is implemented and tested.

### Rollback
If the service fails health checks:
1. Stop it with sudo systemctl stop pier.
2. Restore the saved binary to /opt/pier/bin/pier.
3. Start it with sudo systemctl start pier.
4. If a newer build changed the database schema, restore the matching pre-upgrade database/snapshot too.

Never run rm -rf /opt/pier, delete Docker volumes, or remove the Pier service account during migration.

## Release installation
The release bootstrap path is available only after this repository publishes the expected latest assets (core binary, SHA-256 checksum, agent, and network helper). Until then, build from source. Do not bypass checksum verification.

## Security
Use SSH keys, restrict dashboard exposure, configure trusted TLS, keep Docker and the host patched, store backups off-host, and periodically test restoration. Never share setup tokens, recovery keys, environment files, or logs containing secrets.

See [MIGRATION.md](MIGRATION.md) for the detailed checklist.
