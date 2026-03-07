Resolve ripgrep via absolute path before invoking modified-time sorting.

On OpenBSD, invoking ripgrep as bare `rg` can make `std::env::current_exe()`
fail with "no current exe available (short)", which breaks
`--sortr=modified`. Resolving `rg` with `which` keeps behavior the same while
avoiding hardcoded `/usr/local/bin` paths.

Index: codex-rs/core/src/tools/handlers/grep_files.rs
--- codex-rs/core/src/tools/handlers/grep_files.rs.orig
+++ codex-rs/core/src/tools/handlers/grep_files.rs
@@ -113,7 +113,11 @@ async fn run_rg_search(
     limit: usize,
     cwd: &Path,
 ) -> Result<Vec<String>, FunctionCallError> {
-    let mut command = Command::new("rg");
+    let rg_program = which::which("rg")
+        .unwrap_or_else(|_| std::path::PathBuf::from("rg"));
+
+    let mut command = Command::new(rg_program);
+
     command
         .current_dir(cwd)
         .arg("--files-with-matches")
