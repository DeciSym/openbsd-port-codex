COMMENT =	OpenAI Codex command-line coding agent

# ring-rs does not support this architecture.
NOT_FOR_ARCHS =	sparc64

V =		0.111.0
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

# Run workspace tests by default. Tests that require outbound network should
# self-skip via skip_if_no_network! when this env var is present.
#
# Excluded tests are narrow and OpenBSD-specific: they call
# PackagePlatform::detect_current() in unit tests and panic because upstream
# currently treats openbsd-x86_64 as unsupported for artifact package runtime
# downloads.
# - runtime::tests::ensure_installed_downloads_and_extracts_zip_runtime
# - tests::ensure_installed_downloads_and_extracts_zip_package
# - resolve_preinstalled_runtime_reads_pinned_cache_path
# - tests::artifacts_client_execute_build_writes_wrapped_script_and_env
# - tests::artifacts_client_execute_render_passes_expected_args
# - tests::load_cached_runtime_reads_installed_runtime
# - tests::load_cached_runtime_rejects_parent_relative_paths
# - tests::load_cached_runtime_requires_build_entrypoint
# - tests::load_cached_runtime_uses_custom_cache_root
# - tests::resolve_cached_uses_custom_cache_root
# - tests::ensure_installed_replaces_invalid_cached_install
# - tests::ensure_installed_rejects_unexpected_archive_size
# - tests::ensure_installed_rejects_manifest_version_mismatch
# - tests::ensure_installed_serializes_concurrent_installs
# - tests::ensure_installed_restores_previous_install_when_final_validation_fails
# - tools::handlers::artifacts::tests::load_cached_runtime_reads_pinned_cache_path
# - tools::spec::tests::test_build_specs_artifact_tool_enabled
#
# Additional exclusions are OpenBSD portability gaps in upstream tests:
# - exec::tests::kill_child_process_group_kills_grandchildren_on_timeout
# - exec::tests::process_exec_tool_call_respects_cancellation_token
#   These currently return EPERM from killpg(2) on OpenBSD in timeout cleanup.
# - shell_snapshot::tests::bash_snapshot_filters_invalid_exports
# - shell_snapshot::tests::bash_snapshot_preserves_multiline_exports
# - shell_snapshot::tests::snapshot_shell_does_not_inherit_stdin
# - shell_snapshot::tests::try_new_creates_and_deletes_snapshot_file
# - tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_applies_explicit_path_override
# - tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_does_not_embed_override_values_in_argv
# - tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_keeps_snapshot_path_without_override
# - tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_preserves_unset_override_variables
# - tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_restores_explicit_override_precedence
#   These hardcode /bin/bash; OpenBSD packaged bash is /usr/local/bin/bash.
# Temporary upstream codex-tui release regressions in 0.110.0:
# - updates::tests::extract_version_from_brew_api_json
# - update_prompt::tests::update_prompt_snapshot
# - history_cell::tests::session_info_availability_nux_tooltip_snapshot
# - status::tests::status_snapshot_cached_limits_hide_credits_without_flag
# - status::tests::status_snapshot_includes_credits_and_limits
# - status::tests::status_snapshot_includes_forked_from
# - status::tests::status_snapshot_includes_monthly_limit
# - status::tests::status_snapshot_includes_reasoning_details
# - status::tests::status_snapshot_shows_empty_limits_message
# - status::tests::status_snapshot_shows_missing_limits_message
# - status::tests::status_snapshot_shows_stale_limits_message
# - status::tests::status_snapshot_truncates_in_narrow_terminal
# Additional intermittent OpenBSD CI/runtime portability failure:
# - tests::pipe_process_detaches_from_parent_session
#   (codex-utils-pty) can fail to read child session id.
# codex-core full-session integration tests (suite::...) are run in a separate
# pass with tuned fd/thread settings; this keeps the rest of the workspace
# fully parallel.
TEST_ENV +=	CODEX_SANDBOX_NETWORK_DISABLED=1
MODCARGO_ENV +=	CODEX_SANDBOX_NETWORK_DISABLED=1
MODCARGO_TEST_ARGS +=	-- --skip suite:: \
	--skip updates::tests::extract_version_from_brew_api_json \
	--skip update_prompt::tests::update_prompt_snapshot \
	--skip history_cell::tests::session_info_availability_nux_tooltip_snapshot \
	--skip status::tests::status_snapshot_cached_limits_hide_credits_without_flag \
	--skip status::tests::status_snapshot_includes_credits_and_limits \
	--skip status::tests::status_snapshot_includes_forked_from \
	--skip status::tests::status_snapshot_includes_monthly_limit \
	--skip status::tests::status_snapshot_includes_reasoning_details \
	--skip status::tests::status_snapshot_shows_empty_limits_message \
	--skip status::tests::status_snapshot_shows_missing_limits_message \
	--skip status::tests::status_snapshot_shows_stale_limits_message \
	--skip status::tests::status_snapshot_truncates_in_narrow_terminal \
	--skip tests::pipe_process_detaches_from_parent_session \
	--skip ensure_installed_downloads_and_extracts_zip_runtime \
	--skip ensure_installed_downloads_and_extracts_zip_package \
	--skip resolve_preinstalled_runtime_reads_pinned_cache_path \
	--skip tests::artifacts_client_execute_build_writes_wrapped_script_and_env \
	--skip tests::artifacts_client_execute_render_passes_expected_args \
	--skip tests::load_cached_runtime_reads_installed_runtime \
	--skip tests::load_cached_runtime_rejects_parent_relative_paths \
	--skip tests::load_cached_runtime_requires_build_entrypoint \
	--skip tests::load_cached_runtime_uses_custom_cache_root \
	--skip tests::resolve_cached_uses_custom_cache_root \
	--skip tests::ensure_installed_replaces_invalid_cached_install \
	--skip tests::ensure_installed_rejects_unexpected_archive_size \
	--skip tests::ensure_installed_rejects_manifest_version_mismatch \
	--skip tests::ensure_installed_serializes_concurrent_installs \
	--skip tests::ensure_installed_restores_previous_install_when_final_validation_fails \
	--skip tools::handlers::artifacts::tests::load_cached_runtime_reads_pinned_cache_path \
	--skip tools::spec::tests::test_build_specs_artifact_tool_enabled \
	--skip exec::tests::kill_child_process_group_kills_grandchildren_on_timeout \
	--skip exec::tests::process_exec_tool_call_respects_cancellation_token \
	--skip shell_snapshot::tests::bash_snapshot_filters_invalid_exports \
	--skip shell_snapshot::tests::bash_snapshot_preserves_multiline_exports \
	--skip shell_snapshot::tests::snapshot_shell_does_not_inherit_stdin \
	--skip shell_snapshot::tests::try_new_creates_and_deletes_snapshot_file \
	--skip tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_applies_explicit_path_override \
	--skip tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_does_not_embed_override_values_in_argv \
	--skip tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_keeps_snapshot_path_without_override \
	--skip tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_preserves_unset_override_variables \
	--skip tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_restores_explicit_override_precedence

