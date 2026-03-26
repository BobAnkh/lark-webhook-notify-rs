//! Low-level block types and layout enums for assembling Lark card JSON.
//!
//! All block structs implement [`Default`] and `From<T> for serde_json::Value`.
//! The smart constructor functions (e.g. [`markdown`], [`column`]) fill required
//! fields and leave everything else at its default:
//!
//! ```
//! use lark_webhook_notify::blocks::{Markdown, TextAlign, TextSize, markdown};
//!
//! // Quick path — use the constructor:
//! let v: serde_json::Value = markdown("**hello**").into();
//!
//! // Full control — use struct update syntax:
//! let v: serde_json::Value = Markdown {
//!     content: "**hello**".into(),
//!     text_align: TextAlign::Center,
//!     ..Default::default()
//! }.into();
//! ```

use serde_json::{json, Value};

/// Horizontal text alignment for markdown blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
        }
    }
}

impl std::fmt::Display for TextAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Text size for markdown blocks.
///
/// `NormalV2` maps to `"normal_v2"` and is used in column cells to produce
/// slightly larger text on mobile via the card config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    #[default]
    Normal,
    NormalV2,
    Heading,
    SmallHeading,
}

impl TextSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextSize::Normal => "normal",
            TextSize::NormalV2 => "normal_v2",
            TextSize::Heading => "heading",
            TextSize::SmallHeading => "small_heading",
        }
    }
}

impl std::fmt::Display for TextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Horizontal alignment for column sets and columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl HAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            HAlign::Left => "left",
            HAlign::Center => "center",
            HAlign::Right => "right",
        }
    }
}

impl std::fmt::Display for HAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Vertical alignment for column elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

impl VAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            VAlign::Top => "top",
            VAlign::Center => "center",
            VAlign::Bottom => "bottom",
        }
    }
}

impl std::fmt::Display for VAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Background style for column sets.
///
/// `Grey100` → `"grey-100"`, `Plain` → `"default"` (no background tint).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BgStyle {
    #[default]
    Grey100,
    Plain,
}

impl BgStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BgStyle::Grey100 => "grey-100",
            BgStyle::Plain => "default",
        }
    }
}

impl std::fmt::Display for BgStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Width mode for a column within a column set.
///
/// `Auto` sizes to content; `Weighted` distributes remaining space proportionally
/// using the column's `weight` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnWidth {
    #[default]
    Auto,
    Weighted,
}

impl ColumnWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnWidth::Auto => "auto",
            ColumnWidth::Weighted => "weighted",
        }
    }
}

impl std::fmt::Display for ColumnWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A markdown text block (`"tag": "markdown"`).
///
/// Use the [`markdown`] constructor for the common case. Margin is a CSS
/// shorthand string e.g. `"0px 0px 8px 0px"`.
pub struct Markdown {
    pub content: String,
    pub text_align: TextAlign,
    pub text_size: TextSize,
    pub margin: String,
}

impl Default for Markdown {
    fn default() -> Self {
        Self {
            content: String::new(),
            text_align: TextAlign::Left,
            text_size: TextSize::Normal,
            margin: "0px 0px 0px 0px".into(),
        }
    }
}

impl From<Markdown> for Value {
    fn from(m: Markdown) -> Value {
        json!({
            "tag": "markdown",
            "content": m.content,
            "text_align": m.text_align.as_str(),
            "text_size": m.text_size.as_str(),
            "margin": m.margin,
        })
    }
}

pub fn markdown(content: impl Into<String>) -> Markdown {
    Markdown {
        content: content.into(),
        ..Default::default()
    }
}

/// A colored text-tag badge, used in card header `text_tag_list`.
pub struct TextTag {
    pub text: String,
    /// Lark color name, e.g. `"wathet"`, `"green"`, `"red"`.
    pub color: String,
}

impl From<TextTag> for Value {
    fn from(t: TextTag) -> Value {
        json!({
            "tag": "text_tag",
            "text": {"tag": "plain_text", "content": t.text},
            "color": t.color,
        })
    }
}

pub fn text_tag(text: impl Into<String>, color: impl Into<String>) -> TextTag {
    TextTag {
        text: text.into(),
        color: color.into(),
    }
}

