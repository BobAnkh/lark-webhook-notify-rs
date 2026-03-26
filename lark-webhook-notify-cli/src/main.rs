use clap::{Parser, Subcommand};
use lark_webhook_notify::{
    send_alert, send_simple_message, send_task_failure, send_task_notification, send_task_result,
    send_task_start, CardContent, ColorTheme, LanguageCode, LarkWebhookNotifier,
    LarkWebhookSettings, LegacyTaskTemplate, RawContentTemplate, SeverityLevel,
};

#[derive(Parser)]
#[command(name = "lark-webhook-notify")]
#[command(about = "Send notifications to Lark webhook using predefined templates")]
struct Cli {
    /// Path to TOML config file (overrides default lark_webhook.toml)
    #[arg(long)]
    config: Option<String>,
    /// Override webhook URL
    #[arg(long)]
    webhook_url: Option<String>,
    /// Override webhook secret
    #[arg(long)]
    webhook_secret: Option<String>,
    /// Enable debug output
    #[arg(long)]
    debug: bool,
    /// Display language [zh, en]
    #[arg(long, default_value = "zh")]
    language: LanguageCode,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send task notification (auto-detects template based on status)
    Task {
        task_name: String,
        #[arg(long)]
        status: Option<i32>,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        desc: Option<String>,
        #[arg(long)]
        msg: Option<String>,
        #[arg(long)]
        duration: Option<String>,
        #[arg(long)]
        legacy: bool,
    },
    /// Send task start notification
    Start {
        task_name: String,
        #[arg(long)]
        desc: Option<String>,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        duration: Option<String>,
    },
    /// Send task result (status 0=success uses result template, non-zero uses failure template)
    Result {
        task_name: String,
        status: i32,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        desc: Option<String>,
        #[arg(long)]
        msg: Option<String>,
        #[arg(long)]
        duration: Option<String>,
    },
    /// Send legacy task notification
    Legacy {
        task_name: String,
        #[arg(long)]
        status: Option<i32>,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        summary: Option<String>,
    },
    /// Send alert notification
    Alert {
        title: String,
        message: String,
        #[arg(long, default_value = "warning")]
        severity: SeverityLevel,
        #[arg(long)]
        timestamp: Option<String>,
    },
    /// Send simple message notification
    Message {
        title: String,
        content: String,
        #[arg(long, default_value = "blue")]
        color: ColorTheme,
    },
    /// Send raw card content (JSON string)
    Raw { content: String },
    /// List available template types and usage examples
    Templates,
    /// Send a test notification to verify configuration
    Test,
}

fn main() {
    let cli = Cli::parse();
    let exit_code = run(cli);
    std::process::exit(exit_code);
}

