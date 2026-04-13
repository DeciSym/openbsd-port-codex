# Project Notes for the OpenBSD Port of Codex

Building a new package from a new upstream release can take hours when
performing all the tests. When updating the port for new upstream
releases, track the time spent in each step so that timing estimates
for new releases can be identified and used to inform future
performance improvements. Linking operations in particular seem to
take a long time and should be measured. Long wall-clock is acceptable
if it is justified.

Create patches only when changes are needed specifically to account
for OpenBSD Porting. Do not patch tests. Exclude the tests instead to
be patched upstream.

For upstream release updates, query GitHub releases and select the
latest non-prerelease tag before changing the port. The releases feed
is often dominated by alpha tags, so "latest release" alone is not a
reliable stable-version signal.

Use this release-bump sequence unless there is a concrete reason to
deviate:
1. Verify the chosen stable upstream tag and check whether the
   existing OpenBSD patches still apply against the new sources.
2. Update `V` in the port `Makefile`, then run `make makesum`.
3. Regenerate crates with `make modcargo-gen-crates`, but diff only
   from the first `MODCARGO_CRATES` line onward; the command emits
   framework chatter before the generated list.
4. Run `make checkpatch` before the expensive build/test steps.
5. Run stateful targets sequentially on one `WRKDIR`: `make
   port-lib-depends-check`, `make update-plist`, `make package`, and
   `make test`. Do not overlap them. Parallel runs can attempt to
   re-patch an already-patched tree and leave `.orig`/`.rej`
   artifacts.
6. Run `portcheck -p /path/to/ports/tree` after the build/package/test
   validation is complete.

When running `portcheck` against a local ports clone that is outside the
system `PORTSDIR_PATH`, export `PORTSDIR_PATH=/path/to/tree:/usr/ports:/usr/ports/mystuff`
for the `portcheck` invocation as well. Running `portcheck -A` from the
tree root can also false-positive on `crates.inc` because it expects the
full relative path in `.include`; use the non-`-A` form from the port
directory for `devel/codex`.

When a `Cargo.toml` patch applies with fuzz after an upstream workspace
reshuffle, inspect the result before trusting the patch phase. During
the 0.120.0 update, `patch-Cargo_toml-v8poc` deleted
`codex-shell-command` from `workspace.dependencies` instead of
removing only `codex-v8-poc`, which caused Cargo workspace dependency
inheritance to fail before compilation. Refresh those hunks to exact
line ranges rather than relying on fuzzy matches.

For flavored builds in this tree, use `env FLAVOR=... make ...`
instead of `make FLAVOR=...`. The ports framework rejects the latter
form here. Expect separate flavored work and package paths such as
`.wrk/codex-<version>-all_features` and
`.packages/${MACHINE_ARCH}/all/codex-<version>-all_features.tgz`.

When a new upstream test fails, inspect whether it is actually reading
host state instead of staying inside its temporary fixtures. During the
0.116.0 update, `config_api::tests::batch_write_reloads_user_config_when_requested`
and several `app-server-client` tests used default config loaders,
resolved the host `codex_home`, and failed with `PermissionDenied`
while loading analytics config under ports test execution. Treat that
class of failure as an OpenBSD port test exclusion, not as a source
patch.

Shell snapshot tests need a second check: if a `codex-core`
`shell_snapshot::tests::try_new_*` case fails from
`ShellSnapshot::try_new()` with `validation_failed`, treat it as the
same OpenBSD shell-snapshot portability gap as the other targeted
`shell_snapshot` exclusions. Add a narrow `TARGETED_CORE_TEST_SKIP`
entry instead of patching the test source.

During the 0.120.0 update, two more `codex-core` targeted tests turned
out to be host-tool portability checks rather than product regressions:
`exec::tests::process_exec_tool_call_preserves_full_buffer_capture_policy`
uses GNU `head -c`, which OpenBSD `head` does not implement, and
`tools::runtimes::tests::maybe_wrap_shell_lc_with_snapshot_restores_codex_thread_id_from_env`
hardcodes `/bin/bash` instead of the OpenBSD `bash` path. Treat both as
`TARGETED_CORE_TEST_SKIP` exclusions.

For `codex-utils-pty`, distinguish PTY inherited-FD failures from the
pipe path. During the 0.116.0 update,
`tests::pty_spawn_can_preserve_inherited_fds` exited `1` on OpenBSD
while `pipe_spawn_no_stdin_can_preserve_inherited_fds` still passed.
Treat that PTY-only mismatch as an OpenBSD test exclusion in the main
workspace skip list.

