# ☾ MOONSHIPS
### Your Infrastructure. Your Rules. Your Universe.

**An open-source, self-hosted platform for deploying and operating infrastructure you control.**

MOONSHIPS brings applications, containers, Docker Compose stacks, databases, domains, TLS, backups, and server operations into one lightweight control plane—from a small VPS to a fleet of servers.

## What you can do
- **Deploy applications:** Git-based deployments, Dockerfiles, images, Compose, and Railpack auto-builds.
- **Operate containers:** lifecycle, logs, stats, networks, and images.
- **Manage infrastructure:** domains, reverse proxy, automatic TLS, multi-server agents, and WireGuard mesh.
- **Run data services:** PostgreSQL, MySQL/MariaDB, MongoDB, Redis/Valkey, and more.
- **Protect your work:** scheduled backups, S3 integration, deployment history, rollback, and audit logs.
- **Work from one dashboard:** responsive web UI, role-based access, authentication, and database tools.

## Installation

### Build and install from source
Requirements: Linux, Docker Engine, Docker Compose plugin, Git, curl, and Rust 1.93+.

    git clone https://github.com/BRIDGE-ALLAH/MOONSHIPS.git
    cd MOONSHIPS
    cargo build --release --package pier-core
    sudo bash scripts/install.sh --binary target/release/pier

The executable and systemd identifiers remain Pier-named for compatibility with existing nodes. This is intentional; the public product identity is MOONSHIPS.

### Release installer
The one-command release flow requires a MOONSHIPS GitHub Release with compatible binary and checksum assets. Until those assets are published, use the source installation above. Never bypass checksum verification.

After installation, open https://YOUR_SERVER_IP:8443/setup and create the administrator account.

## Existing Pier server migration

**Upgrade in place; do not uninstall Pier first and do not delete /opt/pier.** Build the MOONSHIPS-compatible binary and run the installer over the existing installation. The compatibility layer retains:
- /opt/pier/data and /opt/pier/.env
- SQLite database and application metadata
- pier system user/group and Docker group membership
- pier.service, pier-net-helper.service, updater units, and /run/pier/net.sock
- Docker networks, containers, volumes, TLS material, and application data

Take a provider snapshot and verify a backup before upgrading. Keep the previous binary for rollback. See [MIGRATION.md](MIGRATION.md).

**Do not rename legacy data paths, service units, accounts, environment keys, or helper sockets during a branding-only upgrade.** The current runtime components depend on these interfaces; changing them independently risks downtime or data loss.

## Requirements
- Basic Docker/Compose workloads: 1 vCPU and 512 MB RAM minimum (more recommended).
- Railpack builds: 4 GB RAM and 40+ GB free disk recommended.
- Linux with systemd for the native installer.

## Project and license
This repository retains compatibility identifiers from the Pier codebase during migration. Existing copyright notices, license files, and applicable upstream attribution are preserved. Review [LICENSE](LICENSE) and comply with AGPL-3.0 when redistributing modified builds.

## Contributing
Issues and pull requests are welcome. Include reproduction steps, expected behavior, and logs with secrets removed.
