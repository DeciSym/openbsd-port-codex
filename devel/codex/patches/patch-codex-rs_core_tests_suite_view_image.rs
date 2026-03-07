Fix release-only view_image test compile by importing missing wiremock symbols.

Index: codex-rs/core/tests/suite/view_image.rs
--- codex-rs/core/tests/suite/view_image.rs.orig
+++ codex-rs/core/tests/suite/view_image.rs
@@ -41,6 +41,8 @@ use serde_json::Value;
 use tokio::time::Duration;
 use wiremock::BodyPrintLimit;
 use wiremock::MockServer;
+use wiremock::ResponseTemplate;
+use wiremock::matchers::body_string_contains;
 
 fn image_messages(body: &Value) -> Vec<&Value> {
     body.get("input")
