use chrono::Local;
use serde_json::Value;

use crate::blocks;
use crate::blocks::{
    Card, CollapsiblePanel, Column, ColumnSet, ColumnWidth, HeaderBlock, Markdown, TextAlign,
    TextSize, TextTag,
};

/// A fully-rendered Lark card as a `serde_json::Value`.
///
/// Produced by [`LarkTemplate::generate`] and consumed by
/// [`LarkWebhookNotifier::send_raw_content`].
pub type CardContent = serde_json::Value;

/// Language for translated field labels in card bodies.
///
/// User-supplied strings (task names, descriptions, error messages) are always
/// passed through unchanged. Only built-in label keys (e.g. "Job Name",
/// "Start Time") are translated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageCode {
    /// Simplified Chinese (`zh`)
    Zh,
    /// English (`en`)
    En,
}

/// Severity of an alert notification, controlling header color and icon.
///
/// | Level | Color | Icon |
/// |-------|-------|------|
/// | `Info` | Blue | `:InfoCircle:` |
/// | `Warning` | Orange | `:WarningTriangle:` |
/// | `Error` | Red | `:CrossMark:` |
/// | `Critical` | Red | `:Fire:` |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Card header background color / status tag color.
///
/// Maps directly to the Lark card API `"template"` field values.
/// [`CardBuilder::header`] can auto-detect the color from a status string —
/// supply `None` for `color` to use that behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    /// `"blue"` — informational / neutral
    Blue,
    /// `"green"` — success / completed
    Green,
    /// `"red"` — failure / error
    Red,
    /// `"orange"` — warning / caution
    Orange,
    /// `"wathet"` — in-progress / running
    Wathet,
    /// `"purple"`
    Purple,
    /// `"grey"`
    Grey,
}

impl ColorTheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorTheme::Blue => "blue",
            ColorTheme::Green => "green",
            ColorTheme::Red => "red",
            ColorTheme::Orange => "orange",
            ColorTheme::Wathet => "wathet",
            ColorTheme::Purple => "purple",
            ColorTheme::Grey => "grey",
        }
    }
}

impl std::fmt::Display for ColorTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ColorTheme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blue" => Ok(ColorTheme::Blue),
            "green" => Ok(ColorTheme::Green),
            "red" => Ok(ColorTheme::Red),
            "orange" => Ok(ColorTheme::Orange),
            "wathet" => Ok(ColorTheme::Wathet),
            "purple" => Ok(ColorTheme::Purple),
            "grey" | "gray" => Ok(ColorTheme::Grey),
            _ => Err(format!("unknown color: {s}")),
        }
    }
}

impl SeverityLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeverityLevel::Info => "info",
            SeverityLevel::Warning => "warning",
            SeverityLevel::Error => "error",
            SeverityLevel::Critical => "critical",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            SeverityLevel::Info => "blue",
            SeverityLevel::Warning => "orange",
            SeverityLevel::Error | SeverityLevel::Critical => "red",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SeverityLevel::Info => ":InfoCircle:",
            SeverityLevel::Warning => ":WarningTriangle:",
            SeverityLevel::Error => ":CrossMark:",
            SeverityLevel::Critical => ":Fire:",
        }
    }
}

impl std::fmt::Display for SeverityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for SeverityLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(SeverityLevel::Info),
            "warning" => Ok(SeverityLevel::Warning),
            "error" => Ok(SeverityLevel::Error),
            "critical" => Ok(SeverityLevel::Critical),
            _ => Err(format!("unknown severity: {s}")),
        }
    }
}

impl std::fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageCode::Zh => write!(f, "zh"),
            LanguageCode::En => write!(f, "en"),
        }
    }
}

impl std::str::FromStr for LanguageCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zh" => Ok(LanguageCode::Zh),
            "en" => Ok(LanguageCode::En),
            _ => Err(format!("unknown language: {s}")),
        }
    }
}