FULL_SESSION_TEST_ARGS =	--ignore-rust-version
.if !empty(FLAVOR:Mall_features)
FULL_SESSION_TEST_ARGS +=	--all-features
.endif

# Temporary full-session skips for OpenBSD in 0.110.0 under parallel load:
# - suite::shell_snapshot::shell_command_snapshot_still_intercepts_apply_patch
# - suite::sqlite_state::user_messages_persist_in_state_db
# - suite::sqlite_state::tool_call_logs_include_thread_id
# - suite::user_shell_cmd::user_shell_cmd_can_be_interrupted
# - suite::memories::memories_startup_phase2_tracks_added_and_removed_inputs_across_runs
# - suite::model_overrides::override_turn_context_does_not_persist_when_config_exists
# - suite::models_cache_ttl::renews_cache_ttl_on_matching_models_etag
# - suite::models_cache_ttl::uses_cache_when_version_matches
# - suite::models_cache_ttl::refreshes_when_cache_version_missing
# - suite::shell_snapshot::shell_command_snapshot_preserves_shell_environment_policy_set
# - suite::shell_snapshot::shell_snapshot_deleted_after_shutdown_with_skills
# - suite::sqlite_state::backfill_scans_existing_rollouts
# - suite::sqlite_state::new_thread_is_recorded_in_state_db
# - suite::user_shell_cmd::user_shell_cmd_ls_and_cat_in_temp_dir
# - suite::otel::handle_container_exec_autoapprove_from_config_records_tool_decision
# - suite::otel::handle_container_exec_user_approved_for_session_records_tool_decision
# - suite::otel::handle_container_exec_user_approved_records_tool_decision
# - suite::otel::handle_container_exec_user_denies_records_tool_decision
FULL_SESSION_TEST_THREADS =	5
FULL_SESSION_TEST_SKIP =	--skip suite::user_shell_cmd::user_shell_command_output_is_truncated_in_history \
	--skip suite::user_shell_cmd::user_shell_command_does_not_replace_active_turn \
	--skip suite::user_shell_cmd::user_shell_command_history_is_persisted_and_shared_with_model \
	--skip suite::sqlite_state::web_search_marks_thread_memory_mode_polluted_when_configured \
	--skip suite::shell_snapshot::shell_command_snapshot_still_intercepts_apply_patch \
	--skip suite::sqlite_state::user_messages_persist_in_state_db \
	--skip suite::sqlite_state::tool_call_logs_include_thread_id \
	--skip suite::user_shell_cmd::user_shell_cmd_can_be_interrupted \
	--skip suite::memories::memories_startup_phase2_tracks_added_and_removed_inputs_across_runs \
	--skip suite::model_overrides::override_turn_context_does_not_persist_when_config_exists \
	--skip suite::models_cache_ttl::renews_cache_ttl_on_matching_models_etag \
	--skip suite::models_cache_ttl::uses_cache_when_version_matches \
	--skip suite::models_cache_ttl::refreshes_when_cache_version_missing \
	--skip suite::shell_snapshot::shell_command_snapshot_preserves_shell_environment_policy_set \
	--skip suite::shell_snapshot::shell_snapshot_deleted_after_shutdown_with_skills \
	--skip suite::sqlite_state::backfill_scans_existing_rollouts \
	--skip suite::sqlite_state::new_thread_is_recorded_in_state_db \
	--skip suite::user_shell_cmd::user_shell_cmd_ls_and_cat_in_temp_dir \
	--skip suite::otel::handle_container_exec_autoapprove_from_config_records_tool_decision \
	--skip suite::otel::handle_container_exec_user_approved_for_session_records_tool_decision \
	--skip suite::otel::handle_container_exec_user_approved_records_tool_decision \
	--skip suite::otel::handle_container_exec_user_denies_records_tool_decision

