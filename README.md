# lark-webhook-notify

A Rust library for sending rich notification cards to [Lark (Feishu)](https://www.feishu.cn) group bots via incoming webhooks.

Send structured cards for CI/CD pipelines, job status, alerts, and custom notifications — with built-in Chinese/English translations, HMAC-SHA256 request signing, and a fluent card builder API.

## Install

```toml
[dependencies]
lark-webhook-notify = "0.1"
```

## Quick Start

```rust
use lark_webhook_notify::{
    LarkWebhookNotifier, LarkWebhookSettings,
    network_submission_start, LanguageCode,
};

fn main() -> lark_webhook_notify::Result<()> {
    let settings = LarkWebhookSettings::load(
        None,                                        // config file (lark_webhook.toml)
        Some("https://open.feishu.cn/open-apis/bot/v2/hook/...".into()),
        Some("your_signing_secret".into()),
    )?;

    let notifier = LarkWebhookNotifier::new(settings)?;

    let card = network_submission_start(
        "my-network-set",
        "TCP",
        Some("my-group"),
        None, None,
        LanguageCode::En,
    );
    notifier.send_template(&card)?;
    Ok(())
}
```

## Configuration

Credentials are loaded in priority order (highest wins):

| Source | Fields |
|--------|--------|
| Direct params to `LarkWebhookSettings::load` | `webhook_url`, `webhook_secret` |
| Environment variables | `LARK_WEBHOOK_URL`, `LARK_WEBHOOK_SECRET` |
| TOML file (`lark_webhook.toml` or custom path) | `webhook_url`, `webhook_secret` |

Example `lark_webhook.toml`:
```toml
webhook_url    = "https://open.feishu.cn/open-apis/bot/v2/hook/..."
webhook_secret = "your_signing_secret"
```

Or provide credentials directly:

```rust
let notifier = LarkWebhookNotifier::from_params(
    "https://open.feishu.cn/open-apis/bot/v2/hook/...",
    "your_signing_secret",
)?;
```

## Pre-built Workflow Templates

For common pipeline and job notification patterns, use the module-level free functions.
All functions return a `GenericCardTemplate` that implements `LarkTemplate`.

### Network submission

```rust
use lark_webhook_notify::{
    network_submission_start,
    network_submission_complete,
    network_submission_failure,
    LanguageCode,
};

// Notify that submission started
let card = network_submission_start("my-nets", "TCP", Some("grp"), None, None, LanguageCode::En);
notifier.send_template(&card)?;

// Notify completion
let card = network_submission_complete("my-nets", Some(42), Some("grp"), None, None, None, LanguageCode::En);
notifier.send_template(&card)?;

// Notify failure
let card = network_submission_failure("my-nets", "connection refused", None, Some("grp"), LanguageCode::En);
notifier.send_template(&card)?;
```

### Job lifecycle

```rust
use lark_webhook_notify::{
    job_submission_start, job_submission_complete, job_submission_failure,
    job_complete, LanguageCode,
};

// Submission started
let card = job_submission_start("my-job", Some("training run"), Some("grp"), None, None, None, LanguageCode::En);
notifier.send_template(&card)?;

// Job finished
let card = job_complete(
    "my-job",
    true,          // success
    0,             // exit status
    Some("grp"),
    None, None, None, None, None,
    LanguageCode::En,
);
notifier.send_template(&card)?;
```

### Task set progress

```rust
use lark_webhook_notify::{task_set_progress, TaskSetProgress, LanguageCode};
use std::collections::HashMap;

let mut progress = HashMap::new();
progress.insert("set_a", TaskSetProgress { complete: 8, total: 10 });
progress.insert("set_b", TaskSetProgress { complete: 3, total: 10 });

let card = task_set_progress(&progress, "running", LanguageCode::En);
notifier.send_template(&card)?;
```

### Other workflow notifications

| Function | Header color | Use case |
|----------|-------------|----------|
| `config_upload_complete` | Green | Config/file upload finished |
| `result_collection_start` | Wathet | Result gathering started |
| `result_collection_complete` | Green | Result gathering finished |
| `comparison_complete` | Orange | Dataset comparison done |

## High-Level Template Structs

For per-template customization, instantiate the structs directly:

```rust
use lark_webhook_notify::{StartTaskTemplate, LanguageCode};

let card = StartTaskTemplate {
    task_name: "my-job".into(),
    desc: Some("long training run".into()),
    group: Some("experiments".into()),
    prefix: Some("runs/2026/".into()),
    msg: None,
    estimated_duration: Some("~4 hours".into()),
    language: LanguageCode::En,
};
notifier.send_template(&card)?;
```

Available template structs: `SimpleMessageTemplate`, `AlertTemplate`, `LegacyTaskTemplate`,
`StartTaskTemplate`, `ReportTaskResultTemplate`, `ReportFailureTaskTemplate`, `RawContentTemplate`.

## Custom Cards with `CardBuilder`

For full control, use the fluent `CardBuilder`:

```rust
use lark_webhook_notify::{CardBuilder, ColorTheme, TextAlign, TextSize};

let card = CardBuilder::new()
    .header("Deployment Complete", Some("success"), Some(ColorTheme::Green), None)
    .markdown("**Service:** api-gateway\n**Version:** v1.4.2", TextAlign::Left, TextSize::Normal)
    .columns()
        .column("Environment", Some("production"), "auto", 1)
        .column("Region", Some("us-east-1"), "weighted", 1)
    .end_columns()
    .collapsible("Deployment logs", "```\nDeploy OK\n```", false)
    .build();

notifier.send_template(&card)?;
```

### Builder methods

| Method | Description |
|--------|-------------|
| `.header(title, status, color, subtitle)` | Card header. `color = None` auto-detects from `status` text |
| `.markdown(content, align, size)` | Markdown text block |
| `.metadata(label, value)` | Single `**Label:** value` line |
| `.metadata_block(&[("Label", "val")])` | Multiple metadata lines in one block |
| `.columns()` / `.column(...)` / `.end_columns()` | Multi-column layout |
| `.collapsible(title, content, expanded)` | Collapsible panel |
| `.divider()` | Horizontal rule |
| `.add_block(block)` | Add any `impl Into<serde_json::Value>` block directly |

Auto color detection from `status`:

| Status text | Color |
|-------------|-------|
| `"running"`, `"submitted"` | Wathet (light blue) |
| `"success"`, `"completed"` | Green |
| `"failed"`, `"error"` | Red |
| `"warning"` | Orange |
| anything else | Blue |

## Language Support

All workflow functions and template structs accept a `LanguageCode` parameter:

```rust
use lark_webhook_notify::LanguageCode;

LanguageCode::Zh  // Chinese (default for most workflow functions)
LanguageCode::En  // English
```

Field labels (job name, status, timestamps) are automatically translated. User-supplied
strings (task names, descriptions, error messages) pass through unchanged.

## Convenience Functions

One-call wrappers for sending without constructing templates manually:

```rust
use lark_webhook_notify::{send_alert, SeverityLevel, LanguageCode};

send_alert(
    &notifier,
    "High memory usage",
    "Heap at 92%",
    SeverityLevel::Warning,
    None,
    LanguageCode::En,
)?;
```

## Block Types (low-level)

For assembling raw card JSON, the `blocks` module exports typed structs. All implement
`From<T> for serde_json::Value`:

```rust
use lark_webhook_notify::blocks::{Markdown, TextAlign, TextSize};

let v: serde_json::Value = Markdown {
    content: "**hello**".into(),
    text_align: TextAlign::Center,
    ..Default::default()
}.into();
```

Available: `Markdown`, `TextTag`, `HeaderBlock`, `Column`, `ColumnSet`,
`CollapsiblePanel`, `TemplateReference`, `Card`.

Layout enums (all re-exported from crate root): `TextAlign`, `TextSize`,
`HAlign`, `VAlign`, `BgStyle`, `ColumnWidth`.

## Error Handling

```rust
use lark_webhook_notify::{LarkWebhookError, Result};

match notifier.send_template(&card) {
    Ok(resp) => println!("sent: {resp}"),
    Err(LarkWebhookError::Config(msg)) => eprintln!("config error: {msg}"),
    Err(LarkWebhookError::ApiError { code, message }) => eprintln!("API {code}: {message}"),
    Err(e) => eprintln!("error: {e}"),
}
```

## License

Apache-2.0. See [LICENSE](LICENSE).
