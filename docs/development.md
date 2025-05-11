# Local Development

This document will explain how to set up Wikijump on your machine for local development.

## Deployment

The `install` folder has everything you need to run a local Wikijump install either in a container or on metal or a VM.

The recommended way to install Wikijump is via Docker. Docker is a way of containerizing, or in the case of Windows or Mac, also virtualizing Linux images. It lets you easily create and destroy different Wikijump builds, and it also acts like a sandbox to protect the rest of your system from dependency pollution. 

> ### For Windows:
>
> For Windows, you will need WSL2, a way of running a Linux distribution simultaneously with Windows. You will need Windows 10 or 11 for this. The only alternative to WSL2 would be using a Linux virtual machine such as [VirtualBox](https://www.virtualbox.org/) — but using a VM isn't recommended, and is not covered by this document.
>
> It is recommended that you use Ubuntu for the WSL2 distribution. Ubuntu in particular has considerations made for WSL2 use and in general will be the most reliable way forward.
>
> [WSL2 download and installation page](https://learn.microsoft.com/en-us/windows/wsl/install)

You will need [Docker](https://www.docker.com/) installed and running:

<pre>$ sudo apt install docker.io</pre>

This example is for Ubuntu. Installation may be different depending on your Linux distro. See [here](https://docs.docker.com/desktop/install/linux-install/) for more information.

After installing Docker, you can run it with the following commands:

<table>
<thead><tr><th>systemd Distros</th><th>WSL2</th></tr></thead>
<tbody valign="top">
<td><p><pre>$ sudo systemctl enable --now docker.service</pre></p></td>
<td><p><pre>$ sudo service docker start</pre></p></td>
</tbody>
</table>

## Setup: Utilities and Programs

You will need some utilities and programs to get started. You will to install the following:

- [Docker](https://www.docker.com/get-started) (see above)
- [Docker Compose](https://docs.docker.com/compose/)
- [NodeJS (and NPM, which comes with it), v15 or greater](https://nodejs.org/en/)
- [PNPM v6](https://pnpm.io/installation)

Node, NPM, and PNPM are well-behaved on Windows and Linux, and the difference in usage between operating systems is negligible.

## Setup: Configuration

There are a couple of configuration files that need to be initialized prior to running your instance of Wikijump. These will be the `config.toml` and `.env` files, both located in the `deepwell` subdirectory. For a typical local deploy, both of these files can be copied from their `.example` counterparts without changing them, though it is worth looking through them briefly to understand what can be configurated for your instance.

There is also a Docker configuration file that configures the various containers that host Wikijump in the local environment. You can find this file in `install/local`, named `docker-compose.yaml`, alongside `docker-compose.dev.yaml` (which provides various helpful tools for developing locally).

Notice that in `docker-compose.yaml`, there are configuration options for the domains to use. For development purposes, these are set to `wikijump.localhost`. This is the domain you will be connecting to, e.g. `https://www.wikijump.localhost`. The TLD `.localhost` is just like the usual `localhost` domain. Even when running locally, HTTPS is used. Because this certificate is self-signed, you will need to dismiss the certificate warning.

## Setup: Dependencies

You will need to install Wikijump's NPM dependencies. Navigate to the `web/` directory and run the following:

```sh
$ pnpm install
```

## Building

You can now start up the Docker images. The basic command used to run Wikijump locally is:

```sh
$ [sudo] docker-compose -p wikijump -f docker-compose.yaml [-f docker-compose.dev.yaml] <action>
```

* On some systems, `sudo` is required to run Docker, but on others it is not.
* The `-f docker-compose.dev.yaml` configuration file provides container bindings for development. In other words, if you modify `deepwell/src/` files locally, then those changes will be reflected in the container.

The `install/local/deploy.py` script is meant as a convenience here. You can pass in `--sudo` or `--no-dev` to make either of the above modifications for you.

The "action" corresponds to actions that `docker-compose` can do. Some common commands are:
* `up` &mdash; Create and start containers for Wikijump.
* `up --build` &mdash; Like the above but it first builds new images before creating containers.
* `start` &mdash; Start any already-existing containers for Wikijump.
* `stop` &mdash; Stop currently-running containers for Wikijump.

To start the project, you want to run a command that probably looks something like:

```
$ install/local/deploy.py up
```

_This might take some time_. Thankfully, Docker's build step is heavily cached. You will generally not need to rerun this build often.

> ### For Windows:
>
> You may encounter various errors involving file permissions if using Windows-based tools alongside WSL2. It is recommend that you fix these issues by either granting the correct file permissions to any that Windows may have modified, or by re-cloning the repository purely within WSL2.

Once everything has started, you can connect to `http://www.wikijump.localhost/`. Changes you make to the codebase should automatically be applied to the containers, as your machine's filesystem has been "bound" to the containers' filesystem (see `docker-compose.dev.yaml` below). This is one-way, so a container can't modify your filesystem. Adding new dependencies, however, will require a rebuild.

You can just kill the terminal (`CTRL + C` usually) when you want to stop the server.

If you want to entirely _reset_ the containers, as their data is otherwise persistent even across restarts, you can run the following:

```sh
$ docker-compose -p wikijump -f docker-compose.yaml -f docker-compose.dev.yaml down
```

It's useful to keep track of existing Docker images and containers, and destroy them when you no longer need them, so you don't waste space rebuilding the same image over and over. If you are using Docker Desktop, you can manage containers and images from the GUI. Otherwise, using the command line:

```
$ docker container ls  # List containers
$ docker rm [ID]       # Destroy the container with this ID
$ docker images        # List images
$ docker rmi [ID]      # Remove the image with this ID
```

You can also use `docker system prune`, which deletes *everything* unused, though this should be used sparingly.

## Entering the container

If you want to enter a container to make temporary changes, you can do so by entering it with a CLI. From Docker Desktop, after running the containers, find the Wikijump app and within it the container you wish to enter, then click the 'CLI' button. Or from the command line:

```
$ docker exec -it [container id] sh
```

...where `[container id]` is the ID of the corresponding container from `docker ps`. (`sh` can be replaced with a different command.)

One reason you may need to enter the container is to manually adjust the Wikijump config. For example, if you use a port other than 80 for your Docker container, you will need to edit `site.custom_domain` to add the port number (e.g. "`www.wikijump.localhost:8080`"). Alternatively, use curl to set the domain directly (e.g. "`-H 'www.wikijump.localhost'`")

## Unhealthy Service

Sometimes when starting a container locally, it will report being "unhealthy" and cause your service to be nonfunctional. There are a few reasons to check for when troubleshooting:

* If the container is not `deepwell`, it may be that `deepwell` is unhealthy and a dependent container is unable to connect to it.
* Since builds are done in runtime, the _initial_ build a container does (before it can do faster incremental builds) can take a long time. In some cases, docker will decide that the service has taken too long and is unhealthy. Try restarting it to allow the build to finish.
* If the build fails (say due to a compiler error), the service will not be healthy. If the failure happened a while ago, the logs will not be visible at the bottom of your screen; check `./deploy logs [container]` to see what the issue may be.
* The service may be trying to access a file which is unavailable. Check that the file hasn't been removed locally, or that if you are not using local binding (i.e. `--no-dev`), that the files have been manually installed into the container.

## Making Web Requests

Once you have a local instance of Wikijump running, you may wish to make `curl` requests against it to test various pieces of its functionality. However there are a few considerations to be had given the deployment situation:

1. **Wikijump is host-sensitive.** Hitting the same web route with two different domains will result in different content (e.g. `scp-sandbox-3.wikidot.com` and `scp-jp.wikidot.com` are different sites).
2. **Caddy local serves self-signed certificates.** Naturally, as this is not deployed on the open web, there are no "real" TLS certificates, but nonetheless Wikijump is designed to be HTTPS-only so we must use even locally.
3. **Svelte does not assume any accepted content types.** Any framerail calls need this, but wjfiles does not.

Thus, the "standard" curl request for a web page would look something like the following:

```
$ curl -i -k -H 'Host: scpwiki.localhost' -H 'Accept: text/html' https://localhost/scp-002
```

The `-k` argument addresses point 2, the `Host` header addresses point 1, and the `Accept` header addresses point 3. This request effectively fetches `https://scpwiki.localhost/scp-002` for you. Naturally, you can replace the route after `localhost` with whatever you are requesting (say `/-/file/scp-001/fractal-mka.jpeg`).

## Clock Drift

If you enable multi-factor authentication on a local container you may find that
the clock drift is too great for TOTP codes to work. In docker-compose (`wsl -d docker-compose`)
you can enter this command to sync your time up:

```
$ ntpd -d -q -n -p 0.pool.ntp.org
```
