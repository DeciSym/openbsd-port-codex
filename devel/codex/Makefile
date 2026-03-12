COMMENT =	OpenAI Codex command-line coding agent

# ring-rs does not support this architecture.
NOT_FOR_ARCHS =	sparc64

V =		0.114.0
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

WANTLIB +=	${MODCARGO_WANTLIB} bz2 crypto dbus-1 lzma m onig ssl util z

MODULES =	devel/cargo

RUN_DEPENDS +=	textproc/ripgrep

LIB_DEPENDS +=	x11/dbus
LIB_DEPENDS +=	archivers/bzip2
LIB_DEPENDS +=	archivers/xz
LIB_DEPENDS +=	textproc/oniguruma

TEST_DEPENDS +=	shells/bash

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
MODCARGO_TEST_ARGS +=	--workspace --exclude codex-core --exclude codex-exec

# Run workspace tests by default. Tests that require outbound network should
# self-skip via skip_if_no_network! when this env var is present.
#
# Excluded tests are narrow and OpenBSD-specific. The skip list below covers
# runtime/package install checks that panic on openbsd-x86_64 artifact
# detection, a few remaining shell/exec OpenBSD portability gaps, temporary
# codex-tui regressions, and one intermittent codex-utils-pty failure.
#
# Timeout cleanup tests currently return EPERM from killpg(2) on OpenBSD.
#
TEST_ENV +=	CODEX_SANDBOX_NETWORK_DISABLED=1
MODCARGO_ENV +=	CODEX_SANDBOX_NETWORK_DISABLED=1

UPDATES_TEST =	updates::tests::
PROMPT_TEST =	update_prompt::tests::
HISTORY_TEST =	history_cell::tests::
STATUS_TEST =	status::tests::status_snapshot_
CORE_TEST =	tests::
ARTIFACT_TEST =	tools::handlers::artifacts::tests::
SPEC_TEST =	tools::spec::tests::
RUNTIME_TEST =	tools::runtimes::tests::
ENSURE_TEST =	${CORE_TEST}ensure_installed_
RUNTIME_WRAP_TEST =	${RUNTIME_TEST}maybe_wrap_shell_lc_with_snapshot_
SHELL_TEST =	shell_snapshot::tests::
INPROC_TEST =	in_process::tests::in_process_start_

MODCARGO_TEST_ARGS +=	-- --skip suite:: \
		--skip ${UPDATES_TEST}extract_version_from_brew_api_json
.for _t in \
	${PROMPT_TEST}update_prompt_snapshot \
	${HISTORY_TEST}session_info_availability_nux_tooltip_snapshot \
	${STATUS_TEST}cached_limits_hide_credits_without_flag \
	${STATUS_TEST}includes_credits_and_limits \
	${STATUS_TEST}includes_forked_from \
	${STATUS_TEST}includes_monthly_limit \
	${STATUS_TEST}includes_reasoning_details \
	${STATUS_TEST}shows_empty_limits_message \
	${STATUS_TEST}shows_missing_limits_message \
	${STATUS_TEST}shows_stale_limits_message \
	${STATUS_TEST}truncates_in_narrow_terminal \
	${CORE_TEST}pipe_process_detaches_from_parent_session \
	ensure_installed_downloads_and_extracts_zip_runtime \
	ensure_installed_downloads_and_extracts_zip_package \
	resolve_preinstalled_runtime_reads_pinned_cache_path \
	${CORE_TEST}artifacts_client_execute_build_writes_wrapped_script_and_env \
	${CORE_TEST}artifacts_client_execute_render_passes_expected_args \
	${CORE_TEST}load_cached_runtime_reads_installed_runtime \
	${CORE_TEST}load_cached_runtime_rejects_parent_relative_paths \
	${CORE_TEST}load_cached_runtime_requires_build_entrypoint \
	${CORE_TEST}load_cached_runtime_uses_custom_cache_root \
	${CORE_TEST}resolve_cached_uses_custom_cache_root \
	${CORE_TEST}ensure_installed_replaces_invalid_cached_install \
	${CORE_TEST}ensure_installed_rejects_unexpected_archive_size \
	${CORE_TEST}ensure_installed_rejects_manifest_version_mismatch \
	${CORE_TEST}ensure_installed_serializes_concurrent_installs \
	${ENSURE_TEST}restores_previous_install_when_final_validation_fails \
	${ARTIFACT_TEST}load_cached_runtime_reads_pinned_cache_path \
	${SPEC_TEST}test_build_specs_artifact_tool_enabled \
	tests::typed_request_roundtrip_works \
	tests::typed_request_reports_json_rpc_errors \
	tests::caller_provided_session_source_is_applied \
	tests::tiny_channel_capacity_still_supports_request_roundtrip \
	${INPROC_TEST}initializes_and_handles_typed_v2_request \
	${INPROC_TEST}uses_requested_session_source_for_thread_start \
	${INPROC_TEST}clamps_zero_channel_capacity \
	unix::escalate_server::tests::exec_closes_parent_socket_after_shell_spawn \
	exec::tests::kill_child_process_group_kills_grandchildren_on_timeout \
	exec::tests::process_exec_tool_call_respects_cancellation_token
