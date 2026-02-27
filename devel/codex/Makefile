COMMENT =	OpenAI Codex command-line coding agent

V =		0.106.0
DISTNAME =	codex-rust-v${V}
PKGNAME =	codex-${V}

# git dependencies from codex-rs/Cargo.toml [patch.crates-io]
CROSSTERM_COMMIT =	87db8bfa6dc99427fd3b071681b07fc31c6ce995
RATATUI_COMMIT =	9b2ad1298408c45918ee9f8241a6f95498cdbed2
TOKIO_TUNGSTENITE_COMMIT =	132f5b39c862e3a970f731d709608b3e6276d5f6
TUNGSTENITE_COMMIT =	9200079d3b54a1ff51072e24d81fd354f085156f
NUCLEO_COMMIT =	4253de9faabb4e5c6d81d946a5e35a90f87347ee
RULES_RUST_COMMIT =	b56cbaa8465e74127f1ea216f813cd377295ad81

DIST_TUPLE +=	github openai codex rust-v${V} .
DIST_TUPLE +=	github nornagon crossterm ${CROSSTERM_COMMIT} ../crossterm
DIST_TUPLE +=	github nornagon ratatui ${RATATUI_COMMIT} ../ratatui
DIST_TUPLE +=	github openai-oss-forks tokio-tungstenite \
	${TOKIO_TUNGSTENITE_COMMIT} ../tokio-tungstenite
DIST_TUPLE +=	github openai-oss-forks tungstenite-rs \
	${TUNGSTENITE_COMMIT} ../tungstenite-rs
DIST_TUPLE +=	github helix-editor nucleo ${NUCLEO_COMMIT} ../nucleo
DIST_TUPLE +=	github dzbarsky rules_rust ${RULES_RUST_COMMIT} ../rules_rust

WRKSRC =	${WRKDIR}/${DISTNAME}/codex-rs

CATEGORIES =	devel
HOMEPAGE =	https://github.com/openai/codex

MAINTAINER =	Donald Anthony Pellegrino Jr., Ph.D. <don@decisym.ai>

# Apache-2.0
PERMIT_PACKAGE =	Yes

FLAVORS =	all_features

WANTLIB +=	${MODCARGO_WANTLIB} dbus-1 lzma m

MODULES =	devel/cargo

LIB_DEPENDS +=	x11/dbus
LIB_DEPENDS +=	archivers/xz

CONFIGURE_STYLE =	cargo
SEPARATE_BUILD =	Yes
MODCARGO_CRATES_KEEP +=	libsqlite3-sys
MODCARGO_CRATES_KEEP +=	zstd-sys
MODCARGO_BUILD_ARGS +=	--ignore-rust-version
MODCARGO_TEST_ARGS +=	--ignore-rust-version

.if !empty(FLAVOR:Mall_features)
MODCARGO_BUILD_ARGS +=	--all-features
MODCARGO_TEST_ARGS +=	--all-features
.endif

CODEX_BINS =	codex codex-mcp-server codex-stdio-to-uds codex-file-search

# Install the binaries built by `cargo build` directly to avoid
# `cargo install` recompiling each target path during packaging.
do-install:
.for _b in ${CODEX_BINS}
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${_b} \
		${PREFIX}/bin/${_b}
.endfor

pre-configure:
	sed -i \
		-e '/^\[dependencies.tungstenite\]/,/^\[dependencies.native-tls-crate\]/{' \
		-e 's/^git = .*/path = "..\/tungstenite-rs"/' \
		-e '/^rev = /d' \
		-e '}' \
		${WRKDIR}/tokio-tungstenite/Cargo.toml

.include "crates.inc"
.include <bsd.port.mk>
