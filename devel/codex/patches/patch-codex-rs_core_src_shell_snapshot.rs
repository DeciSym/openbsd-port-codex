Resolve bash from the environment in bash-specific snapshot tests.

OpenBSD installs bash under ${LOCALBASE}/bin, not /bin. These tests exercise
bash-only snapshot behavior, so keep requiring bash but resolve it through the
same shell lookup logic the application already uses.

Index: codex-rs/core/src/shell_snapshot.rs
--- codex-rs/core/src/shell_snapshot.rs.orig
+++ codex-rs/core/src/shell_snapshot.rs
@@ -648,7 +648,10 @@
     #[cfg(unix)]
     #[test]
     fn bash_snapshot_filters_invalid_exports() -> Result<()> {
-        let output = Command::new("/bin/bash")
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for bash snapshot tests")
+            .shell_path;
+        let output = Command::new(bash)
             .arg("-c")
             .arg(bash_snapshot_script())
             .env("BASH_ENV", "/dev/null")
@@ -673,7 +676,10 @@
     #[test]
     fn bash_snapshot_preserves_multiline_exports() -> Result<()> {
         let multiline_cert = "-----BEGIN CERTIFICATE-----\nabc\n-----END CERTIFICATE-----";
-        let output = Command::new("/bin/bash")
+        let bash = crate::shell::get_shell(ShellType::Bash, None)
+            .expect("bash should be available for bash snapshot tests")
+            .shell_path;
+        let output = Command::new(&bash)
             .arg("-c")
             .arg(bash_snapshot_script())
             .env("BASH_ENV", "/dev/null")
@@ -692,7 +698,7 @@
         let snapshot_path = dir.path().join("snapshot.sh");
         std::fs::write(&snapshot_path, stdout.as_bytes())?;
 
-        let validate = Command::new("/bin/bash")
+        let validate = Command::new(&bash)
             .arg("-c")
             .arg("set -e; . \"$1\"")
             .arg("bash")
@@ -715,7 +721,9 @@
         let dir = tempdir()?;
         let shell = Shell {
             shell_type: ShellType::Bash,
-            shell_path: PathBuf::from("/bin/bash"),
+            shell_path: crate::shell::get_shell(ShellType::Bash, None)
+                .expect("bash should be available for bash snapshot tests")
+                .shell_path,
             shell_snapshot: crate::shell::empty_shell_snapshot_receiver(),
         };
 
@@ -744,7 +752,9 @@
 
         let shell = Shell {
             shell_type: ShellType::Bash,
-            shell_path: PathBuf::from("/bin/bash"),
+            shell_path: crate::shell::get_shell(ShellType::Bash, None)
+                .expect("bash should be available for bash snapshot tests")
+                .shell_path,
             shell_snapshot: crate::shell::empty_shell_snapshot_receiver(),
         };
