//! Configuration, parser and item model of requimd.
//!
//! Positions are byte offsets into the source text. Conversion to lines and
//! columns happens only at the output boundary (CLI, LSP).

pub mod config;
pub mod diag;
pub mod fmt;
pub mod lock;
pub mod model;
pub mod parse;
pub mod project;
pub mod source;
pub mod words;

pub use config::{Config, ConfigError, WordEntry, find_root};
pub use diag::{Diagnostic, Level, Severity};
pub use model::{BodyField, Item, Link, MetaPair, Reference, Step, Target, Term};
pub use project::{
    Includes, LoadError, Project, Resolution, glossary_rel, project_files, read_text,
    relative_path, scan_files,
};
pub use source::{FileId, SourceFile};
