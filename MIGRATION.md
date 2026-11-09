# Pier → MOONSHIPS: in-place migration

## Compatibility-first policy
This migration is an in-place product rebrand and binary upgrade, not an uninstall/reinstall. Current application components still depend on Pier-named runtime contracts. Keep these identifiers unchanged:

| Runtime contract | Preserved value |
|---|---|
| Service user/group | pier |
| Install root | /opt/pier |
| Data and environment | /opt/pier/data, /opt/pier/.env |
| Main systemd unit | pier.service |
| Network helper/socket | pier-net-helper.service, /run/pier/net.sock |
| Updater state | pier-updater.*, /var/lib/pier-updater |
| Environment namespace | PIER_* |
| Executable entry point | pier |

Changing only one of these values can prevent service startup, break SQLite access, or strand the helper/updater.

## Before you begin
- [ ] Take a full VPS/provider snapshot.
- [ ] Verify the latest application backup and test restoration where possible.
- [ ] Save a root-only copy of /opt/pier/.env.
- [ ] Record the dashboard port and current service status.
- [ ] Ensure enough disk space for the new and rollback binaries.
- [ ] Keep provider console/SSH access available.

## Upgrade

The installer automatically creates a root-only pre-upgrade backup under /var/backups/moonships/pre-upgrade-XXXXXX before replacing any existing binary or service unit. It copies /opt/pier/data (including SQLite WAL sidecars), /opt/pier/.env, /opt/pier/bin, and the existing Pier systemd units. A backup failure aborts the upgrade before replacement. Docker volumes are not part of /opt/pier/data; a VPS/provider snapshot remains strongly recommended.

    git clone https://github.com/BRIDGE-ALLAH/MW.git
    cd MOONSHIPS
    cargo build --release --package pier-core
    sudo cp /opt/pier/bin/pier /opt/pier/bin/pier.pre-moonships
    sudo bash scripts/install.sh --binary target/release/pier

The installer is expected to detect the existing Pier service, stop it for replacement, preserve its configuration/data paths, refresh compatible units, and start it again. Do not delete the old data directory.

## Verify

    sudo systemctl is-active pier
    sudo systemctl status pier --no-pager
    sudo journalctl -u pier -n 100 --no-pager
    sudo test -s /opt/pier/.env
    sudo test -d /opt/pier/data

Sign in and verify projects, deployments, domains, certificates, database resources, agents, backups, and workload health.

## Rollback

Find the latest pre-upgrade backup, stop the service, and restore the previous binary, data, environment and systemd units. The example below uses Bash:

    BACKUP="$(sudo ls -dt /var/backups/moonships/pre-upgrade-* | head -n1)"
    sudo systemctl stop pier
    sudo cp -a "$BACKUP/bin/." /opt/pier/bin/
    sudo cp -a "$BACKUP/.env" /opt/pier/.env
    sudo mv /opt/pier/data "/opt/pier/data.failed-$(date +%s)"
    sudo cp -a "$BACKUP/data" /opt/pier/data
    for unit in pier.service pier-net-helper.service pier-updater.service pier-updater.path; do
      if [ -f "$BACKUP/$unit" ]; then sudo cp -a "$BACKUP/$unit" "/etc/systemd/system/$unit"; fi
    done
    sudo systemctl daemon-reload
    sudo systemctl start pier

If the application changed Docker volumes or other host-level resources, restore the provider snapshot too. The installer itself does not remove or alter Docker volumes.
If the new application performed an incompatible database schema migration, restore the pre-upgrade database/snapshot as well. A binary-only rollback is not sufficient after an irreversible schema change.

## Scope boundary
This migration does not rename Linux users, directories, systemd units, environment variables, sockets, or database files. A later runtime namespace migration must implement compatibility aliases, backup/restore tests, helper-unit parity tests, and a tested rollback before retiring old identifiers.
