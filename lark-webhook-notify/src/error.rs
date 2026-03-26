#[derive(thiserror::Error, Debug)]
pub enum LarkWebhookError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Lark API error: code {code} - {message}")]
    ApiError { code: i64, message: String },
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, LarkWebhookError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let e = LarkWebhookError::Config("missing url".to_owned());
        assert_eq!(e.to_string(), "Configuration error: missing url");
    }

    #[test]
    fn test_api_error_display() {
        let e = LarkWebhookError::ApiError {
            code: 99,
            message: "bad request".to_owned(),
        };
        assert_eq!(e.to_string(), "Lark API error: code 99 - bad request");
    }
}