pub fn get_translation(key: &str, lang: LanguageCode) -> String {
    let s = match lang {
        LanguageCode::Zh => zh_translation(key),
        LanguageCode::En => en_translation(key),
    };
    s.map(|s| s.to_owned()).unwrap_or_else(|| key.to_owned())
}

fn zh_translation(key: &str) -> Option<&'static str> {
    Some(match key {
        "task_name" => "作业名称",
        "start_time" => "开始时间",
        "completion_time" => "完成时间",
        "task_description" => "作业描述",
        "estimated_duration" => "预计用时",
        "execution_duration" => "执行时长",
        "execution_status" => "执行状态",
        "result_storage" => "结果存储",
        "storage_prefix" => "存储前缀",
        "result_overview" => "结果概览",
        "running_overview" => "运行概览",
        "state_overview" => "状态概览",
        "metadata_overview" => "元数据概览",
        "running" => "正在运行",
        "completed" => "已成功完成",
        "failed" => "失败",
        "success" => "已完成",
        "failure" => "失败",
        "task_notification" => "作业运行情况通知",
        "task_completion_notification" => "作业完成情况通知",
        "task_failure_notification" => "作业失败情况通知",
        "no_description" => "*No description provided*",
        "timestamp" => "时间",
        "unknown_task" => "未知任务",
        "return_code" => "返回值",
        "group" => "归属组别",
        "network_submission_started" => "网络提交已开始",
        "network_submission_complete" => "网络提交已完成",
        "network_submission_failed" => "网络提交失败",
        "network_set_name" => "网络集名称",
        "network_type" => "网络类型",
        "expected_count" => "预期数量",
        "submitted_count" => "提交总数",
        "submitted" => "已提交",
        "config_uploaded" => "配置已上传",
        "config_name" => "配置名称",
        "files_uploaded" => "已上传文件数",
        "description" => "配置描述",
        "uploaded_files" => "已上传文件的标签",
        "task_submission_started" => "任务提交已开始",
        "task_submission_complete" => "任务提交已完成",
        "task_submission_failed" => "任务提交失败",
        "task_set_name" => "任务集名称",
        "iterations" => "迭代次数",
        "duration" => "持续时间",
        "submission_overview" => "提交概览",
        "status" => "状态",
        "successfully_completed" => "已成功完成",
        "submitted_before_failure" => "失败前已提交",
        "error_details" => "错误详情",
        "task_set_complete" => "任务集已完成",
        "task_set_failed" => "任务集失败",
        "task_set_progress" => "任务集进度",
        "task_set_count" => "任务集数量",
        "result_collection_started" => "结果收集已开始",
        "result_collection_complete" => "结果收集已完成",
        "task_sets" => "任务集",
        "rows" => "行数",
        "columns" => "列数",
        "comparison_complete" => "比较已完成",
        "comparison_name" => "比较名称",
        "task_sets_compared" => "已比较任务集",
        "common_networks" => "公共网络数",
        "result_rows" => "结果行数",
        "result_columns" => "结果列数",
        "comparison_results" => "比较结果",
        "total_items" => "总项目数",
        "summary" => "摘要",
        "items" => "项目",
        _ => return None,
    })
}

