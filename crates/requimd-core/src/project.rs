//! A loaded project: files, items and the ID index.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

use crate::config::Config;
use crate::lock::{LOCK_FILE, Lock};
use crate::model::{Item, Target, Term};
use crate::parse::{Parsed, parse, parse_glossary};
use crate::source::{FileId, SourceFile};

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub config: Config,
    pub files: Vec<SourceFile>,
    pub items: Vec<Item>,
    ids: HashMap<String, Vec<usize>>,
    /// Glossary terms (FR-011) and the file they come from.
    pub glossary: Vec<Term>,
    pub glossary_file: Option<FileId>,
    /// Code ranges per file (FR-009).
    pub code: Vec<Vec<std::ops::Range<usize>>>,
    /// The lock file as read (ADR-0007); empty if there is none.
    pub lock: Lock,
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
        let includes = Includes::new(&config.include).map_err(|message| LoadError {
            path: root.clone(),
            message,
        })?;
        if let Some(g) = glossary_rel(&config)
            && !root.join(&g).is_file()
        {
            return Err(LoadError {
                path: root.join(g),
                message: "glossary file not found (set in [project] glossary)".into(),
            });
        }
        let lock = Lock::load(&root).map_err(|message| LoadError {
            path: root.join(LOCK_FILE),
            message,
        })?;
        let mut files = Vec::new();
        for (rel, path) in project_files(&root, &config, &includes)? {
            let text = read_text(&path)?;
            files.push(SourceFile::new(path, rel, text));
        }
        let mut project = Project::from_files(root, config, files);
        project.lock = lock;
        Ok(project)
    }

    /// Builds a project from files already in memory. The glossary is the
    /// file whose relative path is the configured glossary (FR-011).
    pub fn from_files(root: PathBuf, config: Config, files: Vec<SourceFile>) -> Project {
        let parsed = files
            .iter()
            .enumerate()
            .map(|(id, f)| parse(&config, id, &f.text))
            .collect();
        Project::from_parsed(root, config, files, parsed)
    }

    /// Builds a project from files and their parse results, one per file in
    /// the same order, e.g. cached by the language server.
    pub fn from_parsed(
        root: PathBuf,
        config: Config,
        files: Vec<SourceFile>,
        parsed: Vec<Parsed>,
    ) -> Project {
        let mut items = Vec::new();
        let mut code = Vec::with_capacity(parsed.len());
        for (id, p) in parsed.into_iter().enumerate() {
            items.extend(p.items.into_iter().map(|mut it| {
                it.file = id;
                it
            }));
            code.push(p.code);
        }
        let mut ids: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            if let Some(id) = &item.id {
                ids.entry(id.clone()).or_default().push(i);
            }
        }
        let glossary_file =
            glossary_rel(&config).and_then(|g| files.iter().position(|f| f.rel == g));
        let glossary = glossary_file
            .and_then(|i| files.get(i))
            .map(|f| parse_glossary(&f.text))
            .unwrap_or_default();
        Project {
            root,
            config,
            files,
            items,
            ids,
            glossary,
            glossary_file,
            code,
            lock: Lock::default(),
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

    /// FR-212: one higher than the highest ID of `prefix` in the project
    /// and in the lock file, zero-padded to the configured digits, so
    /// deleted IDs are not reused (BR-002).
    pub fn next_id(&self, prefix: &str) -> String {
        self.format_id(prefix, self.highest_number(prefix).saturating_add(1))
    }

    /// The highest number of an ID with `prefix` in the project and the
    /// lock file, 0 if there is none.
    pub fn highest_number(&self, prefix: &str) -> u64 {
        let in_project = self
            .ids
            .keys()
            .filter_map(|id| {
                id.strip_prefix(prefix)?
                    .strip_prefix('-')?
                    .parse::<u64>()
                    .ok()
            })
            .max()
            .unwrap_or(0);
        in_project.max(self.lock.ids.get(prefix).copied().unwrap_or(0))
    }

    /// An ID with the configured number of digits (BR-001).
    pub fn format_id(&self, prefix: &str, number: u64) -> String {
        format!("{prefix}-{number:0width$}", width = self.config.digits)
    }
}

/// Include patterns of the project (FR-013, FR-201).
#[derive(Debug, Clone)]
pub struct Includes(GlobSet);

impl Includes {
    pub fn new(patterns: &[String]) -> Result<Includes, String> {
        let mut b = GlobSetBuilder::new();
        for p in patterns {
            let glob = GlobBuilder::new(p)
                .literal_separator(true)
                .build()
                .map_err(|e| format!("include pattern \"{p}\": {e}"))?;
            b.add(glob);
        }
        b.build().map(Includes).map_err(|e| e.to_string())
    }

    /// Whether a path relative to the root, with `/` separators, matches.
    pub fn matches(&self, rel: &str) -> bool {
        self.0.is_match(rel)
    }
}

/// Path relative to `root` with `/` separators, if `path` is below it.
pub fn relative_path(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    Some(parts.join("/"))
}

/// Included files below `root`, sorted, as relative and absolute path.
pub fn scan_files(root: &Path, includes: &Includes) -> Result<Vec<(String, PathBuf)>, LoadError> {
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| e.file_name() != ".git");
    for entry in walker {
        let entry = entry.map_err(|e| LoadError {
            path: e
                .path()
                .map_or_else(|| root.to_path_buf(), Path::to_path_buf),
            message: e.to_string(),
        })?;
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(rel) = relative_path(root, entry.path()) else {
            continue;
        };
        if includes.matches(&rel) {
            files.push((rel, entry.into_path()));
        }
    }
    Ok(files)
}

/// The configured glossary path relative to the root, with `/`.
pub fn glossary_rel(config: &Config) -> Option<String> {
    let g = config.glossary.as_ref()?.replace('\\', "/");
    Some(g.strip_prefix("./").unwrap_or(&g).to_string())
}

/// Included files plus the glossary file if it exists, which is part of
/// the project even when the include patterns do not match it.
pub fn project_files(
    root: &Path,
    config: &Config,
    includes: &Includes,
) -> Result<Vec<(String, PathBuf)>, LoadError> {
    let mut files = scan_files(root, includes)?;
    if let Some(g) = glossary_rel(config)
        && !files.iter().any(|(rel, _)| *rel == g)
    {
        let path = root.join(&g);
        if path.is_file() {
            files.push((g, path));
            files.sort();
        }
    }
    Ok(files)
}

/// Reads a file as text; invalid UTF-8 is replaced, never an error.
pub fn read_text(path: &Path) -> Result<String, LoadError> {
    let bytes = std::fs::read(path).map_err(|e| LoadError {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
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
        assert!(
            p.files
                .iter()
                .all(|f| f.rel.starts_with("requirements/") || f.rel.starts_with("acceptance/"))
        );
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
    fn fr_212_next_id_is_one_higher_than_highest() {
        let p = project(&[("a.md", "## FR-007 A\n## FR-102 B\n## FR-? C\n## UC-001 U\n")]);
        assert_eq!(p.next_id("FR"), "FR-103");
        assert_eq!(p.next_id("UC"), "UC-002");
        assert_eq!(p.next_id("TC"), "TC-001");
        let p = project(&[("a.md", "## FR-1234 A\n")]);
        assert_eq!(p.next_id("FR"), "FR-1235");
    }

    #[test]
    fn fr_201_include_patterns_match_relative_paths() {
        let inc = Includes::new(&["requirements/*.md".to_string()]).unwrap();
        assert!(inc.matches("requirements/a.md"));
        assert!(!inc.matches("requirements/sub/a.md"));
        assert!(!inc.matches("README.md"));
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
