//! Source files and position conversion.

use std::path::PathBuf;

/// Index of a file in [`crate::Project::files`].
pub type FileId = usize;

/// A Markdown file of the project.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Absolute path.
    pub path: PathBuf,
    /// Path relative to the project root, with `/` as separator.
    pub rel: String,
    pub text: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(path: PathBuf, rel: String, text: String) -> SourceFile {
        let mut line_starts = vec![0];
        line_starts.extend(text.match_indices('\n').map(|(i, _)| i + 1));
        SourceFile {
            path,
            rel,
            text,
            line_starts,
        }
    }

    /// 1-based line and 1-based column of a byte offset. The column counts
    /// bytes, like compilers and Vim's quickfix list do.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.text.len());
        let line = self
            .line_starts
            .partition_point(|&s| s <= offset)
            .saturating_sub(1);
        let start = self.line_starts.get(line).copied().unwrap_or(0);
        (line + 1, offset - start + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr_300_line_and_column_are_one_based() {
        let f = SourceFile::new(PathBuf::from("a.md"), "a.md".into(), "ab\ncd\n".into());
        assert_eq!(f.line_col(0), (1, 1));
        assert_eq!(f.line_col(1), (1, 2));
        assert_eq!(f.line_col(3), (2, 1));
        assert_eq!(f.line_col(6), (3, 1));
        assert_eq!(f.line_col(99), (3, 1));
    }
}