fn en_translation(key: &str) -> Option<&'static str> {
    Some(match key {
        "task_name" => "Job Name",
        "start_time" => "Start Time",
        "completion_time" => "Completion Time",
        "task_description" => "Job Description",
        "estimated_duration" => "Estimated Duration",
        "execution_duration" => "Execution Duration",
        "execution_status" => "Execution Status",
        "result_storage" => "Result Storage",
        "storage_prefix" => "Storage Prefix",
        "result_overview" => "Result Overview",
        "running_overview" => "Running Overview",
        "state_overview" => "State Overview",
        "metadata_overview" => "Metadata Overview",
        "running" => "Running Now",
        "completed" => "Successfully Completed",
        "failed" => "Failed",
        "success" => "Completed",
        "failure" => "Failed",
        "task_notification" => "Job Status Notification",
        "task_completion_notification" => "Job Completion Notification",
        "task_failure_notification" => "Job Failure Notification",
        "no_description" => "*No description provided*",
        "timestamp" => "Timestamp",
        "unknown_task" => "Unknown Task",
        "return_code" => "Return Status",
        "group" => "Group",
        "network_submission_started" => "Network Submission Started",
        "network_submission_complete" => "Network Submission Complete",
        "network_submission_failed" => "Network Submission Failed",
        "network_set_name" => "Network Set Name",
        "network_type" => "Network Type",
        "expected_count" => "Expected Count",
        "submitted_count" => "Total Count Submitted",
        "submitted" => "Submitted",
        "config_uploaded" => "Configuration Uploaded",
        "config_name" => "Config Name",
        "files_uploaded" => "Files Uploaded",
        "description" => "Config Description",
        "uploaded_files" => "Labels of Uploaded Files",
        "task_submission_started" => "Task Submission Started",
        "task_submission_complete" => "Task Submission Complete",
        "task_submission_failed" => "Task Submission Failed",
        "task_set_name" => "Task Set Name",
        "iterations" => "Iterations",
        "duration" => "Duration",
        "submission_overview" => "Submission Overview",
        "status" => "Status",
        "successfully_completed" => "Successfully Completed",
        "submitted_before_failure" => "Submitted Before Failure",
        "error_details" => "Error Details",
        "task_set_complete" => "Task Set Complete",
        "task_set_failed" => "Task Set Failed",
        "task_set_progress" => "Task Set Progress",
        "task_set_count" => "Task Set Count",
        "result_collection_started" => "Result Collection Started",
        "result_collection_complete" => "Result Collection Complete",
        "task_sets" => "Task Sets",
        "rows" => "Rows",
        "columns" => "Columns",
        "comparison_complete" => "Comparison Complete",
        "comparison_name" => "Comparison Name",
        "task_sets_compared" => "Task Sets Compared",
        "common_networks" => "Common Networks",
        "result_rows" => "Result Rows",
        "result_columns" => "Result Columns",
        "comparison_results" => "Comparison Results",
        "total_items" => "Total Items",
        "summary" => "Summary",
        "items" => "Items",
        _ => return None,
    })
}

/// A type that can render itself into a Lark card JSON payload.
///
/// Implement this trait to define custom templates that work with
/// [`LarkWebhookNotifier::send_template`].
///
/// All pre-built template structs and the output of [`CardBuilder::build`]
/// implement this trait.
pub trait LarkTemplate {
    /// Render this template into a [`CardContent`] (a `serde_json::Value`).
    fn generate(&self) -> CardContent;
}

/// Legacy task notification using a fixed Lark template reference ID.
///
/// Sends a card built from a Lark-hosted template (`template_id = "AAqz08XD5HCzP"`)
/// with variables injected at render time. Use [`StartTaskTemplate`] or
/// [`ReportTaskResultTemplate`] for fully custom cards instead.
pub struct LegacyTaskTemplate {
    pub task_name: String,
    pub status: Option<i32>,
    pub group: String,
    pub prefix: String,
    pub task_summary: String,
    pub language: LanguageCode,
}

impl LegacyTaskTemplate {
    fn t(&self, key: &str) -> String {
        get_translation(key, self.language)
    }
}

impl LarkTemplate for LegacyTaskTemplate {
    fn generate(&self) -> CardContent {
        let task_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let task_status = match self.status {
            Some(s) if s != 0 => format!(
                "<font color='red'> :CrossMark: {}: {} {s}</font>",
                self.t("failed"),
                self.t("return_code")
            ),
            _ => format!(
                "<font color='green'> :CheckMark: {}</font>",
                self.t("completed")
            ),
        };
        blocks::template_reference(
            "AAqz08XD5HCzP",
            "1.0.3",
            serde_json::json!({
                "task_name": self.task_name,
                "task_time": task_time,
                "attachment_group": self.group,
                "attachment_prefix": self.prefix,
                "task_summary": self.task_summary,
                "task_status": task_status,
            }),
        )
        .into()
    }
}

