Limit exec sandbox integration tests to supported platforms.

These tests provide helper implementations only for Linux and macOS.
OpenBSD matches `unix` but has no corresponding helper, causing
compile-time E0425 errors in release test builds.

Index: codex-rs/exec/tests/suite/sandbox.rs
--- codex-rs/exec/tests/suite/sandbox.rs.orig
+++ codex-rs/exec/tests/suite/sandbox.rs
@@ -1,4 +1,4 @@
-#![cfg(unix)]
+#![cfg(any(target_os = "linux", target_os = "macos"))]
 use codex_core::spawn::StdioPolicy;
 use codex_protocol::protocol::SandboxPolicy;
 use codex_utils_absolute_path::AbsolutePathBuf;
