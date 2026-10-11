//! Diagnostics and severities.

use std::fmt;
use std::ops::Range;

use crate::source::FileId;

/// Severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Configured setting of a check: a severity or `off` (FR-117).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Off,
    On(Severity),
}

impl Level {
    pub fn parse(s: &str) -> Option<Level> {
        match s {
            "off" => Some(Level::Off),
            "error" => Some(Level::On(Severity::Error)),
            "warning" => Some(Level::On(Severity::Warning)),
            "info" => Some(Level::On(Severity::Info)),
            _ => None,
        }
    }
}

/// A finding at a byte range of a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub file: FileId,
    pub span: Range<usize>,
    pub severity: Severity,
    pub check: &'static str,
    pub message: String,
}
