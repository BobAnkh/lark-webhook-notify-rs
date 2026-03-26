use chrono::Local;
use std::collections::HashMap;

use crate::blocks::{TextAlign, TextSize};
use crate::builder::CardBuilder;
use crate::templates::{get_translation, ColorTheme, GenericCardTemplate, LanguageCode};

/// Progress counters for a single task set, used by [`task_set_progress`].
pub struct TaskSetProgress {
    /// Number of tasks completed so far.
    pub complete: u32,
    /// Total number of tasks in the set.
    pub total: u32,
}

pub fn network_submission_start(
    network_set_name: &str,
    network_type: &str,
    group: Option<&str>,
    prefix: Option<&str>,
    metadata: Option<&serde_json::Value>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let metadata_text = format!(
        "**{}:** {}\n**{}:** {}",
        t("network_set_name"),
        network_set_name,
        t("network_type"),
        network_type,
    );
    let status_text = t("running");
    let mut builder = CardBuilder::new()
        .header(
            &t("network_submission_started"),
            Some(&status_text),
            Some(ColorTheme::Wathet),
            None,
        )
        .markdown(&metadata_text, TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(&t("group"), Some(group.unwrap_or("*unknown*")), "auto", 1)
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(meta) = metadata {
        let s = serde_json::to_string_pretty(meta).unwrap_or_default();
        builder = builder.collapsible(
            &t("metadata_overview"),
            &format!("```json\n{s}\n```"),
            false,
        );
    }
    builder.build()
}

pub fn network_submission_complete(
    network_set_name: &str,
    submitted_count: Option<u32>,
    group: Option<&str>,
    prefix: Option<&str>,
    duration: Option<&str>,
    metadata: Option<&serde_json::Value>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let mut lines = vec![format!(
        "**{}:** {}",
        t("network_set_name"),
        network_set_name
    )];
    if let Some(c) = submitted_count {
        lines.push(format!("**{}:** {c}", t("submitted_count")));
    }
    if let Some(d) = duration {
        lines.push(format!("**{}:** {d}", t("duration")));
    }

    let status_text = t("success");
    let mut builder = CardBuilder::new()
        .header(
            &t("network_submission_complete"),
            Some(&status_text),
            Some(ColorTheme::Green),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(&t("group"), Some(group.unwrap_or("*unknown*")), "auto", 1)
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(meta) = metadata {
        let s = serde_json::to_string_pretty(meta).unwrap_or_default();
        builder = builder.collapsible(
            &t("metadata_overview"),
            &format!("```json\n{s}\n```"),
            false,
        );
    }
    builder.build()
}

pub fn network_submission_failure(
    network_set_name: &str,
    error_message: &str,
    submitted_count: Option<u32>,
    group: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let mut lines = vec![
        format!("**{}:** {}", t("network_set_name"), network_set_name),
        format!("**{}:** {}", t("group"), group.unwrap_or("*unknown*")),
    ];
    if let Some(c) = submitted_count {
        lines.push(format!("**{}:** {c}", t("submitted_count")));
    }

    let status_text = t("failed");
    CardBuilder::new()
        .header(
            &t("network_submission_failed"),
            Some(&status_text),
            Some(ColorTheme::Red),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal)
        .collapsible(&t("error_details"), error_message, false)
        .build()
}

pub fn config_upload_complete(
    config_name: &str,
    file_count: u32,
    labels: Option<&[&str]>,
    desc: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let mut lines = vec![
        format!("**{}:** {}", t("config_name"), config_name),
        format!("**{}:** {}", t("files_uploaded"), file_count),
    ];
    if let Some(d) = desc {
        lines.push(format!("**{}:** {d}", t("description")));
    }

    let status_text = t("success");
    let mut builder = CardBuilder::new()
        .header(
            &t("config_uploaded"),
            Some(&status_text),
            Some(ColorTheme::Green),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if let Some(lbls) = labels {
        if !lbls.is_empty() {
            builder = builder.collapsible(&t("uploaded_files"), &lbls.join(","), false);
        }
    }
    builder.build()
}

pub fn job_submission_start(
    job_title: &str,
    desc: Option<&str>,
    group: Option<&str>,
    prefix: Option<&str>,
    msg: Option<&str>,
    metadata: Option<&serde_json::Value>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let start_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let running_status = format!(
        "<font color='wathet-400'> :StatusInFlight: {}</font>",
        t("running")
    );
    let mut lines = vec![
        format!("**{}:** {}", t("task_name"), job_title),
        format!("**{}:** {}", t("start_time"), start_time),
    ];
    if let Some(d) = desc {
        lines.push(format!("**{}:** {d}", t("task_description")));
    }
    lines.push(format!("**{}:** {}", t("execution_status"), running_status));

    let status_text = t("running");
    let mut builder = CardBuilder::new()
        .header(
            &t("task_submission_started"),
            Some(&status_text),
            Some(ColorTheme::Wathet),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(
                &t("result_storage"),
                Some(group.unwrap_or("*unknown*")),
                "auto",
                1,
            )
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(m) = msg {
        builder = builder.collapsible(&t("running_overview"), m, false);
    }
    if let Some(meta) = metadata {
        let s = serde_json::to_string_pretty(meta).unwrap_or_default();
        builder = builder.collapsible(
            &t("metadata_overview"),
            &format!("```json\n{s}\n```"),
            false,
        );
    }
    builder.build()
}

#[allow(clippy::too_many_arguments)]
pub fn job_submission_complete(
    job_title: &str,
    submitted_count: u32,
    desc: Option<&str>,
    group: Option<&str>,
    prefix: Option<&str>,
    duration: Option<&str>,
    msg: Option<&str>,
    metadata: Option<&serde_json::Value>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let completion_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let status_text = format!(
        "<font color='wathet-400'> :StatusInFlight: {}, {}</font>",
        t("task_submission_complete"),
        t("running")
    );
    let mut lines = vec![
        format!("**{}:** {}", t("task_name"), job_title),
        format!("**{}:** {}", t("completion_time"), completion_time),
        format!("**{}:** {}", t("submitted_count"), submitted_count),
    ];
    if let Some(d) = desc {
        lines.push(format!("**{}:** {d}", t("task_description")));
    }
    if let Some(d) = duration {
        lines.push(format!("**{}:** {d}", t("execution_duration")));
    }
    lines.push(format!("**{}:** {}", t("execution_status"), status_text));

    let header_status_text = t("submitted");
    let mut builder = CardBuilder::new()
        .header(
            &t("task_submission_complete"),
            Some(&header_status_text),
            Some(ColorTheme::Wathet),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(
                &t("result_storage"),
                Some(group.unwrap_or("*unknown*")),
                "auto",
                1,
            )
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(m) = msg {
        builder = builder.collapsible(&t("running_overview"), m, false);
    }
    if let Some(meta) = metadata {
        let s = serde_json::to_string_pretty(meta).unwrap_or_default();
        builder = builder.collapsible(
            &t("metadata_overview"),
            &format!("```json\n{s}\n```"),
            false,
        );
    }
    builder.build()
}

pub fn job_submission_failure(
    job_title: &str,
    error_message: &str,
    submitted_count: Option<u32>,
    group: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let mut lines = vec![
        format!("**{}:** {}", t("task_name"), job_title),
        format!("**{}:** {}", t("group"), group.unwrap_or("*unknown*")),
    ];
    if let Some(c) = submitted_count {
        lines.push(format!("**{}:** {c}", t("submitted_before_failure")));
    }

    let status_text = t("failed");
    CardBuilder::new()
        .header(
            &t("task_submission_failed"),
            Some(&status_text),
            Some(ColorTheme::Red),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal)
        .collapsible(&t("error_details"), error_message, false)
        .build()
}

#[allow(clippy::too_many_arguments)]
pub fn job_complete(
    job_title: &str,
    success: bool,
    status: i32,
    group: Option<&str>,
    prefix: Option<&str>,
    desc: Option<&str>,
    msg: Option<&str>,
    duration: Option<&str>,
    title: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let completion_time = Local::now().format("%Y-%m-%d %H:%M").to_string();

    let (task_status, status_text, color, default_title) = if success {
        (
            format!("<font color='green'> :CheckMark: {}</font>", t("completed")),
            t("success"),
            ColorTheme::Green,
            t("task_completion_notification"),
        )
    } else {
        (
            format!(
                "<font color='red'> :CrossMark: {}: {status}</font>",
                t("failed")
            ),
            t("failure"),
            ColorTheme::Red,
            t("task_failure_notification"),
        )
    };

    let card_title = title.unwrap_or(&default_title);
    let mut lines = vec![
        format!("**{}：** {}", t("task_name"), job_title),
        format!("**{}：** {}", t("completion_time"), completion_time),
    ];
    if let Some(d) = desc {
        lines.push(format!("**{}：** {d}", t("task_description")));
    }
    if let Some(d) = duration {
        lines.push(format!("**{}：** {d}", t("execution_duration")));
    }
    lines.push(format!("**{}：** {}", t("execution_status"), task_status));

    let mut builder = CardBuilder::new()
        .header(card_title, Some(&status_text), Some(color), None)
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(&t("group"), Some(group.unwrap_or("*unknown*")), "auto", 1)
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(m) = msg {
        builder = builder.collapsible(&t("result_overview"), m, false);
    }
    builder.build()
}

/// NOTE: The header is always blue. `overall_status` is used only as the header
/// tag label (not for color detection), allowing callers to pass a descriptive
/// string like "3/5 complete" for display purposes.
pub fn task_set_progress(
    progress: &HashMap<&str, TaskSetProgress>,
    overall_status: &str,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let mut table_lines = vec![
        format!(
            "| {} | Progress | {} | Total |",
            t("task_set_name"),
            t("completed")
        ),
        "|:---|:---|---:|---:|".to_owned(),
    ];
    for (name, p) in progress {
        let pct = if p.total > 0 {
            p.complete as f64 / p.total as f64 * 100.0
        } else {
            0.0
        };
        table_lines.push(format!(
            "| {name} | {pct:.1}% | {} | {} |",
            p.complete, p.total
        ));
    }

    CardBuilder::new()
        .header(
            &t("task_set_progress"),
            Some(overall_status),
            Some(ColorTheme::Blue),
            None,
        )
        .collapsible(&t("task_sets"), &table_lines.join("\n"), true)
        .build()
}

pub fn result_collection_start(
    task_set_names: &[&str],
    job_title: Option<&str>,
    group: Option<&str>,
    msg: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let resolved_title = job_title.map(|s| s.to_owned()).unwrap_or_else(|| {
        if task_set_names.len() == 1 {
            task_set_names[0].to_owned()
        } else {
            format!("Collection of {} Task Sets", task_set_names.len())
        }
    });
    let mut lines = vec![
        format!("**{}：** {}", t("task_name"), resolved_title),
        format!("**{}:** {}", t("task_set_count"), task_set_names.len()),
    ];
    if let Some(g) = group {
        lines.push(format!("**{}:** {g}", t("group")));
    }

    let status_text = t("running");
    let mut builder = CardBuilder::new()
        .header(
            &t("result_collection_started"),
            Some(&status_text),
            Some(ColorTheme::Wathet),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if let Some(m) = msg {
        builder = builder.collapsible(&t("running_overview"), m, false);
    }
    builder.build()
}

pub fn result_collection_complete(
    task_set_names: &[&str],
    job_title: Option<&str>,
    group: Option<&str>,
    prefix: Option<&str>,
    msg: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let resolved_title = job_title.map(|s| s.to_owned()).unwrap_or_else(|| {
        if task_set_names.len() == 1 {
            task_set_names[0].to_owned()
        } else {
            format!("Collection of {} Task Sets", task_set_names.len())
        }
    });
    let mut lines = vec![
        format!("**{}：** {}", t("task_name"), resolved_title),
        format!("**{}:** {}", t("task_set_count"), task_set_names.len()),
    ];
    if let Some(g) = group {
        lines.push(format!("**{}:** {g}", t("group")));
    }

    let status_text = t("success");
    let mut builder = CardBuilder::new()
        .header(
            &t("result_collection_complete"),
            Some(&status_text),
            Some(ColorTheme::Green),
            None,
        )
        .markdown(&lines.join("\n"), TextAlign::Left, TextSize::Normal);

    if group.is_some() || prefix.is_some() {
        builder = builder
            .columns()
            .column(&t("group"), Some(group.unwrap_or("*unknown*")), "auto", 1)
            .column(
                &t("storage_prefix"),
                Some(prefix.unwrap_or("*N/A*")),
                "weighted",
                1,
            )
            .end_columns();
    }
    if let Some(m) = msg {
        builder = builder.collapsible(&t("state_overview"), m, false);
    }
    builder.build()
}

pub fn comparison_complete(
    comparison_name: &str,
    task_set_count: u32,
    result_rows: u32,
    result_columns: u32,
    comparison_table: Option<&str>,
    language: LanguageCode,
) -> GenericCardTemplate {
    let t = |k: &str| get_translation(k, language);
    let metadata_text = format!(
        "**{}:** {}\n**{}:** {}",
        t("comparison_name"),
        comparison_name,
        t("task_sets_compared"),
        task_set_count,
    );

    let status_text = t("success");
    let mut builder = CardBuilder::new()
        .header(
            &t("comparison_complete"),
            Some(&status_text),
            Some(ColorTheme::Orange),
            None,
        )
        .markdown(&metadata_text, TextAlign::Left, TextSize::Normal)
        .columns()
        .column(
            &t("result_rows"),
            Some(&result_rows.to_string()),
            "weighted",
            1,
        )
        .column(
            &t("result_columns"),
            Some(&result_columns.to_string()),
            "weighted",
            1,
        )
        .end_columns();

    if let Some(tbl) = comparison_table {
        builder = builder.collapsible(&t("comparison_results"), tbl, false);
    }
    builder.build()
}

pub fn create_custom_template(language: LanguageCode) -> CardBuilder {
    CardBuilder::new().language(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{LanguageCode, LarkTemplate};
    use std::collections::HashMap;

    fn zh() -> LanguageCode {
        LanguageCode::Zh
    }

    #[test]
    fn test_network_submission_start() {
        let t = network_submission_start("netset1", "TCP", None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "wathet");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("netset1"));
        assert!(el0.contains("TCP"));
    }

    #[test]
    fn test_network_submission_complete_green() {
        let t = network_submission_complete("ns", Some(10), None, None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "green");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("ns"));
    }

    #[test]
    fn test_network_submission_failure_red() {
        let t = network_submission_failure("ns", "timeout", None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "red");
        let elements = card["body"]["elements"].as_array().unwrap();
        assert!(elements.len() >= 2); // markdown + collapsible
    }

    #[test]
    fn test_config_upload_complete() {
        let t = config_upload_complete("cfg1", 5, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "green");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("cfg1"));
    }

    #[test]
    fn test_job_submission_start() {
        let t = job_submission_start("job1", None, None, None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "wathet");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("job1"));
    }

    #[test]
    fn test_job_submission_complete() {
        let t = job_submission_complete("job1", 42, None, None, None, None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "wathet");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("job1"));
        assert!(el0.contains("42"));
    }

    #[test]
    fn test_job_submission_failure_red() {
        let t = job_submission_failure("job1", "oom", None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "red");
        let elements = card["body"]["elements"].as_array().unwrap();
        assert!(elements.len() >= 2);
    }

    #[test]
    fn test_job_complete_success() {
        let t = job_complete("job1", true, 0, None, None, None, None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "green");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("job1"));
    }

    #[test]
    fn test_job_complete_failure() {
        let t = job_complete("job1", false, 1, None, None, None, None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "red");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("1")); // status code in body
    }

    #[test]
    fn test_task_set_progress() {
        let mut progress = HashMap::new();
        progress.insert(
            "set_a",
            TaskSetProgress {
                complete: 5,
                total: 10,
            },
        );
        let t = task_set_progress(&progress, "running", zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "blue");
        let elements = card["body"]["elements"].as_array().unwrap();
        assert!(!elements.is_empty());
    }

    #[test]
    fn test_result_collection_start() {
        let t = result_collection_start(&["set_a", "set_b"], None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "wathet");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("2")); // task_set_count
    }

    #[test]
    fn test_result_collection_complete() {
        let t = result_collection_complete(&["set_a"], Some("my_job"), None, None, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "green");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("my_job"));
    }

    #[test]
    fn test_comparison_complete_orange() {
        let t = comparison_complete("cmp1", 3, 100, 5, None, zh());
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "orange");
        let el0 = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(el0.contains("cmp1"));
    }

    #[test]
    fn test_task_set_progress_fields() {
        let p = TaskSetProgress {
            complete: 3,
            total: 7,
        };
        assert_eq!(p.complete, 3);
        assert_eq!(p.total, 7);
    }

    #[test]
    fn test_create_custom_template() {
        let t = create_custom_template(zh())
            .header("Custom", Some("info"), None, None)
            .markdown("content", TextAlign::Left, TextSize::Normal)
            .build();
        let card = t.generate();
        assert_eq!(card["header"]["title"]["content"], "Custom");
    }
}