fn run(cli: Cli) -> i32 {
    let url = cli.webhook_url.as_deref();
    let secret = cli.webhook_secret.as_deref();
    let cfg = cli.config.as_deref();
    let lang = cli.language;

    let result = match cli.command {
        Commands::Task {
            task_name,
            status,
            group,
            prefix,
            desc,
            msg,
            duration,
            legacy,
        } => send_task_notification(
            &task_name,
            status,
            group.as_deref(),
            prefix.as_deref(),
            desc.as_deref(),
            msg.as_deref(),
            duration.as_deref(),
            legacy,
            lang,
            url,
            secret,
            cfg,
        ),
        Commands::Start {
            task_name,
            desc,
            group,
            prefix,
            duration,
        } => send_task_start(
            &task_name,
            desc.as_deref(),
            group.as_deref(),
            prefix.as_deref(),
            duration.as_deref(),
            lang,
            url,
            secret,
            cfg,
        ),
        Commands::Result {
            task_name,
            status,
            group,
            prefix,
            desc,
            msg,
            duration,
        } => {
            if status == 0 {
                send_task_result(
                    &task_name,
                    status,
                    group.as_deref(),
                    prefix.as_deref(),
                    desc.as_deref(),
                    msg.as_deref(),
                    duration.as_deref(),
                    None,
                    lang,
                    url,
                    secret,
                    cfg,
                )
            } else {
                send_task_failure(
                    &task_name,
                    status,
                    group.as_deref(),
                    prefix.as_deref(),
                    desc.as_deref(),
                    msg.as_deref(),
                    duration.as_deref(),
                    None,
                    lang,
                    url,
                    secret,
                    cfg,
                )
            }
        }
        Commands::Legacy {
            task_name,
            status,
            group,
            prefix,
            summary,
        } => {
            let settings =
                LarkWebhookSettings::load(cfg, url.map(str::to_owned), secret.map(str::to_owned));
            match settings.and_then(LarkWebhookNotifier::new) {
                Ok(notifier) => {
                    let t = LegacyTaskTemplate {
                        task_name,
                        status,
                        group: group.unwrap_or_default(),
                        prefix: prefix.unwrap_or_default(),
                        task_summary: summary.unwrap_or_default(),
                        language: lang,
                    };
                    notifier.send_template(&t)
                }
                Err(e) => Err(e),
            }
        }
        Commands::Alert {
            title,
            message,
            severity,
            timestamp,
        } => send_alert(
            &title,
            &message,
            severity,
            timestamp.as_deref(),
            lang,
            url,
            secret,
            cfg,
        ),
        Commands::Message {
            title,
            content,
            color,
        } => send_simple_message(&title, &content, color, lang, url, secret, cfg),
        Commands::Raw { content } => match serde_json::from_str::<CardContent>(&content) {
            Ok(card_content) => {
                let settings = LarkWebhookSettings::load(
                    cfg,
                    url.map(str::to_owned),
                    secret.map(str::to_owned),
                );
                match settings.and_then(LarkWebhookNotifier::new) {
                    Ok(notifier) => {
                        let t = RawContentTemplate {
                            content: card_content,
                            language: lang,
                        };
                        notifier.send_template(&t)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => {
                eprintln!("Invalid JSON: {e}");
                return 1;
            }
        },
        Commands::Templates => {
            println!("Available template types:");
            println!("  task     - Auto-detect template from status");
            println!("  start    - Task start notification");
            println!("  result   - Task completion/result notification");
            println!("  legacy   - Legacy compatible format");
            println!("  alert    - Severity-based alert notification");
            println!("  message  - Simple text message");
            println!("  raw      - Raw card content passthrough");
            println!();
            println!("Usage patterns:");
            println!("  lark-webhook-notify start 'task-name' --desc 'Description'");
            println!("  lark-webhook-notify result 'task-name' 0 --duration '5 min'");
            println!("  lark-webhook-notify result 'task-name' 1 --msg 'Error details'");
            println!("  lark-webhook-notify alert 'Title' 'Message' --severity error");
            println!("  lark-webhook-notify message 'Title' 'Content' --color green");
            return 0;
        }
        Commands::Test => send_simple_message(
            "Test Notification",
            "This is a test message from lark-webhook-notify",
            ColorTheme::Blue,
            lang,
            url,
            secret,
            cfg,
        ),
    };

    match result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_task_start() {
        let cli = Cli::parse_from(["lark-webhook-notify", "start", "my-task", "--desc", "hello"]);
        match cli.command {
            Commands::Start {
                task_name, desc, ..
            } => {
                assert_eq!(task_name, "my-task");
                assert_eq!(desc.as_deref(), Some("hello"));
            }
            _ => panic!("expected Start"),
        }
    }

    #[test]
    fn test_parse_alert() {
        let cli = Cli::parse_from([
            "lark-webhook-notify",
            "alert",
            "MyTitle",
            "MyMsg",
            "--severity",
            "error",
        ]);
        match cli.command {
            Commands::Alert {
                title,
                message,
                severity,
                ..
            } => {
                assert_eq!(title, "MyTitle");
                assert_eq!(message, "MyMsg");
                assert_eq!(severity.as_str(), "error");
            }
            _ => panic!("expected Alert"),
        }
    }

    #[test]
    fn test_parse_result() {
        let cli = Cli::parse_from([
            "lark-webhook-notify",
            "result",
            "my-task",
            "0",
            "--duration",
            "5m",
        ]);
        match cli.command {
            Commands::Result {
                task_name,
                status,
                duration,
                ..
            } => {
                assert_eq!(task_name, "my-task");
                assert_eq!(status, 0);
                assert_eq!(duration.as_deref(), Some("5m"));
            }
            _ => panic!("expected Result"),
        }
    }

    #[test]
    fn test_parse_language() {
        let cli = Cli::parse_from([
            "lark-webhook-notify",
            "--language",
            "en",
            "message",
            "T",
            "C",
        ]);
        assert_eq!(cli.language.to_string(), "en");
    }

    #[test]
    fn test_parse_raw() {
        let cli = Cli::parse_from(["lark-webhook-notify", "raw", r#"{"schema":"2.0"}"#]);
        match cli.command {
            Commands::Raw { content } => assert!(content.contains("schema")),
            _ => panic!("expected Raw"),
        }
    }
}