/// Card header block.
///
/// `template` is the Lark color name string (e.g. `"green"`, `"red"`).
/// Prefer using [`CardBuilder::header`] which accepts [`ColorTheme`](crate::ColorTheme)
/// and handles the color-name mapping automatically.
pub struct HeaderBlock {
    pub title: String,
    pub template: String,
    pub subtitle: Option<String>,
    pub text_tag_list: Option<Vec<Value>>,
    pub padding: Option<String>,
}

impl Default for HeaderBlock {
    fn default() -> Self {
        Self {
            title: String::new(),
            template: "blue".into(),
            subtitle: None,
            text_tag_list: None,
            padding: None,
        }
    }
}

impl From<HeaderBlock> for Value {
    fn from(h: HeaderBlock) -> Value {
        let mut v = json!({
            "title": {"tag": "plain_text", "content": h.title},
            "template": h.template,
        });
        if let Some(s) = h.subtitle {
            v["subtitle"] = json!({"tag": "plain_text", "content": s});
        }
        if let Some(tags) = h.text_tag_list {
            v["text_tag_list"] = json!(tags);
        }
        if let Some(p) = h.padding {
            v["padding"] = json!(p);
        }
        v
    }
}

pub fn header(
    title: impl Into<String>,
    template: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    text_tag_list: Option<Vec<Value>>,
    padding: Option<impl Into<String>>,
) -> HeaderBlock {
    HeaderBlock {
        title: title.into(),
        template: template.into(),
        subtitle: subtitle.map(|s| s.into()),
        text_tag_list,
        padding: padding.map(|p| p.into()),
    }
}

pub struct Column {
    pub elements: Vec<Value>,
    pub width: ColumnWidth,
    pub vertical_spacing: String,
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub weight: Option<u32>,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            width: ColumnWidth::Auto,
            vertical_spacing: "8px".into(),
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            weight: None,
        }
    }
}

impl From<Column> for Value {
    fn from(c: Column) -> Value {
        let mut v = json!({
            "tag": "column",
            "width": c.width.as_str(),
            "elements": c.elements,
            "vertical_spacing": c.vertical_spacing,
            "horizontal_align": c.h_align.as_str(),
            "vertical_align": c.v_align.as_str(),
        });
        if let Some(w) = c.weight {
            v["weight"] = json!(w);
        }
        v
    }
}

pub fn column(elements: Vec<Value>) -> Column {
    Column {
        elements,
        ..Default::default()
    }
}

pub struct ColumnSet {
    pub columns: Vec<Value>,
    pub bg_style: BgStyle,
    pub h_spacing: String,
    pub h_align: HAlign,
    pub margin: String,
}

impl Default for ColumnSet {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            bg_style: BgStyle::Grey100,
            h_spacing: "12px".into(),
            h_align: HAlign::Left,
            margin: "0px 0px 0px 0px".into(),
        }
    }
}

impl From<ColumnSet> for Value {
    fn from(cs: ColumnSet) -> Value {
        json!({
            "tag": "column_set",
            "background_style": cs.bg_style.as_str(),
            "horizontal_spacing": cs.h_spacing,
            "horizontal_align": cs.h_align.as_str(),
            "columns": cs.columns,
            "margin": cs.margin,
        })
    }
}

pub fn column_set(columns: Vec<Value>) -> ColumnSet {
    ColumnSet {
        columns,
        ..Default::default()
    }
}

pub struct CollapsiblePanel {
    pub title_markdown: String,
    pub elements: Vec<Value>,
    pub expanded: bool,
    pub bg_color: String,
    pub border_color: String,
    pub corner_radius: String,
    pub vertical_spacing: String,
    pub padding: String,
}

impl Default for CollapsiblePanel {
    fn default() -> Self {
        Self {
            title_markdown: String::new(),
            elements: Vec::new(),
            expanded: false,
            bg_color: "grey-200".into(),
            border_color: "grey".into(),
            corner_radius: "5px".into(),
            vertical_spacing: "8px".into(),
            padding: "8px 8px 8px 8px".into(),
        }
    }
}