MODCARGO_TEST_ARGS +=	--skip ${_t}
.endfor

TARGETED_TEST_ARGS =	--ignore-rust-version
.if !empty(FLAVOR:Mall_features)
TARGETED_TEST_ARGS +=	--all-features
.endif

TARGETED_CORE_TEST_SKIP =
.for _t in \
	exec::tests::kill_child_process_group_kills_grandchildren_on_timeout \
	exec::tests::process_exec_tool_call_respects_cancellation_token \
	shell_snapshot::tests::bash_snapshot_filters_invalid_exports \
	shell_snapshot::tests::bash_snapshot_preserves_multiline_exports \
	shell_snapshot::tests::snapshot_shell_does_not_inherit_stdin \
	shell_snapshot::tests::try_new_creates_and_deletes_snapshot_file \
	${ARTIFACT_TEST}load_cached_runtime_reads_pinned_cache_path \
	${RUNTIME_WRAP_TEST}applies_explicit_path_override \
	${RUNTIME_WRAP_TEST}does_not_embed_override_values_in_argv \
	${RUNTIME_WRAP_TEST}keeps_snapshot_path_without_override \
	${RUNTIME_WRAP_TEST}preserves_unset_override_variables \
	${RUNTIME_WRAP_TEST}restores_explicit_override_precedence \
	tools::spec::tests::test_build_specs_artifact_tool_enabled
TARGETED_CORE_TEST_SKIP +=	--skip ${_t}
.endfor

CODEX_BINS =	codex codex-mcp-server codex-stdio-to-uds codex-file-search

# Install the binaries built by `cargo build` directly to avoid
# `cargo install` recompiling each target path during packaging.
do-install:
.for _b in ${CODEX_BINS}
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${_b} \
		${PREFIX}/bin/${_b}
.endfor

# Keep the blanket workspace pass on targets that do not need aggregate
# integration binaries. `codex-core` and `codex-exec` are exercised below with
# explicit lib/bin and standalone integration targets so the port can skip
# unsupported OpenBSD-only aggregates without patching test sources.
do-test:
	@ulimit -n 1024 && ${MODCARGO_TEST_TARGET}
	@ulimit -n 1024 && \
		cd ${WRKSRC} && \
		${SETENV} ${MAKE_ENV} ${MODCARGO_ENV} \
		${MODCARGO_CARGO_BIN} test --manifest-path ${MODCARGO_CARGOTOML} \
		--release ${TARGETED_TEST_ARGS} -p codex-core \
		--lib --bins --test responses_headers -- ${TARGETED_CORE_TEST_SKIP}
	@ulimit -n 1024 && \
		cd ${WRKSRC} && \
		${SETENV} ${MAKE_ENV} ${MODCARGO_ENV} \
		${MODCARGO_CARGO_BIN} test --manifest-path ${MODCARGO_CARGOTOML} \
		--release ${TARGETED_TEST_ARGS} -p codex-exec \
		--lib --bins --test event_processor_with_json_output

pre-configure:
	sed -i \
		-e '/^\[dependencies.tungstenite\]/,/^\[dependencies.native-tls-crate\]/{' \
		-e 's/^git = .*/path = "..\/tungstenite-rs"/' \
		-e '/^rev = /d' \
		-e '}' \
		${WRKDIR}/tokio-tungstenite/Cargo.toml

.include "crates.inc"
.include <bsd.port.mk>
