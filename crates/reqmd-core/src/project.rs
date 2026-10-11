//! A loaded project: files, items and the ID index.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

use crate::config::Config;
use crate::model::{Item, Target};
use crate::parse::parse_file;
use crate::source::SourceFile;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub config: Config,
    pub files: Vec<SourceFile>,
    pub items: Vec<Item>,
    ids: HashMap<String, Vec<usize>>,
}

/// Result of resolving a reference (FR-007, FR-100).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Item(usize),
    /// Item index and step index.
    Step(usize, usize),
    /// The ID is declared more than once; resolves to no item (FR-100).
    Duplicate,
    MissingItem,
    /// The item exists but has no such step.
    MissingStep(usize),
}

#[derive(Debug)]
pub struct LoadError {
    pub path: PathBuf,
    pub message: String,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.message)
    }
}

impl std::error::Error for LoadError {}

impl Project {
    /// Reads all included files below `root` (FR-013). Identity and
    /// relationships come only from file content (BR-005).
    pub fn load(root: &Path, config: Config) -> Result<Project, LoadError> {
        let root = std::path::absolute(root).map_err(|e| LoadError {
            path: root.to_path_buf(),
            message: e.to_string(),
        })?;
        let globs = include_set(&config.include).map_err(|message| LoadError {
            path: root.clone(),
            message,
        })?;
        let mut files = Vec::new();
        let walker = walkdir::WalkDir::new(&root)
            .sort_by_file_name()
            .into_iter()
            .filter_entry(|e| e.file_name() != ".git");
        for entry in walker {
            let entry = entry.map_err(|e| LoadError {
                path: e.path().map_or_else(|| root.clone(), Path::to_path_buf),
                message: e.to_string(),
            })?;
            if !entry.file_type().is_file() {
                continue;
            }
            let Ok(rel) = entry.path().strip_prefix(&root) else {
                continue;
            };
            let rel: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            let rel = rel.join("/");
            if !globs.is_match(&rel) {
                continue;
            }
            let bytes = std::fs::read(entry.path()).map_err(|e| LoadError {
                path: entry.path().to_path_buf(),
                message: e.to_string(),
            })?;
            let text = String::from_utf8_lossy(&bytes).into_owned();
            files.push(SourceFile::new(entry.path().to_path_buf(), rel, text));
        }
        Ok(Project::from_files(root, config, files))
    }

    /// Builds a project from files already in memory.
    pub fn from_files(root: PathBuf, config: Config, files: Vec<SourceFile>) -> Project {
        let mut items = Vec::new();
        for (id, f) in files.iter().enumerate() {
            items.extend(parse_file(&config, id, &f.text));
        }
        let mut ids: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            if let Some(id) = &item.id {
                ids.entry(id.clone()).or_default().push(i);
            }
        }
        Project {
            root,
            config,
            files,
            items,
            ids,
        }
    }

    /// Indices of the items declaring `id`.
    pub fn declarations(&self, id: &str) -> &[usize] {
        self.ids.get(id).map_or(&[], Vec::as_slice)
    }

    pub fn resolve(&self, target: &Target) -> Resolution {
        let item = match self.declarations(&target.id) {
            [] => return Resolution::MissingItem,
            [i] => *i,
            _ => return Resolution::Duplicate,
        };
        let Some(label) = &target.step else {
            return Resolution::Item(item);
        };
        self.items
            .get(item)
            .and_then(|it| it.steps.iter().position(|s| &s.label == label))
            .map_or(Resolution::MissingStep(item), |s| Resolution::Step(item, s))
    }
}

fn include_set(patterns: &[String]) -> Result<GlobSet, String> {
    let mut b = GlobSetBuilder::new();
    for p in patterns {
        let glob = GlobBuilder::new(p)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("include pattern \"{p}\": {e}"))?;
        b.add(glob);
    }
    b.build().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(files: &[(&str, &str)]) -> Project {
        let config = Config::defaults().unwrap();
        let files = files
            .iter()
            .map(|(name, text)| {
                SourceFile::new(PathBuf::from(name), name.to_string(), text.to_string())
            })
            .collect();
        Project::from_files(PathBuf::from("."), config, files)
    }

    fn target(id: &str, step: Option<&str>) -> Target {
        Target {
            id: id.into(),
            step: step.map(Into::into),
        }
    }

    #[test]
    fn fr_007_resolves_items_and_steps() {
        let p = project(&[
            ("a.md", "## UC-001 U\n\n- s1: A.\n- s2a: B.\n"),
            ("b.md", "## FR-001 F\n"),
        ]);
        assert_eq!(p.resolve(&target("UC-001", None)), Resolution::Item(0));
        assert_eq!(
            p.resolve(&target("UC-001", Some("s2a"))),
            Resolution::Step(0, 1)
        );
        assert_eq!(
            p.resolve(&target("UC-001", Some("s9"))),
            Resolution::MissingStep(0)
        );
        assert_eq!(p.resolve(&target("FR-001", None)), Resolution::Item(1));
        assert_eq!(p.resolve(&target("FR-999", None)), Resolution::MissingItem);
    }

    #[test]
    fn fr_100_duplicate_ids_resolve_to_no_item() {
        let p = project(&[("a.md", "## FR-001 A\n## FR-001 B\n")]);
        assert_eq!(p.declarations("FR-001"), [0, 1]);
        assert_eq!(p.resolve(&target("FR-001", None)), Resolution::Duplicate);
    }

    #[test]
    fn fr_013_load_uses_include_patterns() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let config = Config::load(&root).unwrap();
        let p = Project::load(&root, config).unwrap();
        assert!(p.files.iter().all(|f| f.rel.starts_with("requirements/")));
        assert!(
            p.files
                .iter()
                .any(|f| f.rel == "requirements/functional/format.md")
        );
        assert!(!p.declarations("FR-001").is_empty());
    }

    #[test]
    fn fr_014_default_include_is_all_markdown_files() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/shop");
        let p = Project::load(&root, Config::load(&root).unwrap()).unwrap();
        let names: Vec<&str> = p.files.iter().map(|f| f.rel.as_str()).collect();
        assert!(names.contains(&"usecases.md") && names.contains(&"README.md"));
        assert!(names.iter().all(|n| n.ends_with(".md")));
    }

    #[test]
    fn br_005_file_layout_carries_no_meaning() {
        let a = "## UC-001 U\n\n- s1: A.\n";
        let b = "## FR-001 F\nimpl: UC-001.s1\n";
        let split = project(&[("x/one.md", a), ("y/two.md", b)]);
        let merged = project(&[("all.md", &format!("{a}\n{b}"))]);
        let summary = |p: &Project| -> Vec<(Option<String>, String, usize, usize)> {
            p.items
                .iter()
                .map(|i| (i.id.clone(), i.kind.clone(), i.links.len(), i.steps.len()))
                .collect()
        };
        assert_eq!(summary(&split), summary(&merged));
        assert_eq!(
            split.resolve(&target("UC-001", Some("s1"))),
            merged.resolve(&target("UC-001", Some("s1")))
        );
    }
}
