//! Conversion at the LSP boundary: byte offsets to positions and back,
//! file paths to URIs and back.

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use lsp_types::{Position, Uri};

/// Unit of `Position::character` negotiated with the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf16,
}

/// Line starts of a text, for position conversion.
pub struct LineIndex<'a> {
    text: &'a str,
    starts: Vec<usize>,
    encoding: Encoding,
}

impl<'a> LineIndex<'a> {
    pub fn new(text: &'a str, encoding: Encoding) -> LineIndex<'a> {
        let mut starts = vec![0];
        starts.extend(text.match_indices('\n').map(|(i, _)| i + 1));
        LineIndex {
            text,
            starts,
            encoding,
        }
    }

    pub fn position(&self, offset: usize) -> Position {
        let offset = clamp_to_char(self.text, offset.min(self.text.len()));
        let line = self
            .starts
            .partition_point(|&s| s <= offset)
            .saturating_sub(1);
        let start = self.starts.get(line).copied().unwrap_or(0);
        let prefix = self.text.get(start..offset).unwrap_or_default();
        let character = match self.encoding {
            Encoding::Utf8 => prefix.len(),
            Encoding::Utf16 => prefix.encode_utf16().count(),
        };
        Position {
            line: u32::try_from(line).unwrap_or(u32::MAX),
            character: u32::try_from(character).unwrap_or(u32::MAX),
        }
    }

    pub fn range(&self, r: &Range<usize>) -> lsp_types::Range {
        lsp_types::Range {
            start: self.position(r.start),
            end: self.position(r.end),
        }
    }

    /// Byte offset of a position; positions beyond a line end or the text
    /// are clamped.
    pub fn offset(&self, pos: Position) -> usize {
        let Some(&start) = self.starts.get(pos.line as usize) else {
            return self.text.len();
        };
        let end = self
            .starts
            .get(pos.line as usize + 1)
            .map_or(self.text.len(), |&n| n.saturating_sub(1));
        let line = self.text.get(start..end).unwrap_or_default();
        let line = line.strip_suffix('\r').unwrap_or(line);
        let mut units = 0usize;
        for (i, c) in line.char_indices() {
            if units >= pos.character as usize {
                return start + i;
            }
            units += match self.encoding {
                Encoding::Utf8 => c.len_utf8(),
                Encoding::Utf16 => c.len_utf16(),
            };
        }
        start + line.len()
    }
}

fn clamp_to_char(text: &str, mut offset: usize) -> usize {
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

/// Path of a `file:` URI.
pub fn uri_to_path(uri: &Uri) -> Option<PathBuf> {
    let s = uri.as_str();
    let rest = s.strip_prefix("file://")?;
    // Skip an authority such as `localhost`.
    let path = &rest[rest.find('/')?..];
    let decoded = percent_decode(path)?;
    if cfg!(windows) {
        // `/C:/x` → `C:\x`, drive letter upper-case.
        let trimmed = decoded.strip_prefix('/').unwrap_or(&decoded);
        let mut chars = trimmed.chars();
        let drive = chars.next()?;
        if chars.next() != Some(':') {
            return Some(PathBuf::from(decoded));
        }
        let tail: String = chars.collect();
        Some(PathBuf::from(format!(
            "{}:{}",
            drive.to_ascii_uppercase(),
            tail.replace('/', "\\")
        )))
    } else {
        Some(PathBuf::from(decoded))
    }
}

/// The real spelling of a path, so that paths from different sources
/// compare equal: Windows 8.3 short names such as `RUNNER~1` are expanded
/// and symbolic links resolved. A path that does not exist (a deleted file)
/// is resolved through its directory.
pub fn real_path(path: &Path) -> PathBuf {
    if let Ok(p) = std::fs::canonicalize(path) {
        return strip_verbatim(p);
    }
    if let (Some(dir), Some(name)) = (path.parent(), path.file_name())
        && let Ok(d) = std::fs::canonicalize(dir)
    {
        return strip_verbatim(d).join(name);
    }
    path.to_path_buf()
}

/// `\\?\C:\x` → `C:\x`, `\\?\UNC\server\share` → `\\server\share`.
fn strip_verbatim(p: PathBuf) -> PathBuf {
    let s = p.to_string_lossy();
    if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{rest}"));
    }
    if let Some(rest) = s.strip_prefix(r"\\?\")
        && rest.get(1..2) == Some(":")
    {
        return PathBuf::from(rest);
    }
    p
}

/// `file:` URI of an absolute path.
pub fn path_to_uri(path: &std::path::Path) -> Option<Uri> {
    let s = path.to_string_lossy().replace('\\', "/");
    let s = if s.starts_with('/') {
        s
    } else {
        format!("/{s}")
    };
    let mut out = String::from("file://");
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"/-._~".contains(&b) {
            out.push(char::from(b));
        } else if b == b':' {
            // Drive letter colon, as most clients write it.
            out.push(':');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    Uri::from_str(&out).ok()
}

fn percent_decode(s: &str) -> Option<String> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b.get(i) == Some(&b'%') {
            let hex = s.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(*b.get(i)?);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr_202_positions_in_utf16_and_utf8() {
        let t = "ab\näxy\r\n𝄞z\n";
        let u16 = LineIndex::new(t, Encoding::Utf16);
        let u8 = LineIndex::new(t, Encoding::Utf8);
        let x = t.find('x').unwrap();
        assert_eq!(u16.position(x), Position::new(1, 1));
        assert_eq!(u8.position(x), Position::new(1, 2));
        let z = t.find('z').unwrap();
        assert_eq!(u16.position(z), Position::new(2, 2));
        assert_eq!(u8.position(z), Position::new(2, 4));
        for i in [0, x, z, t.len()] {
            assert_eq!(u16.offset(u16.position(i)), i);
            assert_eq!(u8.offset(u8.position(i)), i);
        }
        // Beyond the line end: clamped before "\r\n".
        assert_eq!(u16.offset(Position::new(1, 99)), t.find('\r').unwrap());
        assert_eq!(u16.offset(Position::new(99, 0)), t.len());
    }

    #[test]
    fn fr_217_real_path_expands_short_names_and_keeps_missing_files() {
        let dir = std::env::temp_dir().join("requimd-long-directory-name-test");
        std::fs::create_dir_all(&dir).unwrap();
        let real = real_path(&dir);
        assert!(!real.to_string_lossy().starts_with(r"\\?\"));
        // A file that does not exist keeps its name below the real directory.
        assert_eq!(real_path(&dir.join("gone.md")), real.join("gone.md"));
        if cfg!(windows) {
            // The 8.3 short form, if the volume has short names.
            let out = std::process::Command::new("cmd")
                .args(["/c", "for %I in (.) do @echo %~sI"])
                .current_dir(&dir)
                .output()
                .unwrap();
            let short = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
            if short != dir {
                assert_eq!(real_path(&short), real);
                assert_eq!(real_path(&short.join("gone.md")), real.join("gone.md"));
            }
        }
    }

    #[test]
    fn fr_200_uri_roundtrip() {
        let path = if cfg!(windows) {
            PathBuf::from(r"C:\Users\a b\req ä.md")
        } else {
            PathBuf::from("/home/a b/req ä.md")
        };
        let uri = path_to_uri(&path).unwrap();
        assert!(uri.as_str().starts_with("file:///"));
        assert_eq!(uri_to_path(&uri).unwrap(), path);
        if cfg!(windows) {
            let lower = Uri::from_str("file:///c%3A/Users/x.md").unwrap();
            assert_eq!(
                uri_to_path(&lower).unwrap(),
                PathBuf::from(r"C:\Users\x.md")
            );
        }
    }
}
