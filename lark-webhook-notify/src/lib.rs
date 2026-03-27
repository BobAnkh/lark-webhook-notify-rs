//! Send rich notification cards to [Lark (Feishu)](https://www.feishu.cn) group bots
//! via incoming webhooks.
//!
//! ## Getting started
//!
//! ```no_run
//! use lark_webhook_notify::{LarkWebhookNotifier, job_complete, LanguageCode};
//!
//! # fn main() -> lark_webhook_notify::Result<()> {
//! let notifier = LarkWebhookNotifier::from_params(
//!     "https://open.feishu.cn/open-apis/bot/v2/hook/...",
//!     "your_signing_secret",
//! )?;
//!
//! let card = job_complete(
//!     "my-training-job", true, 0,
//!     Some("experiments"), None, None, None, None, None,
//!     LanguageCode::En,
//! );
//! notifier.send_template(&card)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The library has three layers:
//!
//! 1. **Workflow functions** ([`workflow`]) — high-level, opinionated notifications for
//!    common pipeline events (network submission, job lifecycle, comparisons, etc.).
//!    Use these for the fastest path to a good-looking notification.
//!
//! 2. **Template structs** ([`templates`]) — mid-level structs implementing [`LarkTemplate`].
//!    Instantiate directly when you need to set every field explicitly, or subclass with
//!    your own `impl LarkTemplate`.
//!
//! 3. **[`CardBuilder`]** — fluent builder for fully custom cards. Accepts any combination
//!    of markdown, columns, collapsible panels, and raw block values.
//!
//! All three layers produce a value that implements [`LarkTemplate`], which you pass to
//! [`LarkWebhookNotifier::send_template`].

pub mod blocks;
pub mod builder;
pub mod client;
pub mod config;
pub mod convenience;
pub mod error;
pub mod templates;
pub mod workflow;

// Re-export the main public API
pub use blocks::{BgStyle, ColumnWidth, HAlign, TextAlign, TextSize, VAlign};
pub use builder::CardBuilder;
pub use client::LarkWebhookNotifier;
pub use config::LarkWebhookSettings;
pub use convenience::{
    send_alert, send_simple_message, send_task_failure, send_task_notification, send_task_result,
    send_task_start,
};
pub use error::{LarkWebhookError, Result};
pub use templates::{
    AlertTemplate, CardContent, ColorTheme, GenericCardTemplate, LanguageCode, LarkTemplate,
    LegacyTaskTemplate, RawContentTemplate, ReportFailureTaskTemplate, ReportTaskResultTemplate,
    SeverityLevel, SimpleMessageTemplate, StartTaskTemplate, get_translation,
};
pub use workflow::{
    TaskSetProgress, comparison_complete, config_upload_complete, create_custom_template,
    job_complete, job_submission_complete, job_submission_failure, job_submission_start,
    network_submission_complete, network_submission_failure, network_submission_start,
    result_collection_complete, result_collection_start, task_set_progress,
};
