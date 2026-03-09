Resolve bash from the environment in runtime snapshot tests.

These tests execute rewritten bash commands to verify snapshot environment
precedence. On OpenBSD, bash is installed under ${LOCALBASE}/bin, so resolve it
via the existing shell lookup logic instead of hardcoding /bin/bash.

Index: codex-rs/core/src/tools/runtimes/mod.rs
--- codex-rs/core/src/tools/runtimes/mod.rs.orig
+++ codex-rs/core/src/tools/runtimes/mod.rs
@@ -422,14 +422,19 @@
             "# Snapshot file\nexport TEST_ENV_SNAPSHOT=global\nexport SNAPSHOT_ONLY=from_snapshot\n",
         )
         .expect("write snapshot");
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for runtime snapshot tests")
+            .shell_path
+            .to_string_lossy()
+            .into_owned();
         let session_shell = shell_with_snapshot(
             ShellType::Bash,
-            "/bin/bash",
+            &bash,
             snapshot_path,
             dir.path().to_path_buf(),
         );
         let command = vec![
-            "/bin/bash".to_string(),
+            bash.clone(),
             "-lc".to_string(),
             "printf '%s|%s' \"$TEST_ENV_SNAPSHOT\" \"${SNAPSHOT_ONLY-unset}\"".to_string(),
         ];
@@ -463,14 +468,19 @@
             "# Snapshot file\nexport PATH='/snapshot/bin'\n",
         )
         .expect("write snapshot");
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for runtime snapshot tests")
+            .shell_path
+            .to_string_lossy()
+            .into_owned();
         let session_shell = shell_with_snapshot(
             ShellType::Bash,
-            "/bin/bash",
+            &bash,
             snapshot_path,
             dir.path().to_path_buf(),
         );
         let command = vec![
-            "/bin/bash".to_string(),
+            bash.clone(),
             "-lc".to_string(),
             "printf '%s' \"$PATH\"".to_string(),
         ];
@@ -498,14 +508,19 @@
             "# Snapshot file\nexport PATH='/snapshot/bin'\n",
         )
         .expect("write snapshot");
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for runtime snapshot tests")
+            .shell_path
+            .to_string_lossy()
+            .into_owned();
         let session_shell = shell_with_snapshot(
             ShellType::Bash,
-            "/bin/bash",
+            &bash,
             snapshot_path,
             dir.path().to_path_buf(),
         );
         let command = vec![
-            "/bin/bash".to_string(),
+            bash.clone(),
             "-lc".to_string(),
             "printf '%s' \"$PATH\"".to_string(),
         ];
@@ -536,14 +551,19 @@
             "# Snapshot file\nexport OPENAI_API_KEY='snapshot-value'\n",
         )
         .expect("write snapshot");
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for runtime snapshot tests")
+            .shell_path
+            .to_string_lossy()
+            .into_owned();
         let session_shell = shell_with_snapshot(
             ShellType::Bash,
-            "/bin/bash",
+            &bash,
             snapshot_path,
             dir.path().to_path_buf(),
         );
         let command = vec![
-            "/bin/bash".to_string(),
+            bash.clone(),
             "-lc".to_string(),
             "printf '%s' \"$OPENAI_API_KEY\"".to_string(),
         ];
@@ -580,14 +600,19 @@
             "# Snapshot file\nexport CODEX_TEST_UNSET_OVERRIDE='snapshot-value'\n",
         )
         .expect("write snapshot");
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for runtime snapshot tests")
+            .shell_path
+            .to_string_lossy()
+            .into_owned();
         let session_shell = shell_with_snapshot(
             ShellType::Bash,
-            "/bin/bash",
+            &bash,
             snapshot_path,
             dir.path().to_path_buf(),
         );
         let command = vec![
-            "/bin/bash".to_string(),
+            bash.clone(),
             "-lc".to_string(),
             "if [ \"${CODEX_TEST_UNSET_OVERRIDE+x}\" = x ]; then printf 'set:%s' \"$CODEX_TEST_UNSET_OVERRIDE\"; else printf 'unset'; fi".to_string(),
         ];
