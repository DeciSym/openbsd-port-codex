# OpenBSD Port: Codex

This repository contains a draft OpenBSD port for Codex (`devel/codex`).

## Project-Local Environment

Create `env.sh` in this repository root:

```sh
export REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
export PORTSDIR_PATH="${REPO_ROOT}:/usr/ports:/usr/ports/mystuff"
export MAKE_JOBS=16
export CARGO_BUILD_JOBS=16
export WRKOBJDIR="${REPO_ROOT}/.wrk"
export DISTDIR="${REPO_ROOT}/.distfiles"
export PACKAGE_REPOSITORY="${REPO_ROOT}/.packages"
export PLIST_REPOSITORY="${REPO_ROOT}/.plist"
export BULK_COOKIES_DIR="${REPO_ROOT}/.bulk"
```

Load it for each shell session:

```sh
cd /path/to/openbsd-port-codex
. ./env.sh
```

## Build Package

```sh
cd "${REPO_ROOT}/devel/codex"
make clean=all
make package
```

Output package path:

`${REPO_ROOT}/.packages/amd64/all/codex-0.104.0.tgz`

## Install Package

```sh
doas pkg_add -D unsigned "${REPO_ROOT}/.packages/amd64/all/codex-0.104.0.tgz"
```

## Quick Validation

```sh
cd "${REPO_ROOT}/devel/codex"
PORTSDIR_PATH="${REPO_ROOT}:/usr/ports:/usr/ports/mystuff" \
  /usr/ports/infrastructure/bin/portcheck -p "${REPO_ROOT}"
```
