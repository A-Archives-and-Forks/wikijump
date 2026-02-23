## Dev Environment

This illustrates the setup for the [Dokploy](https://dokploy.com)-based dev tier hosting `wikijump.dev`:

1. Create a Virtual Private Server with Ubuntu 24.04 LTS.
2. Set up a non-root administrator account:
```
# adduser --disabled-password maintainer
# gpasswd -a maintainer sudo
```
3. Disable password-based SSH (if not already disabled):
```
$ sudoedit /etc/ssh/sshd_config
PasswordAuthentication no
PermitEmptyPasswords no
$ sudo systemctl reload ssh.service
```
4. Add SSH keys to enable login as `maintainer`:
```
$ mkdir -m700 .ssh
$ nano .ssh/authorized_keys
$ chmod 600 .ssh/authorized_keys
```
5. Install Dokploy:
```
# curl -sSL https://dokploy.com/install.sh | sh
```
6. Go to the new HTTP endpoint and set up the Dokploy owner account.
7. Set up the HTTPS custom domain for Dokploy (here, `deploy.wikijump.dev`).
8. Add [GitHub Container Registry to Dokploy](https://docs.dokploy.com/docs/core/registry/ghcr).
9. Add S3 buckets to Dokploy and configure regular backups.
10. Add git repository connection to Dokploy.
11. Create "Wikijump" project.
  1. Add PostgreSQL database. Set database / user to `wikijump`.
  2. Add Valkey / Redis database.
  3. Add "Compose" application. This uses the git repository to read the docker-compose file at `install/dev/docker-compose.yaml` on branch `develop`.
12. Create all relevant domains (and subdomains) for the project. Each needs to point to `caddy` at port 80, and should have Let's Encrypt provide TLS certificates.
13. Create deployments for all services.