/// Card template for notifying that a job or task has started.
///
/// Generates a wathet (light blue) header card with the task name, start time,
/// optional description/duration, running status, and optionally a storage
/// column pair and collapsible running-overview panel.
pub struct StartTaskTemplate {
    pub task_name: String,
    pub desc: Option<String>,
    pub group: Option<String>,
    pub prefix: Option<String>,
    pub msg: Option<String>,
    pub estimated_duration: Option<String>,
    pub language: LanguageCode,
}

impl StartTaskTemplate {
    fn t(&self, key: &str) -> String {
        get_translation(key, self.language)
    }
}

impl LarkTemplate for StartTaskTemplate {
    fn generate(&self) -> CardContent {
        let task_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let no_desc = get_translation("no_description", self.language);
        let desc = self.desc.as_deref().unwrap_or(&no_desc);
        let duration_text = self
            .estimated_duration
            .as_ref()
            .map(|d| format!("\n**{}:** {d}", self.t("estimated_duration")))
            .unwrap_or_default();
        let task_status = format!(
            "<font color='wathet-400'> :StatusInFlight: {}</font>",
            self.t("running")
        );
        let main_text = format!(
            "**{}:** {}\n**{}:** {}\n**{}:** {}{}\n**{}:** {}",
            self.t("task_name"),
            self.task_name,
            self.t("start_time"),
            task_time,
            self.t("task_description"),
            desc,
            duration_text,
            self.t("execution_status"),
            task_status,
        );

        let mut elements = vec![blocks::markdown(&main_text).into()];

        if self.group.is_some() || self.prefix.is_some() {
            elements.push(storage_columns(
                &self.t("result_storage"),
                self.group.as_deref().unwrap_or(""),
                &self.t("storage_prefix"),
                self.prefix.as_deref().unwrap_or(""),
            ));
        }

        if let Some(msg) = &self.msg {
            elements.push(overview_panel(&self.t("running_overview"), msg));
        }

        let hdr: Value = HeaderBlock {
            title: self.t("task_notification"),
            template: "wathet".into(),
            subtitle: Some("".into()),
            text_tag_list: Some(vec![
                TextTag {
                    text: self.t("running"),
                    color: "wathet".into(),
                }
                .into(),
            ]),
            padding: Some("12px 8px 12px 8px".into()),
        }
        .into();
        Card {
            elements,
            header: hdr,
            config: Some(blocks::config_textsize_normal_v2()),
            ..Default::default()
        }
        .into()
    }
}

/// Card template for reporting that a job completed successfully (green header).
///
/// Shows task name, completion time, optional description, duration, and
/// optionally a storage column pair and result-overview panel.
/// For failure notifications use [`ReportFailureTaskTemplate`].
pub struct ReportTaskResultTemplate {
    pub task_name: String,
    pub status: i32,
    pub group: Option<String>,
    pub prefix: Option<String>,
    pub desc: Option<String>,
    pub msg: Option<String>,
    pub duration: Option<String>,
    pub title: Option<String>,
    pub language: LanguageCode,
}

impl ReportTaskResultTemplate {
    fn t(&self, key: &str) -> String {
        get_translation(key, self.language)
    }
}

