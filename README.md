# OpenBSD Port: Codex

This repository contains a draft OpenBSD port for Codex (`devel/codex`).

## Project-Local Environment

Create `env.sh` in this repository root:

```sh
export REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
export PORTSDIR_PATH="${REPO_ROOT}:/usr/ports:/usr/ports/mystuff"
export MAKE_JOBS="$(sysctl -n hw.ncpuonline)"
export CARGO_BUILD_JOBS="${MAKE_JOBS}"
export WRKOBJDIR="${REPO_ROOT}/.wrk"
export DISTDIR="${REPO_ROOT}/.distfiles"
export PACKAGE_REPOSITORY="${REPO_ROOT}/.packages"
export PLIST_REPOSITORY="${REPO_ROOT}/.plist"
export BULK_COOKIES_DIR="${REPO_ROOT}/.bulk"
```

Canonical local directories (use these exact dotted names only):

- `.wrk/`
- `.distfiles/`
- `.packages/`
- `.plist/`
- `.bulk/`

Do not create or use undotted duplicates like `wrk/`, `distfiles/`,
`packages/`, `plist/`, or `bulk/`. Always source `env.sh` and rely on
those variables instead of overriding paths ad hoc on `make` command lines.

Load it for each shell session:

```sh
cd /path/to/openbsd-port-codex
. ./env.sh
```

Optional guard check before building:

```sh
for d in wrk distfiles packages plist bulk; do
  test ! -e "$d" || {
    echo "Unexpected undotted directory: $d (use .$d instead)"
    exit 1
  }
done
```

## Build Package

```sh
cd "${REPO_ROOT}/devel/codex"
make clean=all
make package
```

Output package path:

`${REPO_ROOT}/.packages/amd64/all/codex-0.105.0.tgz`

## Install Package

```sh
doas pkg_add -D unsigned "${REPO_ROOT}/.packages/amd64/all/codex-0.105.0.tgz"
```

## Quick Validation

```sh
cd "${REPO_ROOT}/devel/codex"
PORTSDIR_PATH="${REPO_ROOT}:/usr/ports:/usr/ports/mystuff" \
  /usr/ports/infrastructure/bin/portcheck -p "${REPO_ROOT}"
```
