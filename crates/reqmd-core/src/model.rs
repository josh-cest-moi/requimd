//! Item model. All spans are byte ranges into the item's source file.

use std::ops::Range;

use crate::source::FileId;

/// An item declared by a heading (FR-001).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub file: FileId,
    /// `None` for a placeholder ID such as `FR-?` (FR-010).
    pub id: Option<String>,
    pub prefix: String,
    /// Name of the item type, e.g. `functional`.
    pub kind: String,
    pub title: String,
    pub level: u8,
    /// The heading line.
    pub heading: Range<usize>,
    pub id_span: Range<usize>,
    /// From the line after the heading to the end of the item (FR-002).
    pub body: Range<usize>,
    /// Metadata pairs (FR-003), including unknown keys and link keys.
    pub meta: Vec<MetaPair>,
    pub links: Vec<Link>,
    pub fields: Vec<BodyField>,
    pub steps: Vec<Step>,
    pub mentions: Vec<Reference>,
}

impl Item {
    /// The ID as written, `FR-?` for a placeholder.
    pub fn display_id(&self) -> String {
        match &self.id {
            Some(id) => id.clone(),
            None => format!("{}-?", self.prefix),
        }
    }

    /// Value of a known metadata key.
    pub fn meta_value(&self, key: &str) -> Option<&str> {
        self.meta
            .iter()
            .find(|p| p.known && p.key == key)
            .map(|p| p.value.as_str())
    }
}

/// A `key: value` pair of a metadata block (FR-003).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaPair {
    pub key: String,
    pub key_span: Range<usize>,
    pub value: String,
    pub value_span: Range<usize>,
    /// Whether the key is configured for the item type (FR-102).
    pub known: bool,
}

/// A link from a metadata link key, e.g. `impl: UC-012.s2` (FR-004).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub key: String,
    pub reference: Reference,
}

/// A reference to an item or a step, from a link or a mention (FR-007,
/// FR-008).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub text: String,
    pub span: Range<usize>,
    /// `None` if the text is not a valid reference.
    pub target: Option<Target>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    pub id: String,
    pub step: Option<String>,
}

/// A body field such as `pre:` or `ac:` (FR-005).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyField {
    pub key: String,
    pub key_span: Range<usize>,
    /// Rest of the line plus a list that follows directly.
    pub value_span: Range<usize>,
}

/// A use case step such as `- s2: System shows the total.` (FR-006).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub label: String,
    pub label_span: Range<usize>,
    /// The step text after the colon.
    pub text_span: Range<usize>,
}