impl LarkTemplate for ReportTaskResultTemplate {
    fn generate(&self) -> CardContent {
        let task_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let task_status = format!(
            "<font color='green'> :CheckMark: {}</font>",
            self.t("completed")
        );
        let desc_text = self
            .desc
            .as_ref()
            .map(|d| format!("\n**{}：** {d}", self.t("task_description")))
            .unwrap_or_default();
        let dur_text = self
            .duration
            .as_ref()
            .map(|d| format!("\n**{}：** {d}", self.t("execution_duration")))
            .unwrap_or_default();
        let main_text = format!(
            "**{}：** {}\n**{}：** {}{}{}\n**{}：** {}",
            self.t("task_name"),
            self.task_name,
            self.t("completion_time"),
            task_time,
            desc_text,
            dur_text,
            self.t("execution_status"),
            task_status,
        );

        let mut elements = vec![blocks::markdown(&main_text).into()];
        if self.group.is_some() || self.prefix.is_some() {
            elements.push(storage_columns(
                &self.t("group"),
                self.group.as_deref().unwrap_or(""),
                &self.t("storage_prefix"),
                self.prefix.as_deref().unwrap_or(""),
            ));
        }
        if let Some(msg) = &self.msg {
            elements.push(overview_panel(&self.t("result_overview"), msg));
        }

        let default_title = self.t("task_completion_notification");
        let card_title = self.title.as_deref().unwrap_or(&default_title);
        let hdr: Value = HeaderBlock {
            title: card_title.into(),
            template: "green".into(),
            subtitle: Some("".into()),
            text_tag_list: Some(vec![
                TextTag {
                    text: self.t("success"),
                    color: "green".into(),
                }
                .into(),
            ]),
            padding: Some("12px 8px 12px 8px".into()),
        }
        .into();
        Card {
            elements,
            header: hdr,
            config: Some(blocks::config_textsize_normal_v2()),
            ..Default::default()
        }
        .into()
    }
}

/// Card template for reporting a job failure (red header).
///
/// Shows task name, completion time, non-zero exit `status`, optional
/// description/duration, and optionally a storage column pair and
/// result-overview panel.
pub struct ReportFailureTaskTemplate {
    pub task_name: String,
    /// Non-zero process exit code. Displayed in the status line.
    pub status: i32,
    pub group: Option<String>,
    pub prefix: Option<String>,
    pub desc: Option<String>,
    pub msg: Option<String>,
    pub duration: Option<String>,
    pub title: Option<String>,
    pub language: LanguageCode,
}

impl ReportFailureTaskTemplate {
    fn t(&self, key: &str) -> String {
        get_translation(key, self.language)
    }
}

impl LarkTemplate for ReportFailureTaskTemplate {
    fn generate(&self) -> CardContent {
        let task_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let status = self.status;
        let task_status = format!(
            "<font color='red'> :CrossMark: {}: {status}</font>",
            self.t("failed")
        );
        let desc_text = self
            .desc
            .as_ref()
            .map(|d| format!("\n**{}：** {d}", self.t("task_description")))
            .unwrap_or_default();
        let dur_text = self
            .duration
            .as_ref()
            .map(|d| format!("\n**{}：** {d}", self.t("execution_duration")))
            .unwrap_or_default();
        let main_text = format!(
            "**{}：** {}\n**{}：** {}{}{}\n**{}：** {}",
            self.t("task_name"),
            self.task_name,
            self.t("completion_time"),
            task_time,
            desc_text,
            dur_text,
            self.t("execution_status"),
            task_status,
        );

        let mut elements = vec![blocks::markdown(&main_text).into()];
        if self.group.is_some() || self.prefix.is_some() {
            elements.push(storage_columns(
                &self.t("group"),
                self.group.as_deref().unwrap_or(""),
                &self.t("storage_prefix"),
                self.prefix.as_deref().unwrap_or(""),
            ));
        }
        if let Some(msg) = &self.msg {
            elements.push(overview_panel(&self.t("result_overview"), msg));
        }

        let default_title = self.t("task_failure_notification");
        let card_title = self.title.as_deref().unwrap_or(&default_title);
        let hdr: Value = HeaderBlock {
            title: card_title.into(),
            template: "red".into(),
            subtitle: Some("".into()),
            text_tag_list: Some(vec![
                TextTag {
                    text: self.t("failure"),
                    color: "red".into(),
                }
                .into(),
            ]),
            padding: Some("12px 8px 12px 8px".into()),
        }
        .into();
        Card {
            elements,
            header: hdr,
            config: Some(blocks::config_textsize_normal_v2()),
            ..Default::default()
        }
        .into()
    }
}

/// A simple one-block card with a title, a color, and a markdown body.
pub struct SimpleMessageTemplate {
    pub title: String,
    pub content: String,
    pub color: ColorTheme,
    pub language: LanguageCode,
}

