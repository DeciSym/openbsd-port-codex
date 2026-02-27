# OpenBSD Port: Codex

This repository contains a draft OpenBSD port for Codex (`devel/codex`).

## Project-Local Environment

This repository includes `env.sh` in the repository root.

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

`${REPO_ROOT}/.packages/amd64/all/codex-0.106.0.tgz`

## Install Package

```sh
doas pkg_add -D unsigned "${REPO_ROOT}/.packages/amd64/all/codex-0.106.0.tgz"
```

## Validation

```sh
cd /path/to/openbsd-port-codex
. ./env.sh
cd "${REPO_ROOT}/devel/codex"
/usr/ports/infrastructure/bin/portcheck -p "${REPO_ROOT}"
```
