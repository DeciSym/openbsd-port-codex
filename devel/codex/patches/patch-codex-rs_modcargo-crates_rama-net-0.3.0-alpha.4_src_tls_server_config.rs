Fix Rust 1.90 compatibility: avoid unstable Duration::from_hours.

Index: codex-rs/modcargo-crates/rama-net-0.3.0-alpha.4/src/tls/server/config.rs
--- codex-rs/modcargo-crates/rama-net-0.3.0-alpha.4/src/tls/server/config.rs.orig
+++ codex-rs/modcargo-crates/rama-net-0.3.0-alpha.4/src/tls/server/config.rs
@@ -93,7 +93,7 @@ impl Default for CacheKind {
     fn default() -> Self {
         Self::MemCache {
             max_size: CACHE_KIND_DEFAULT_MAX_SIZE,
-            ttl: Some(std::time::Duration::from_hours(24 * 7)), // 7 days
+            ttl: Some(std::time::Duration::from_secs(24 * 7 * 60 * 60)), // 7 days
         }
     }
 }