impl LarkTemplate for SimpleMessageTemplate {
    fn generate(&self) -> CardContent {
        let hdr: Value = HeaderBlock {
            title: self.title.clone(),
            template: self.color.as_str().into(),
            ..Default::default()
        }
        .into();
        Card {
            elements: vec![blocks::markdown(&self.content).into()],
            header: hdr,
            ..Default::default()
        }
        .into()
    }
}

/// Alert notification card. Header color and icon are driven by [`SeverityLevel`].
///
/// Use the [`send_alert`](crate::send_alert) convenience function to send one
/// without instantiating this struct directly.
pub struct AlertTemplate {
    pub title: String,
    pub message: String,
    pub severity: SeverityLevel,
    /// Timestamp string to display in the card body (caller-formatted).
    pub timestamp: String,
    pub language: LanguageCode,
}

impl AlertTemplate {
    fn t(&self, key: &str) -> String {
        get_translation(key, self.language)
    }
}

impl LarkTemplate for AlertTemplate {
    fn generate(&self) -> CardContent {
        let color = self.severity.color();
        let icon = self.severity.icon();
        let severity_str = self.severity.as_str().to_uppercase();
        let body_text = format!(
            "{icon} **{}**\n\n**{}：** {}",
            self.message,
            self.t("timestamp"),
            self.timestamp,
        );
        let hdr: Value = HeaderBlock {
            title: self.title.clone(),
            template: color.into(),
            subtitle: Some(severity_str.clone()),
            text_tag_list: Some(vec![
                TextTag {
                    text: severity_str,
                    color: color.into(),
                }
                .into(),
            ]),
            ..Default::default()
        }
        .into();
        Card {
            elements: vec![blocks::markdown(&body_text).into()],
            header: hdr,
            ..Default::default()
        }
        .into()
    }
}

/// Pass-through template that wraps a pre-built [`CardContent`] value.
///
/// Useful when you have a `serde_json::Value` from an external source and want
/// to send it through [`LarkWebhookNotifier::send_template`].
pub struct RawContentTemplate {
    pub content: CardContent,
    pub language: LanguageCode,
}

impl LarkTemplate for RawContentTemplate {
    fn generate(&self) -> CardContent {
        self.content.clone()
    }
}

/// Output type of [`CardBuilder::build`] and all workflow functions.
///
/// Implements [`LarkTemplate`]; pass directly to [`LarkWebhookNotifier::send_template`].
pub struct GenericCardTemplate {
    pub(crate) content: CardContent,
}

impl LarkTemplate for GenericCardTemplate {
    fn generate(&self) -> CardContent {
        self.content.clone()
    }
}

pub(crate) fn storage_columns(
    group_label: &str,
    group_value: &str,
    prefix_label: &str,
    prefix_value: &str,
) -> Value {
    let col1: Value = Column {
        elements: vec![
            Markdown {
                content: format!("**{group_label}**\n{group_value}"),
                text_align: TextAlign::Center,
                text_size: TextSize::NormalV2,
                margin: "0px 4px 0px 4px".into(),
            }
            .into(),
        ],
        width: ColumnWidth::Auto,
        ..Default::default()
    }
    .into();
    let col2: Value = Column {
        elements: vec![
            Markdown {
                content: format!("**{prefix_label}**\n{prefix_value}"),
                text_align: TextAlign::Center,
                text_size: TextSize::NormalV2,
                ..Default::default()
            }
            .into(),
        ],
        width: ColumnWidth::Weighted,
        weight: Some(1),
        ..Default::default()
    }
    .into();
    ColumnSet {
        columns: vec![col1, col2],
        ..Default::default()
    }
    .into()
}

