Respect managed config override env var in release test builds.

`CODEX_APP_SERVER_MANAGED_CONFIG_PATH` is used by integration tests to point
the app server at a temporary managed config file. Upstream currently gates
this override behind `debug_assertions`, but ports run tests in release mode,
so the override is ignored and config-layer tests fail.

Index: codex-rs/app-server/src/main.rs
--- codex-rs/app-server/src/main.rs.orig
+++ codex-rs/app-server/src/main.rs
@@ -46,15 +46,12 @@ fn main() -> anyhow::Result<()> {
 }
 
 fn managed_config_path_from_debug_env() -> Option<PathBuf> {
-    #[cfg(debug_assertions)]
-    {
-        if let Ok(value) = std::env::var(MANAGED_CONFIG_PATH_ENV_VAR) {
-            return if value.is_empty() {
-                None
-            } else {
-                Some(PathBuf::from(value))
-            };
-        }
+    if let Ok(value) = std::env::var(MANAGED_CONFIG_PATH_ENV_VAR) {
+        return if value.is_empty() {
+            None
+        } else {
+            Some(PathBuf::from(value))
+        };
     }
 
     None
