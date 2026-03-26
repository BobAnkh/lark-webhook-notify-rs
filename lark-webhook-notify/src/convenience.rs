use serde_json::Value;

use crate::client::LarkWebhookNotifier;
use crate::config::LarkWebhookSettings;
use crate::error::Result;
use crate::templates::{
    AlertTemplate, ColorTheme, LanguageCode, LegacyTaskTemplate, ReportFailureTaskTemplate,
    ReportTaskResultTemplate, SeverityLevel, SimpleMessageTemplate, StartTaskTemplate,
};

fn make_notifier(
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<LarkWebhookNotifier> {
    let settings = LarkWebhookSettings::load(
        config_file,
        webhook_url.map(|s| s.to_owned()),
        webhook_secret.map(|s| s.to_owned()),
    )?;
    LarkWebhookNotifier::new(settings)
}

#[allow(clippy::too_many_arguments)]
pub fn send_task_notification(
    task_name: &str,
    status: Option<i32>,
    group: Option<&str>,
    prefix: Option<&str>,
    desc: Option<&str>,
    msg: Option<&str>,
    duration: Option<&str>,
    legacy_format: bool,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    if legacy_format {
        let t = LegacyTaskTemplate {
            task_name: task_name.to_owned(),
            status,
            group: group.unwrap_or("").to_owned(),
            prefix: prefix.unwrap_or("").to_owned(),
            task_summary: msg.unwrap_or("").to_owned(),
            language,
        };
        notifier.send_template(&t)
    } else {
        match status {
            None => {
                let t = StartTaskTemplate {
                    task_name: task_name.to_owned(),
                    desc: desc.map(str::to_owned),
                    group: group.map(str::to_owned),
                    prefix: prefix.map(str::to_owned),
                    msg: msg.map(str::to_owned),
                    estimated_duration: duration.map(str::to_owned),
                    language,
                };
                notifier.send_template(&t)
            }
            Some(s) if s == 0 => {
                // title not exposed here; use send_task_result for custom titles
                let t = ReportTaskResultTemplate {
                    task_name: task_name.to_owned(),
                    status: s,
                    group: group.map(str::to_owned),
                    prefix: prefix.map(str::to_owned),
                    desc: desc.map(str::to_owned),
                    msg: msg.map(str::to_owned),
                    duration: duration.map(str::to_owned),
                    title: None,
                    language,
                };
                notifier.send_template(&t)
            }
            Some(s) => {
                // title not exposed here; use send_task_failure for custom titles
                let t = ReportFailureTaskTemplate {
                    task_name: task_name.to_owned(),
                    status: s,
                    group: group.map(str::to_owned),
                    prefix: prefix.map(str::to_owned),
                    desc: desc.map(str::to_owned),
                    msg: msg.map(str::to_owned),
                    duration: duration.map(str::to_owned),
                    title: None,
                    language,
                };
                notifier.send_template(&t)
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn send_alert(
    title: &str,
    message: &str,
    severity: SeverityLevel,
    timestamp: Option<&str>,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    let ts = timestamp
        .map(str::to_owned)
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    let t = AlertTemplate {
        title: title.to_owned(),
        message: message.to_owned(),
        severity,
        timestamp: ts,
        language,
    };
    notifier.send_template(&t)
}

pub fn send_simple_message(
    title: &str,
    content: &str,
    color: ColorTheme,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    let t = SimpleMessageTemplate {
        title: title.to_owned(),
        content: content.to_owned(),
        color,
        language,
    };
    notifier.send_template(&t)
}

#[allow(clippy::too_many_arguments)]
pub fn send_task_start(
    task_name: &str,
    desc: Option<&str>,
    group: Option<&str>,
    prefix: Option<&str>,
    estimated_duration: Option<&str>,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    let t = StartTaskTemplate {
        task_name: task_name.to_owned(),
        desc: desc.map(str::to_owned),
        group: group.map(str::to_owned),
        prefix: prefix.map(str::to_owned),
        msg: None,
        estimated_duration: estimated_duration.map(str::to_owned),
        language,
    };
    notifier.send_template(&t)
}

#[allow(clippy::too_many_arguments)]
pub fn send_task_result(
    task_name: &str,
    status: i32,
    group: Option<&str>,
    prefix: Option<&str>,
    desc: Option<&str>,
    msg: Option<&str>,
    duration: Option<&str>,
    title: Option<&str>,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    let t = ReportTaskResultTemplate {
        task_name: task_name.to_owned(),
        status,
        group: group.map(str::to_owned),
        prefix: prefix.map(str::to_owned),
        desc: desc.map(str::to_owned),
        msg: msg.map(str::to_owned),
        duration: duration.map(str::to_owned),
        title: title.map(str::to_owned),
        language,
    };
    notifier.send_template(&t)
}

#[allow(clippy::too_many_arguments)]
pub fn send_task_failure(
    task_name: &str,
    status: i32,
    group: Option<&str>,
    prefix: Option<&str>,
    desc: Option<&str>,
    msg: Option<&str>,
    duration: Option<&str>,
    title: Option<&str>,
    language: LanguageCode,
    webhook_url: Option<&str>,
    webhook_secret: Option<&str>,
    config_file: Option<&str>,
) -> Result<Value> {
    let notifier = make_notifier(webhook_url, webhook_secret, config_file)?;
    let t = ReportFailureTaskTemplate {
        task_name: task_name.to_owned(),
        status,
        group: group.map(str::to_owned),
        prefix: prefix.map(str::to_owned),
        desc: desc.map(str::to_owned),
        msg: msg.map(str::to_owned),
        duration: duration.map(str::to_owned),
        title: title.map(str::to_owned),
        language,
    };
    notifier.send_template(&t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{ColorTheme, LanguageCode, SeverityLevel};

    #[test]
    fn test_send_simple_message_no_config_returns_error() {
        let result = send_simple_message(
            "title",
            "content",
            ColorTheme::Blue,
            LanguageCode::Zh,
            None,
            None,
            None,
        );
        assert!(result.is_err());
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("Configuration") || err_str.contains("webhook"),
            "unexpected error: {err_str}"
        );
    }

    #[test]
    fn test_send_alert_no_config_returns_error() {
        let result = send_alert(
            "title",
            "message",
            SeverityLevel::Warning,
            None,
            LanguageCode::Zh,
            None,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_send_task_notification_no_config_returns_error() {
        let result = send_task_notification(
            "my-task",
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            LanguageCode::Zh,
            None,
            None,
            None,
        );
        assert!(result.is_err());
    }
}
