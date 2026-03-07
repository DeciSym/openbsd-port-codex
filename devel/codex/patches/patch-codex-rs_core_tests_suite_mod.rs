Keep integration test arg0 setup compatible with release-mode hardening.

The arg0 helper setup now refuses CODEX_HOME under std::env::temp_dir() in
release builds. The integration test ctor created CODEX_HOME via tempfile(),
which uses the temp root and causes a startup panic before tests run.

Use a test-owned directory under the current workspace directory instead.

Index: codex-rs/core/tests/suite/mod.rs
--- codex-rs/core/tests/suite/mod.rs.orig
+++ codex-rs/core/tests/suite/mod.rs
@@ -21,9 +21,11 @@ pub static CODEX_ALIASES_TEMP_DIR: TestCodexAliasesGuard = unsafe {
 #[ctor]
 pub static CODEX_ALIASES_TEMP_DIR: TestCodexAliasesGuard = unsafe {
     #[allow(clippy::unwrap_used)]
+    // arg0 helper creation rejects CODEX_HOME under std::env::temp_dir() in
+    // release builds, so keep this test-owned home under the workspace cwd.
     let codex_home = tempfile::Builder::new()
         .prefix("codex-core-tests")
-        .tempdir()
+        .tempdir_in(std::env::current_dir().unwrap())
         .unwrap();
     let previous_codex_home = std::env::var_os(CODEX_HOME_ENV_VAR);
     // arg0_dispatch() creates helper links under CODEX_HOME/tmp. Point it at a
