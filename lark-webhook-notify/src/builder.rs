use serde_json::Value;

use crate::blocks::{
    self, Card, CollapsiblePanel, Column, ColumnSet, ColumnWidth, HeaderBlock, Markdown, TextAlign,
    TextSize, TextTag,
};
use crate::templates::{ColorTheme, GenericCardTemplate, LanguageCode};

/// Fluent builder for fully custom Lark interactive cards.
///
/// Produces a [`GenericCardTemplate`] via [`CardBuilder::build`].
///
/// # Example
///
/// ```
/// use lark_webhook_notify::{CardBuilder, ColorTheme, TextAlign, TextSize};
///
/// let card = CardBuilder::new()
///     .header("Deploy Complete", Some("success"), Some(ColorTheme::Green), None)
///     .markdown("**Service:** api v1.4", TextAlign::Left, TextSize::Normal)
///     .columns()
///         .column("Env", Some("prod"), "auto", 1)
///         .column("Region", Some("us-east-1"), "weighted", 1)
///     .end_columns()
///     .collapsible("Logs", "Deploy OK", false)
///     .build();
/// ```
///
/// # Panics
///
/// [`build`](CardBuilder::build) panics if [`columns`](CardBuilder::columns) was
/// called without a matching [`end_columns`](CardBuilder::end_columns).
pub struct CardBuilder {
    /// Language carried for consumers (e.g. `create_custom_template`).
    /// Translations are performed by callers (WorkflowTemplates, etc.) before
    /// passing strings into the builder methods — builder methods accept raw strings.
    language: LanguageCode,
    header: Option<Value>,
    elements: Vec<Value>,
    column_stack: Vec<Vec<Value>>,
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            language: LanguageCode::Zh,
            header: None,
            elements: Vec::new(),
            column_stack: Vec::new(),
        }
    }

    /// Override the language carried by this builder (default: `Zh`).
    pub fn language(mut self, lang: LanguageCode) -> Self {
        self.language = lang;
        self
    }

    /// Set the card header. Status auto-detects color if color is None:
    ///   "running"|"submitted" → Wathet, "success"|"completed" → Green,
    ///   "failed"|"error" → Red, "warning" → Orange, _ → Blue
    pub fn header(
        mut self,
        title: &str,
        status: Option<&str>,
        color: Option<ColorTheme>,
        subtitle: Option<&str>,
    ) -> Self {
        let resolved_color = match color {
            Some(c) => c,
            None => match status.map(|s| s.to_lowercase()).as_deref() {
                Some("running") | Some("submitted") => ColorTheme::Wathet,
                Some("success") | Some("completed") => ColorTheme::Green,
                Some("failed") | Some("error") => ColorTheme::Red,
                Some("warning") => ColorTheme::Orange,
                _ => ColorTheme::Blue,
            },
        };
        let text_tag_list = status.map(|s| {
            vec![Value::from(TextTag {
                text: s.into(),
                color: resolved_color.as_str().into(),
            })]
        });
        self.header = Some(
            HeaderBlock {
                title: title.into(),
                template: resolved_color.as_str().into(),
                subtitle: subtitle.or(Some("")).map(|s| s.into()),
                text_tag_list,
                padding: Some("12px 8px 12px 8px".into()),
            }
            .into(),
        );
        self
    }

    /// Add a single `**Label:** value` metadata line as a markdown block.
    pub fn metadata(mut self, label: &str, value: &str) -> Self {
        self.elements
            .push(blocks::markdown(format!("**{label}:** {value}")).into());
        self
    }

    /// Add multiple `**Label:** value` lines as a single markdown block.
    pub fn metadata_block(mut self, fields: &[(&str, &str)]) -> Self {
        let lines: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("**{k}:** {v}"))
            .collect();
        self.elements
            .push(blocks::markdown(lines.join("\n")).into());
        self
    }

    /// Add a markdown text block with explicit alignment and size.
    pub fn markdown(mut self, content: &str, text_align: TextAlign, text_size: TextSize) -> Self {
        self.elements.push(
            Markdown {
                content: content.into(),
                text_align,
                text_size,
                ..Default::default()
            }
            .into(),
        );
        self
    }

    /// Begin a column group. Must be closed with [`end_columns`](CardBuilder::end_columns).
    pub fn columns(mut self) -> Self {
        self.column_stack.push(Vec::new());
        self
    }

    /// Add a column to the current column group.
    ///
    /// `width`: `"auto"` for content-sized or `"weighted"` for proportional.
    /// `weight` is only used when `width = "weighted"`.
    pub fn column(mut self, label: &str, value: Option<&str>, width: &str, weight: u32) -> Self {
        let margin = if width == "auto" {
            "0px 4px 0px 4px"
        } else {
            "0px 0px 0px 0px"
        };
        let col_content: Value = Markdown {
            content: match value {
                Some(v) => format!("**{label}**\n{v}"),
                None => label.to_string(),
            },
            text_align: TextAlign::Center,
            text_size: TextSize::NormalV2,
            margin: margin.into(),
        }
        .into();
        let col_block: Value = Column {
            elements: vec![col_content],
            width: if width == "weighted" {
                ColumnWidth::Weighted
            } else {
                ColumnWidth::Auto
            },
            weight: if width == "weighted" {
                Some(weight)
            } else {
                None
            },
            ..Default::default()
        }
        .into();
        self.column_stack
            .last_mut()
            .expect("call .columns() first")
            .push(col_block);
        self
    }

    /// Close the current column group and add a `column_set` block to the card body.
    pub fn end_columns(mut self) -> Self {
        let cols = self.column_stack.pop().expect("no open column context");
        self.elements.push(
            ColumnSet {
                columns: cols,
                ..Default::default()
            }
            .into(),
        );
        self
    }

    /// Add a collapsible panel with a markdown body.
    ///
    /// `expanded = true` renders the panel open by default.
    pub fn collapsible(mut self, title: &str, content: &str, expanded: bool) -> Self {
        self.elements.push(
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
                expanded,
                ..Default::default()
            }
            .into(),
        );
        self
    }

    /// Add a horizontal divider (`---`).
    pub fn divider(mut self) -> Self {
        self.elements.push(blocks::markdown("---").into());
        self
    }

    /// Add any block that implements `Into<serde_json::Value>` directly.
    ///
    /// Use this to insert raw block structs from the [`blocks`](crate::blocks) module
    /// or hand-crafted JSON values.
    pub fn add_block(mut self, block: impl Into<Value>) -> Self {
        self.elements.push(block.into());
        self
    }

    /// Finalize and return the card as a [`GenericCardTemplate`].
    ///
    /// # Panics
    ///
    /// Panics if any column group opened with [`columns`](CardBuilder::columns)
    /// was not closed with [`end_columns`](CardBuilder::end_columns).
    pub fn build(self) -> GenericCardTemplate {
        assert!(
            self.column_stack.is_empty(),
            "unclosed column context: call .end_columns()"
        );
        let card: Value = Card {
            elements: self.elements,
            header: self.header.unwrap_or(Value::Null),
            config: Some(blocks::config_textsize_normal_v2()),
            ..Default::default()
        }
        .into();
        GenericCardTemplate { content: card }
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::LarkTemplate;

    #[test]
    fn test_basic_card() {
        let t = CardBuilder::new()
            .header("My Title", Some("running"), None, None)
            .markdown("Hello world", TextAlign::Left, TextSize::Normal)
            .build();
        let card = t.generate();
        assert_eq!(card["schema"], "2.0");
        assert_eq!(card["header"]["title"]["content"], "My Title");
        assert_eq!(card["header"]["template"], "wathet"); // auto-detected from "running"
        assert_eq!(card["body"]["elements"][0]["content"], "Hello world");
    }

    #[test]
    fn test_explicit_color_overrides_auto() {
        let t = CardBuilder::new()
            .header("T", Some("running"), Some(ColorTheme::Red), None)
            .build();
        let card = t.generate();
        assert_eq!(card["header"]["template"], "red");
    }

    #[test]
    fn test_metadata() {
        let t = CardBuilder::new()
            .header("H", None, None, None)
            .metadata("Key", "Value")
            .build();
        let card = t.generate();
        let content = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(content.contains("**Key:**"));
        assert!(content.contains("Value"));
    }

    #[test]
    fn test_metadata_block() {
        let t = CardBuilder::new()
            .header("H", None, None, None)
            .metadata_block(&[("Name", "foo"), ("Age", "42")])
            .build();
        let card = t.generate();
        let content = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert!(content.contains("**Name:**"));
        assert!(content.contains("**Age:**"));
    }

    #[test]
    fn test_columns() {
        let t = CardBuilder::new()
            .header("H", None, None, None)
            .columns()
            .column("Group", Some("grp1"), "auto", 1)
            .column("Prefix", Some("p/"), "weighted", 1)
            .end_columns()
            .build();
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0]["tag"], "column_set");
    }

    #[test]
    #[should_panic]
    fn test_build_panics_with_open_columns() {
        let _ = CardBuilder::new()
            .header("H", None, None, None)
            .columns()
            // forgot .end_columns()
            .build();
    }

    #[test]
    fn test_collapsible() {
        let t = CardBuilder::new()
            .header("H", None, None, None)
            .collapsible("Details", "some content", false)
            .build();
        let card = t.generate();
        let elements = card["body"]["elements"].as_array().unwrap();
        assert_eq!(elements[0]["tag"], "collapsible_panel");
    }

    #[test]
    fn test_divider() {
        let t = CardBuilder::new()
            .header("H", None, None, None)
            .divider()
            .build();
        let card = t.generate();
        let content = card["body"]["elements"][0]["content"].as_str().unwrap();
        assert_eq!(content, "---");
    }

    #[test]
    fn test_has_config() {
        let t = CardBuilder::new().header("H", None, None, None).build();
        let card = t.generate();
        assert!(card.get("config").is_some());
    }
}
