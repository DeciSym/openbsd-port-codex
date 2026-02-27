# shellcheck shell=sh
# Source this from the repository root:
#   cd /path/to/openbsd-port-codex
#   . ./env.sh
REPO_ROOT=${REPO_ROOT:-$(pwd -P)}
MAKE_JOBS=$(sysctl -n hw.ncpuonline)

export REPO_ROOT
export PORTSDIR_PATH="${REPO_ROOT}:/usr/ports:/usr/ports/mystuff"
export MAKE_JOBS
export CARGO_BUILD_JOBS="${MAKE_JOBS}"
export WRKOBJDIR="${REPO_ROOT}/.wrk"
export DISTDIR="${REPO_ROOT}/.distfiles"
export PACKAGE_REPOSITORY="${REPO_ROOT}/.packages"
export PLIST_REPOSITORY="${REPO_ROOT}/.plist"
export BULK_COOKIES_DIR="${REPO_ROOT}/.bulk"
