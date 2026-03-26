use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;

use crate::config::LarkWebhookSettings;
use crate::error::{LarkWebhookError, Result};
use crate::templates::{CardContent, LarkTemplate};

/// HTTP client for sending cards to a Lark (Feishu) group bot webhook.
///
/// Handles HMAC-SHA256 request signing automatically on every send.
///
/// # Construction
///
/// Use [`LarkWebhookNotifier::from_params`] for quick setup, or
/// [`LarkWebhookNotifier::new`] with a [`LarkWebhookSettings`] loaded from
/// environment variables or a TOML file:
///
/// ```no_run
/// use lark_webhook_notify::{LarkWebhookNotifier, LarkWebhookSettings};
///
/// # fn main() -> lark_webhook_notify::Result<()> {
/// // From env vars / config file
/// let settings = LarkWebhookSettings::load(None, None, None)?;
/// let notifier = LarkWebhookNotifier::new(settings)?;
///
/// // Or directly
/// let notifier = LarkWebhookNotifier::from_params("https://...", "secret")?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct LarkWebhookNotifier {
    webhook_url: String,
    webhook_secret: String,
    client: reqwest::blocking::Client,
}

impl LarkWebhookNotifier {
    /// Create a notifier from a [`LarkWebhookSettings`] instance.
    ///
    /// Returns an error if `webhook_url` or `webhook_secret` is missing or empty.
    pub fn new(settings: LarkWebhookSettings) -> Result<Self> {
        let webhook_url = settings.webhook_url.ok_or_else(|| {
            LarkWebhookError::Config(
                "webhook_url is required. Set via LARK_WEBHOOK_URL env var, config file, or direct param".to_owned()
            )
        })?;
        let webhook_secret = settings.webhook_secret.ok_or_else(|| {
            LarkWebhookError::Config(
                "webhook_secret is required. Set via LARK_WEBHOOK_SECRET env var, config file, or direct param".to_owned()
            )
        })?;
        if webhook_url.is_empty() {
            return Err(LarkWebhookError::Config(
                "webhook_url cannot be empty".to_owned(),
            ));
        }
        if webhook_secret.is_empty() {
            return Err(LarkWebhookError::Config(
                "webhook_secret cannot be empty".to_owned(),
            ));
        }
        Ok(Self {
            webhook_url,
            webhook_secret,
            client: reqwest::blocking::Client::new(),
        })
    }

    /// Create a notifier by passing the webhook URL and signing secret directly.
    pub fn from_params(url: &str, secret: &str) -> Result<Self> {
        if url.is_empty() {
            return Err(LarkWebhookError::Config(
                "webhook_url cannot be empty".to_owned(),
            ));
        }
        if secret.is_empty() {
            return Err(LarkWebhookError::Config(
                "webhook_secret cannot be empty".to_owned(),
            ));
        }
        Ok(Self {
            webhook_url: url.to_owned(),
            webhook_secret: secret.to_owned(),
            client: reqwest::blocking::Client::new(),
        })
    }

    /// Render `template` and send the resulting card to the webhook.
    pub fn send_template(&self, template: &dyn LarkTemplate) -> Result<Value> {
        let content = template.generate();
        self.send_raw_content(content)
    }

    /// Send a pre-built [`CardContent`] JSON value directly.
    pub fn send_raw_content(&self, content: CardContent) -> Result<Value> {
        let payload = self.create_payload(content);
        self.send_payload(payload)
    }

    fn create_payload(&self, content: CardContent) -> Value {
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let sign = gen_sign(&timestamp, &self.webhook_secret);
        json!({
            "timestamp": timestamp,
            "sign": sign,
            "msg_type": "interactive",
            "card": content,
        })
    }

    fn send_payload(&self, payload: Value) -> Result<Value> {
        let resp = self.client.post(&self.webhook_url).json(&payload).send()?;
        let resp_data: Value = resp.error_for_status()?.json()?;
        if let Some(code) = resp_data.get("code").and_then(|c| c.as_i64()) {
            if code != 0 {
                let message = resp_data
                    .get("msg")
                    .and_then(|m| m.as_str())
                    .unwrap_or("unknown error")
                    .to_owned();
                return Err(LarkWebhookError::ApiError { code, message });
            }
        }
        Ok(resp_data)
    }
}

pub(crate) fn gen_sign(timestamp: &str, secret: &str) -> String {
    let key = format!("{timestamp}\n{secret}");
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts any key length");
    mac.update(b"");
    let result = mac.finalize();
    base64::engine::general_purpose::STANDARD.encode(result.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_sign_non_empty() {
        let result = gen_sign("1234567890", "test_secret");
        // base64 of 32-byte SHA256 = 44 chars
        assert_eq!(result.len(), 44);
        // valid base64 — decode must succeed
        use base64::Engine;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&result)
            .unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn test_gen_sign_deterministic() {
        let a = gen_sign("ts", "sec");
        let b = gen_sign("ts", "sec");
        assert_eq!(a, b);
    }

    #[test]
    fn test_gen_sign_different_inputs() {
        let a = gen_sign("ts1", "sec");
        let b = gen_sign("ts2", "sec");
        assert_ne!(a, b);
    }

    #[test]
    fn test_gen_sign_known_vector() {
        // Pre-computed: HMAC-SHA256(key="1234567890\ntest_secret", msg=b"") as standard base64
        let expected = "3H7JNC7ltBAwibQHFO1KFVN9HTkLtm2virjdsmGcAzw=";
        assert_eq!(gen_sign("1234567890", "test_secret"), expected);
    }

    #[test]
    fn test_from_params_missing_url() {
        let result = LarkWebhookNotifier::from_params("", "secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_params_missing_secret() {
        let result = LarkWebhookNotifier::from_params("https://example.com", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_params_ok() {
        let result = LarkWebhookNotifier::from_params("https://example.com", "secret");
        assert!(result.is_ok());
    }
}
