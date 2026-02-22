Fallback to argv[0]/PATH when current_exe() is unavailable.

Index: codex-rs/arg0/src/lib.rs
--- codex-rs/arg0/src/lib.rs.orig
+++ codex-rs/arg0/src/lib.rs
@@ -230,7 +230,7 @@ pub fn prepend_path_entry_for_codex_aliases() -> std
         #[cfg(target_os = "linux")]
         LINUX_SANDBOX_ARG0,
     ] {
-        let exe = std::env::current_exe()?;
+        let exe = resolve_current_exe()?;
 
         #[cfg(unix)]
         {
@@ -269,6 +269,39 @@ pub fn prepend_path_entry_for_codex_aliases() -> std
     Ok(Arg0PathEntryGuard::new(temp_dir, lock_file))
 }
 
+fn resolve_current_exe() -> std::io::Result<PathBuf> {
+    if let Ok(exe) = std::env::current_exe() {
+        return Ok(exe);
+    }
+
+    let argv0 = std::env::args_os().next().unwrap_or_default();
+    let argv0_path = PathBuf::from(&argv0);
+
+    if argv0_path.is_absolute() && argv0_path.is_file() {
+        return Ok(argv0_path);
+    }
+
+    if argv0_path.components().count() > 1 {
+        if let Ok(cwd) = std::env::current_dir() {
+            let joined = cwd.join(&argv0_path);
+            if joined.is_file() {
+                return Ok(joined);
+            }
+        }
+    }
+
+    if let Some(path_var) = std::env::var_os("PATH") {
+        for dir in std::env::split_paths(&path_var) {
+            let candidate = dir.join(&argv0);
+            if candidate.is_file() {
+                return Ok(candidate);
+            }
+        }
+    }
+
+    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no current exe available"))
+}
+
 fn janitor_cleanup(temp_root: &Path) -> std::io::Result<()> {
     let entries = match std::fs::read_dir(temp_root) {
         Ok(entries) => entries,