pub(crate) fn overview_panel(title: &str, content: &str) -> Value {
    CollapsiblePanel {
        title_markdown: format!("**<font color='grey-800'>{title}</font>**"),
        elements: vec![
            Markdown {
                content: content.into(),
                text_size: TextSize::NormalV2,
                ..Default::default()
            }
            .into(),
        ],
        expanded: false,
        ..Default::default()
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_zh() {
        assert_eq!(get_translation("task_name", LanguageCode::Zh), "作业名称");
        assert_eq!(get_translation("running", LanguageCode::Zh), "正在运行");
    }

    #[test]
    fn test_translation_en() {
        assert_eq!(get_translation("task_name", LanguageCode::En), "Job Name");
        assert_eq!(get_translation("running", LanguageCode::En), "Running Now");
    }

    #[test]
    fn test_translation_fallback() {
        let result = get_translation("nonexistent_key_xyz", LanguageCode::Zh);
        assert_eq!(result, "nonexistent_key_xyz");
    }

    #[test]
    fn test_simple_message_generate() {
        let t = SimpleMessageTemplate {
            title: "Hello".to_owned(),
            content: "World".to_owned(),
            color: ColorTheme::Blue,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["title"]["content"], "Hello");
        assert_eq!(card["header"]["template"], "blue");
        assert_eq!(card["body"]["elements"][0]["content"], "World");
        assert!(card.get("config").is_none());
    }

    #[test]
    fn test_alert_template_warning() {
        let t = AlertTemplate {
            title: "Alert!".to_owned(),
            message: "High CPU".to_owned(),
            severity: SeverityLevel::Warning,
            timestamp: "2026-01-01 00:00:00".to_owned(),
            language: LanguageCode::En,
        };
        let card = t.generate();
        assert_eq!(card["header"]["template"], "orange");
        let body_content = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(body_content.contains("High CPU"));
    }

    #[test]
    fn test_raw_content_passthrough() {
        let content = serde_json::json!({"schema": "2.0", "custom": true});
        let t = RawContentTemplate {
            content: content.clone(),
            language: LanguageCode::Zh,
        };
        assert_eq!(t.generate(), content);
    }

    #[test]
    fn test_legacy_task_template() {
        let t = LegacyTaskTemplate {
            task_name: "my-task".to_owned(),
            status: Some(0),
            group: "g1".to_owned(),
            prefix: "p/".to_owned(),
            task_summary: "summary".to_owned(),
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        assert_eq!(card["type"], "template");
        assert_eq!(card["data"]["template_id"], "AAqz08XD5HCzP");
        assert_eq!(card["data"]["template_variable"]["task_name"], "my-task");
        assert!(
            card["data"]["template_variable"]["task_status"]
                .as_str()
                .unwrap()
                .contains("CheckMark")
        );
    }

    #[test]
    fn test_start_task_template_structure() {
        let t = StartTaskTemplate {
            task_name: "build".to_owned(),
            desc: None,
            group: None,
            prefix: None,
            msg: None,
            estimated_duration: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["template"], "wathet");
        assert!(card.get("config").is_some());
    }

    #[test]
    fn test_report_task_result_green() {
        let t = ReportTaskResultTemplate {
            task_name: "build".to_owned(),
            status: 0,
            group: None,
            prefix: None,
            desc: None,
            msg: None,
            duration: None,
            title: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        assert_eq!(card["header"]["template"], "green");
    }

    #[test]
    fn test_report_failure_task_red() {
        let t = ReportFailureTaskTemplate {
            task_name: "build".to_owned(),
            status: 1,
            group: None,
            prefix: None,
            desc: None,
            msg: None,
            duration: None,
            title: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        assert_eq!(card["header"]["template"], "red");
    }

    #[test]
    fn test_report_failure_body_contains_status_code() {
        let t = ReportFailureTaskTemplate {
            task_name: "myjob".to_owned(),
            status: 42,
            group: None,
            prefix: None,
            desc: None,
            msg: None,
            duration: None,
            title: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        let body_text = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(
            body_text.contains("42"),
            "body should contain the return code"
        );
        assert!(body_text.contains("myjob"), "body should contain task name");
    }

    #[test]
    fn test_start_task_template_body_element_count_no_group() {
        let t = StartTaskTemplate {
            task_name: "t".to_owned(),
            desc: None,
            group: None,
            prefix: None,
            msg: None,
            estimated_duration: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(
            elements.len(),
            1,
            "only markdown block when no group/prefix/msg"
        );
    }

    #[test]
    fn test_start_task_template_with_group_prefix_has_column_set() {
        let t = StartTaskTemplate {
            task_name: "t".to_owned(),
            desc: None,
            group: Some("grp".to_owned()),
            prefix: Some("pfx/".to_owned()),
            msg: None,
            estimated_duration: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(
            elements.len(),
            2,
            "markdown + column_set when group/prefix set"
        );
        assert_eq!(elements[1]["tag"], "column_set");
    }

    #[test]
    fn test_start_task_template_with_group_prefix_and_msg() {
        let t = StartTaskTemplate {
            task_name: "t".to_owned(),
            desc: None,
            group: Some("grp".to_owned()),
            prefix: Some("pfx/".to_owned()),
            msg: Some("running details".to_owned()),
            estimated_duration: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(elements.len(), 3, "markdown + column_set + collapsible");
        assert_eq!(elements[2]["tag"], "collapsible_panel");
    }

    #[test]
    fn test_language_switching_body_text() {
        let zh_t = SimpleMessageTemplate {
            title: "T".to_owned(),
            content: "C".to_owned(),
            color: ColorTheme::Blue,
            language: LanguageCode::Zh,
        };
        let en_t = SimpleMessageTemplate {
            title: "T".to_owned(),
            content: "C".to_owned(),
            color: ColorTheme::Blue,
            language: LanguageCode::En,
        };
        // body content is the same (caller-supplied), but header title translations differ
        let zh_card = zh_t.generate();
        let en_card = en_t.generate();
        assert_eq!(zh_card["header"]["title"]["content"], "T");
        assert_eq!(en_card["header"]["title"]["content"], "T");
        // schema and structure identical
        assert_eq!(zh_card["schema"], en_card["schema"]);
    }

    #[test]
    fn test_start_task_language_switching_zh_en() {
        let make = |lang| StartTaskTemplate {
            task_name: "job".to_owned(),
            desc: None,
            group: None,
            prefix: None,
            msg: None,
            estimated_duration: None,
            language: lang,
        };
        let zh_body = make(LanguageCode::Zh).generate();
        let en_body = make(LanguageCode::En).generate();
        let zh_text = zh_body["body"]["elements"][0]["content"].as_str().unwrap();
        let en_text = en_body["body"]["elements"][0]["content"].as_str().unwrap();
        // zh and en produce different label text in the body
        assert_ne!(zh_text, en_text, "zh and en body text should differ");
        assert!(zh_text.contains("job"), "both should contain the task name");
        assert!(en_text.contains("job"));
    }

    #[test]
    fn test_report_task_result_body_contains_task_name() {
        let t = ReportTaskResultTemplate {
            task_name: "myjob".to_owned(),
            status: 0,
            group: Some("g".to_owned()),
            prefix: Some("p/".to_owned()),
            desc: None,
            msg: None,
            duration: None,
            title: None,
            language: LanguageCode::Zh,
        };
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(
            elements.len(),
            2,
            "markdown + column_set when group/prefix set"
        );
        let body_text = elements[0]["content"].as_str().unwrap();
        assert!(body_text.contains("myjob"));
        assert_eq!(elements[1]["tag"], "column_set");
    }

    #[test]
    fn test_generic_card_template_passthrough() {
        use crate::builder::CardBuilder;
        let card_val = CardBuilder::new()
            .header("My Card", None, None, None)
            .markdown(
                "hello",
                crate::blocks::TextAlign::Left,
                crate::blocks::TextSize::Normal,
            )
            .build()
            .generate();
        assert_eq!(card_val["schema"], "2.0");
        assert_eq!(card_val["header"]["title"]["content"], "My Card");
        assert_eq!(card_val["body"]["elements"][0]["content"], "hello");
    }
}
