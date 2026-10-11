//! Configuration, parser and item model of reqmd.
//!
//! Positions are byte offsets into the source text. Conversion to lines and
//! columns happens only at the output boundary (CLI, LSP).

pub mod config;
pub mod diag;
pub mod model;
pub mod parse;
pub mod project;
pub mod source;

pub use config::{Config, ConfigError, find_root};
pub use diag::{Diagnostic, Level, Severity};
pub use model::{BodyField, Item, Link, MetaPair, Reference, Step, Target};
pub use project::{LoadError, Project, Resolution};
pub use source::{FileId, SourceFile};