impl From<CollapsiblePanel> for Value {
    fn from(p: CollapsiblePanel) -> Value {
        json!({
            "tag": "collapsible_panel",
            "expanded": p.expanded,
            "header": {
                "title": {"tag": "markdown", "content": p.title_markdown},
                "background_color": p.bg_color,
                "vertical_align": "center",
                "icon": {
                    "tag": "standard_icon",
                    "token": "down-small-ccm_outlined",
                    "color": "",
                    "size": "16px 16px",
                },
                "icon_position": "right",
                "icon_expanded_angle": -180,
            },
            "border": {"color": p.border_color, "corner_radius": p.corner_radius},
            "vertical_spacing": p.vertical_spacing,
            "padding": p.padding,
            "elements": p.elements,
        })
    }
}

pub fn collapsible_panel(
    title_markdown: impl Into<String>,
    elements: Vec<Value>,
) -> CollapsiblePanel {
    CollapsiblePanel {
        title_markdown: title_markdown.into(),
        elements,
        ..Default::default()
    }
}

pub struct TemplateReference {
    pub template_id: String,
    pub version_name: String,
    pub variables: Value,
}

impl From<TemplateReference> for Value {
    fn from(t: TemplateReference) -> Value {
        json!({
            "type": "template",
            "data": {
                "template_id": t.template_id,
                "template_version_name": t.version_name,
                "template_variable": t.variables,
            }
        })
    }
}

pub fn template_reference(
    template_id: impl Into<String>,
    version_name: impl Into<String>,
    variables: Value,
) -> TemplateReference {
    TemplateReference {
        template_id: template_id.into(),
        version_name: version_name.into(),
        variables,
    }
}

pub struct Card {
    pub elements: Vec<Value>,
    pub header: Value,
    pub schema: String,
    pub config: Option<Value>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            header: Value::Null,
            schema: "2.0".into(),
            config: None,
        }
    }
}

impl From<Card> for Value {
    fn from(c: Card) -> Value {
        let mut v = json!({
            "schema": c.schema,
            "body": {
                "direction": "vertical",
                "elements": c.elements,
            },
            "header": c.header,
        });
        if let Some(cfg) = c.config {
            v["config"] = cfg;
        }
        v
    }
}

pub fn card(elements: Vec<Value>, header: Value) -> Card {
    Card {
        elements,
        header,
        ..Default::default()
    }
}