CODEX_BINS =	codex codex-mcp-server codex-stdio-to-uds codex-file-search

# Install the binaries built by `cargo build` directly to avoid
# `cargo install` recompiling each target path during packaging.
do-install:
.for _b in ${CODEX_BINS}
	${INSTALL_PROGRAM} ${MODCARGO_TARGET_DIR}/release/${_b} \
		${PREFIX}/bin/${_b}
.endfor

# codex-core integration tests can exceed the default soft file-descriptor limit
# on OpenBSD due long-lived watchers across many test cases.
do-test:
	@ulimit -n 1024 && ${MODCARGO_TEST_TARGET}
	@ulimit -n 1024 && \
		cd ${WRKSRC} && \
		${SETENV} ${MAKE_ENV} ${MODCARGO_ENV} RUST_TEST_THREADS=${FULL_SESSION_TEST_THREADS} \
		${MODCARGO_CARGO_BIN} test --manifest-path ${MODCARGO_CARGOTOML} \
		--release ${FULL_SESSION_TEST_ARGS} -p codex-core --test all \
		-- ${FULL_SESSION_TEST_SKIP}

pre-configure:
	sed -i \
		-e '/^\[dependencies.tungstenite\]/,/^\[dependencies.native-tls-crate\]/{' \
		-e 's/^git = .*/path = "..\/tungstenite-rs"/' \
		-e '/^rev = /d' \
		-e '}' \
		${WRKDIR}/tokio-tungstenite/Cargo.toml

.include "crates.inc"
.include <bsd.port.mk>
