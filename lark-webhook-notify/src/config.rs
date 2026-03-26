// lark-webhook-notify/src/config.rs
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use crate::error::{LarkWebhookError, Result};

/// Webhook credentials used to construct a [`LarkWebhookNotifier`].
///
/// Load via [`LarkWebhookSettings::load`] (supports env vars + TOML file),
/// or set fields directly.
///
/// # Required fields
///
/// Both `webhook_url` and `webhook_secret` must be `Some` before passing to
/// [`LarkWebhookNotifier::new`].
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct LarkWebhookSettings {
    /// The full incoming-webhook URL from the Lark bot configuration page.
    pub webhook_url: Option<String>,
    /// The signing secret used to generate HMAC-SHA256 request signatures.
    pub webhook_secret: Option<String>,
}

impl LarkWebhookSettings {
    /// Load config with figment hierarchy (lowest → highest priority):
    /// 1. Defaults (all None)
    /// 2. TOML file (lark_webhook.toml or custom path)
    /// 3. LARK_WEBHOOK_URL / LARK_WEBHOOK_SECRET env vars
    /// 4. Direct params
    pub fn load(
        toml_file: Option<&str>,
        webhook_url: Option<String>,
        webhook_secret: Option<String>,
    ) -> Result<Self> {
        let path = toml_file.unwrap_or("lark_webhook.toml");
        let mut settings: LarkWebhookSettings =
            Figment::from(Serialized::defaults(LarkWebhookSettings::default()))
                .merge(Toml::file(path))
                .merge(Env::prefixed("LARK_"))
                .extract()
                .map_err(|e| LarkWebhookError::Config(e.to_string()))?;

        if let Some(url) = webhook_url {
            settings.webhook_url = Some(url);
        }
        if let Some(secret) = webhook_secret {
            settings.webhook_secret = Some(secret);
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serialize tests that read/write env vars to prevent parallel interference.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_defaults_are_none() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // Run without any env vars or files set
        // Use a non-existent toml file path so figment skips it gracefully
        let s = LarkWebhookSettings::load(Some("nonexistent_xyz.toml"), None, None).unwrap();
        assert!(s.webhook_url.is_none());
        assert!(s.webhook_secret.is_none());
    }

    #[test]
    fn test_direct_params_override() {
        let _guard = ENV_MUTEX.lock().unwrap();
        let s = LarkWebhookSettings::load(
            None,
            Some("https://direct.url".to_owned()),
            Some("direct_secret".to_owned()),
        )
        .unwrap();
        assert_eq!(s.webhook_url.as_deref(), Some("https://direct.url"));
        assert_eq!(s.webhook_secret.as_deref(), Some("direct_secret"));
    }

    #[test]
    fn test_toml_file_loading() {
        let _guard = ENV_MUTEX.lock().unwrap();
        use std::io::Write;
        let mut f = tempfile::NamedTempFile::new().unwrap();
        write!(
            f,
            "webhook_url = \"https://toml.url\"\nwebhook_secret = \"toml_secret\"\n"
        )
        .unwrap();
        let path = f.path().to_str().unwrap().to_owned();
        let s = LarkWebhookSettings::load(Some(&path), None, None).unwrap();
        assert_eq!(s.webhook_url.as_deref(), Some("https://toml.url"));
        assert_eq!(s.webhook_secret.as_deref(), Some("toml_secret"));
    }

    #[test]
    fn test_env_var_loading() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // NOTE: env var tests can interfere with each other if run in parallel.
        // The ENV_MUTEX above serializes all env-sensitive tests in this module.
        // This test cleans up after itself before asserting.
        unsafe {
            std::env::set_var("LARK_WEBHOOK_URL", "https://env.url");
            std::env::set_var("LARK_WEBHOOK_SECRET", "env_secret");
        }
        let result = LarkWebhookSettings::load(Some("nonexistent_xyz.toml"), None, None);
        unsafe {
            std::env::remove_var("LARK_WEBHOOK_URL");
            std::env::remove_var("LARK_WEBHOOK_SECRET");
        }
        let s = result.unwrap();
        assert_eq!(s.webhook_url.as_deref(), Some("https://env.url"));
        assert_eq!(s.webhook_secret.as_deref(), Some("env_secret"));
    }
}
