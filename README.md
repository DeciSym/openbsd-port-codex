# OpenBSD Port: Codex

This repository contains a draft OpenBSD port for Codex (`devel/codex`).

## Project-Local Environment

Create `env.sh` in this repository root:

```sh
export PORTSDIR_PATH=/home/user/projects/openbsd-port-codex:/usr/ports:/usr/ports/mystuff
export MAKE_JOBS=16
export CARGO_BUILD_JOBS=16
export WRKOBJDIR=/home/user/projects/openbsd-port-codex/.wrk
export DISTDIR=/home/user/projects/openbsd-port-codex/.distfiles
export PACKAGE_REPOSITORY=/home/user/projects/openbsd-port-codex/.packages
export PLIST_REPOSITORY=/home/user/projects/openbsd-port-codex/.plist
export BULK_COOKIES_DIR=/home/user/projects/openbsd-port-codex/.bulk
```

Load it for each shell session:

```sh
cd /home/user/projects/openbsd-port-codex
. ./env.sh
```

## Build Package

```sh
cd /home/user/projects/openbsd-port-codex/devel/codex
make clean=all
make package
```

Output package path:

`/home/user/projects/openbsd-port-codex/.packages/amd64/all/codex-0.104.0.tgz`

## Install Package

```sh
doas pkg_add /home/user/projects/openbsd-port-codex/.packages/amd64/all/codex-0.104.0.tgz
```

## Quick Validation

```sh
cd /home/user/projects/openbsd-port-codex/devel/codex
PORTSDIR_PATH=/home/user/projects/openbsd-port-codex:/usr/ports:/usr/ports/mystuff \
  /usr/ports/infrastructure/bin/portcheck -p /home/user/projects/openbsd-port-codex
```