pub fn config_textsize_normal_v2() -> Value {
    json!({
        "update_multi": true,
        "style": {
            "text_size": {
                "normal_v2": {
                    "default": "normal",
                    "pc": "normal",
                    "mobile": "heading",
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_defaults() {
        let b: Value = markdown("hello").into();
        assert_eq!(b["tag"], "markdown");
        assert_eq!(b["content"], "hello");
        assert_eq!(b["text_align"], "left");
        assert_eq!(b["text_size"], "normal");
    }

    #[test]
    fn test_header_required_only() {
        let h: Value = header("Title", "blue", None::<&str>, None, None::<&str>).into();
        assert_eq!(h["title"]["tag"], "plain_text");
        assert_eq!(h["title"]["content"], "Title");
        assert_eq!(h["template"], "blue");
        assert!(h.get("subtitle").is_none());
        assert!(h.get("text_tag_list").is_none());
        assert!(h.get("padding").is_none());
    }

    #[test]
    fn test_header_with_optionals() {
        let tags = vec![Value::from(text_tag("running", "wathet"))];
        let h: Value = header(
            "T",
            "green",
            Some("sub"),
            Some(tags),
            Some("12px 8px 12px 8px"),
        )
        .into();
        assert_eq!(h["subtitle"]["content"], "sub");
        assert_eq!(h["text_tag_list"][0]["color"], "wathet");
        assert_eq!(h["padding"], "12px 8px 12px 8px");
    }

    #[test]
    fn test_card_without_config() {
        let hdr: Value = header("H", "blue", None::<&str>, None, None::<&str>).into();
        let c: Value = Card {
            elements: vec![Value::from(markdown("body"))],
            header: hdr,
            ..Default::default()
        }
        .into();
        assert_eq!(c["schema"], "2.0");
        assert_eq!(c["body"]["direction"], "vertical");
        assert_eq!(c["body"]["elements"].as_array().unwrap().len(), 1);
        assert!(c.get("config").is_none());
    }

    #[test]
    fn test_card_with_config() {
        let hdr: Value = header("H", "blue", None::<&str>, None, None::<&str>).into();
        let cfg = config_textsize_normal_v2();
        let c: Value = Card {
            elements: vec![],
            header: hdr,
            config: Some(cfg),
            ..Default::default()
        }
        .into();
        assert!(c.get("config").is_some());
    }

    #[test]
    fn test_column_set() {
        let col1: Value = Column {
            elements: vec![],
            ..Default::default()
        }
        .into();
        let cs: Value = ColumnSet {
            columns: vec![col1],
            ..Default::default()
        }
        .into();
        assert_eq!(cs["tag"], "column_set");
        assert_eq!(cs["background_style"], "grey-100");
    }

    #[test]
    fn test_template_reference() {
        let tr: Value =
            template_reference("tmpl_id", "1.0.0", serde_json::json!({"key": "val"})).into();
        assert_eq!(tr["type"], "template");
        assert_eq!(tr["data"]["template_id"], "tmpl_id");
        assert_eq!(tr["data"]["template_version_name"], "1.0.0");
        assert_eq!(tr["data"]["template_variable"]["key"], "val");
    }

    #[test]
    fn test_text_align_as_str() {
        assert_eq!(TextAlign::Left.as_str(), "left");
        assert_eq!(TextAlign::Center.as_str(), "center");
        assert_eq!(TextAlign::Right.as_str(), "right");
    }

    #[test]
    fn test_text_size_as_str() {
        assert_eq!(TextSize::Normal.as_str(), "normal");
        assert_eq!(TextSize::NormalV2.as_str(), "normal_v2");
        assert_eq!(TextSize::Heading.as_str(), "heading");
        assert_eq!(TextSize::SmallHeading.as_str(), "small_heading");
    }

    #[test]
    fn test_halign_as_str() {
        assert_eq!(HAlign::Left.as_str(), "left");
        assert_eq!(HAlign::Center.as_str(), "center");
        assert_eq!(HAlign::Right.as_str(), "right");
    }

    #[test]
    fn test_valign_as_str() {
        assert_eq!(VAlign::Top.as_str(), "top");
        assert_eq!(VAlign::Center.as_str(), "center");
        assert_eq!(VAlign::Bottom.as_str(), "bottom");
    }

    #[test]
    fn test_bg_style_as_str() {
        assert_eq!(BgStyle::Grey100.as_str(), "grey-100");
        assert_eq!(BgStyle::Plain.as_str(), "default");
    }

    #[test]
    fn test_column_width_as_str() {
        assert_eq!(ColumnWidth::Auto.as_str(), "auto");
        assert_eq!(ColumnWidth::Weighted.as_str(), "weighted");
    }

    #[test]
    fn test_markdown_struct_from() {
        let v: Value = Markdown {
            content: "hello".into(),
            ..Default::default()
        }
        .into();
        assert_eq!(v["tag"], "markdown");
        assert_eq!(v["content"], "hello");
        assert_eq!(v["text_align"], "left");
        assert_eq!(v["text_size"], "normal");
        assert_eq!(v["margin"], "0px 0px 0px 0px");
    }

    #[test]
    fn test_markdown_struct_custom_align() {
        let v: Value = Markdown {
            content: "hi".into(),
            text_align: TextAlign::Center,
            text_size: TextSize::NormalV2,
            ..Default::default()
        }
        .into();
        assert_eq!(v["text_align"], "center");
        assert_eq!(v["text_size"], "normal_v2");
    }

    #[test]
    fn test_text_tag_struct_from() {
        let v: Value = TextTag {
            text: "running".into(),
            color: "wathet".into(),
        }
        .into();
        assert_eq!(v["tag"], "text_tag");
        assert_eq!(v["text"]["tag"], "plain_text");
        assert_eq!(v["text"]["content"], "running");
        assert_eq!(v["color"], "wathet");
    }

    #[test]
    fn test_header_block_struct_required_only() {
        let v: Value = HeaderBlock {
            title: "Title".into(),
            template: "blue".into(),
            ..Default::default()
        }
        .into();
        assert_eq!(v["title"]["tag"], "plain_text");
        assert_eq!(v["title"]["content"], "Title");
        assert_eq!(v["template"], "blue");
        assert!(v.get("subtitle").is_none());
        assert!(v.get("text_tag_list").is_none());
        assert!(v.get("padding").is_none());
    }

    #[test]
    fn test_header_block_struct_with_optionals() {
        let tag: Value = TextTag {
            text: "ok".into(),
            color: "green".into(),
        }
        .into();
        let v: Value = HeaderBlock {
            title: "T".into(),
            template: "green".into(),
            subtitle: Some("sub".into()),
            text_tag_list: Some(vec![tag]),
            padding: Some("12px 8px 12px 8px".into()),
        }
        .into();
        assert_eq!(v["subtitle"]["content"], "sub");
        assert_eq!(v["text_tag_list"][0]["color"], "green");
        assert_eq!(v["padding"], "12px 8px 12px 8px");
    }

    #[test]
    fn test_column_struct_from() {
        let v: Value = Column {
            elements: vec![],
            ..Default::default()
        }
        .into();
        assert_eq!(v["tag"], "column");
        assert_eq!(v["width"], "auto");
        assert_eq!(v["vertical_spacing"], "8px");
        assert_eq!(v["horizontal_align"], "left");
        assert_eq!(v["vertical_align"], "top");
        assert!(v.get("weight").is_none());
    }

    #[test]
    fn test_column_struct_weighted() {
        let v: Value = Column {
            elements: vec![],
            width: ColumnWidth::Weighted,
            weight: Some(2),
            ..Default::default()
        }
        .into();
        assert_eq!(v["width"], "weighted");
        assert_eq!(v["weight"], 2);
    }

    #[test]
    fn test_column_set_struct_from() {
        let v: Value = ColumnSet {
            columns: vec![],
            ..Default::default()
        }
        .into();
        assert_eq!(v["tag"], "column_set");
        assert_eq!(v["background_style"], "grey-100");
        assert_eq!(v["horizontal_spacing"], "12px");
        assert_eq!(v["horizontal_align"], "left");
    }

    #[test]
    fn test_collapsible_panel_struct_from() {
        let v: Value = CollapsiblePanel {
            title_markdown: "Details".into(),
            elements: vec![],
            ..Default::default()
        }
        .into();
        assert_eq!(v["tag"], "collapsible_panel");
        assert_eq!(v["expanded"], false);
        assert_eq!(v["header"]["title"]["content"], "Details");
        assert_eq!(v["border"]["corner_radius"], "5px");
    }

    #[test]
    fn test_template_reference_struct_from() {
        let v: Value = TemplateReference {
            template_id: "tmpl_id".into(),
            version_name: "1.0.0".into(),
            variables: serde_json::json!({"k": "v"}),
        }
        .into();
        assert_eq!(v["type"], "template");
        assert_eq!(v["data"]["template_id"], "tmpl_id");
        assert_eq!(v["data"]["template_version_name"], "1.0.0");
        assert_eq!(v["data"]["template_variable"]["k"], "v");
    }

    #[test]
    fn test_card_struct_from() {
        let hdr: Value = HeaderBlock {
            title: "H".into(),
            template: "blue".into(),
            ..Default::default()
        }
        .into();
        let v: Value = Card {
            elements: vec![],
            header: hdr,
            ..Default::default()
        }
        .into();
        assert_eq!(v["schema"], "2.0");
        assert_eq!(v["body"]["direction"], "vertical");
        assert!(v.get("config").is_none());
    }

    #[test]
    fn test_card_struct_with_config() {
        let hdr: Value = HeaderBlock {
            title: "H".into(),
            template: "blue".into(),
            ..Default::default()
        }
        .into();
        let cfg = config_textsize_normal_v2();
        let v: Value = Card {
            elements: vec![],
            header: hdr,
            config: Some(cfg),
            ..Default::default()
        }
        .into();
        assert!(v.get("config").is_some());
    }
}