Re-check that assumption on later releases. During the 0.120.0 update,
`tests::pipe_spawn_no_stdin_can_preserve_inherited_fds` also exited `1`
on OpenBSD even though `/dev/fd/$fd` worked from the shell, so the
preserved-FD pipe exit mismatch is now the same OpenBSD exclusion class
as the PTY inherited-FD case.

When a new test failure looks like a shared-state race, confirm it
before extending the skip list. During the 0.116.0 `all_features`
update, `tests::list_all_connectors_uses_shared_cache` failed under the
workspace runner because `codex-connectors` stores only one global
cache entry and its sibling test overwrote that slot in parallel. The
already-built test binary under
`target/release/deps/codex_connectors-*` passed both when run with the
exact test name and when run with `--test-threads=1`. Use that pattern
to distinguish flaky shared-state tests from real OpenBSD behavior
breakage.

If `codex-exec` upstream collapses standalone integration tests into a
single aggregate `all` target, inspect whether that target still
compiles on OpenBSD before wiring it into `do-test`. During the 0.120.0
update, `exec/tests/all.rs` pulled in `tests/suite/sandbox.rs`, which
only defines `spawn_command_under_sandbox()` and Linux/macOS
`writable_roots` helpers behind `#[cfg(target_os = "linux"|"macos")]`.
That aggregate test target does not compile on OpenBSD, so keep the
targeted `codex-exec` pass to `--lib --bins` until upstream restores a
portable standalone integration target.

Measured timings for the 0.120.0 update on 2026-04-13:
- final `make makesum`: 14.46s real
- `make modcargo-gen-crates`: 54.03s real
- final `env FLAVOR=all_features make checkpatch`: 55.60s real
- `env FLAVOR=all_features make port-lib-depends-check`: 1302.97s real;
  the flavored release build phase reported `Finished 'release'
  profile [optimized] target(s) in 19m 44s`
- `env FLAVOR=all_features make update-plist`: 2.21s real
- `env FLAVOR=all_features make package`: 16.35s real
- first `env FLAVOR=all_features make test`: 1258.19s real; failed in
  `codex-app-server-client` after the workspace pass
- targeted-core rerun after the first skip update: 999.40s real;
  `codex-core` rebuilt in 14m 13s before failing on GNU `head -c` and
  `/bin/bash` assumptions
- hot rerun after those core skips: 50.79s real; failed at
  `tests::pipe_spawn_no_stdin_can_preserve_inherited_fds`
- hot rerun after the `codex-utils-pty` skip: 146.39s real; failed
  because upstream renamed the `codex-exec` integration target
- rerun with `codex-exec --test all`: 654.25s real; failed because the
  aggregate target pulled in `sandbox.rs`, which does not compile on
  OpenBSD
- final `env FLAVOR=all_features make test`: 147.24s real; passed after
  limiting the targeted `codex-exec` pass to `--lib --bins`

Measured timings for the 0.116.0 update on 2026-03-25:
- `make makesum`: 8.64s real
- `make modcargo-gen-crates`: 44.38s real
- `make checkpatch`: 0.64s real
- `make port-lib-depends-check`: 1494.38s real; the cargo build phase
  reported `Finished 'release' profile [optimized] target(s) in 24m
  10s`
- `make update-plist`: 1.33s real
- `make package`: 11.60s real
- `env FLAVOR=all_features make checkpatch`: 44.08s real on the first
  flavored pass; 0.72s real after the skip-list update
- `env FLAVOR=all_features make port-lib-depends-check`: 1473.89s real;
  the flavored release build phase reported `Finished 'release'
  profile [optimized] target(s) in 23m 50s`
- `env FLAVOR=all_features make package`: 11.86s real
- first `env FLAVOR=all_features make test`: 1537.43s real; failed at
  `tests::list_all_connectors_uses_shared_cache` after the workspace
  compile phase reported `Finished 'release' profile [optimized]
  target(s) in 25m 31s`
- second `env FLAVOR=all_features make test`: 2408.38s real; passed
  after targeted `codex-core` lib tests rebuilt in 24m 40s and the
  targeted `codex-exec` pass rebuilt for another 13m 12s

The dominant cost remains the release build and link steps. Measure
those first when estimating a new upstream update. For `all_features`,
the slowest tail is no longer the workspace pass itself: the explicit
targeted `codex-core` and `codex-exec` test invocations can exceed the
earlier package build time and should be included in any estimate.
