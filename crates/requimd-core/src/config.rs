//! Project configuration: discovery (FR-013) and built-in defaults (FR-014).

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::diag::Level;

pub const CONFIG_FILE: &str = "requimd.toml";

/// Item type whose items have steps (FR-006).
pub const USECASE: &str = "usecase";
/// Name of a step as link target type in `to` lists (FR-104).
pub const STEP: &str = "step";
/// Link key whose cycles are reported (FR-108).
pub const REF: &str = "ref";
/// Link key that covers use case steps (FR-106).
pub const IMPL: &str = "impl";
/// Item types checked for weak words (FR-110).
pub const FUNCTIONAL: &str = "functional";
pub const QUALITY: &str = "quality";
/// Item type checked for the user story pattern (FR-113).
pub const STORY: &str = "story";

/// Built-in weak words with reasons (FR-110). Configured words extend them.
pub const WEAK_WORDS: &[(&str, &str)] = &[
    ("fast", "not measurable; state a time limit"),
    ("quick", "not measurable; state a time limit"),
    ("quickly", "not measurable; state a time limit"),
    (
        "easy",
        "not testable; state who does what in how many steps",
    ),
    (
        "user-friendly",
        "not testable; name the usability criterion",
    ),
    ("intuitive", "not testable; name the usability criterion"),
    (
        "flexible",
        "vague; name the variations that must be supported",
    ),
    ("robust", "vague; name the faults that must be tolerated"),
    ("appropriate", "vague; state the criterion"),
    ("adequate", "vague; state the criterion"),
    ("several", "vague quantity; state the number"),
    ("etc.", "incomplete list; name all items"),
    (
        "and/or",
        "ambiguous; write \"and\", \"or\" or \"one or both of\"",
    ),
    ("as needed", "unclear trigger; state the condition"),
    (
        "if possible",
        "optional in disguise; decide if it is required",
    ),
    ("simple", "not testable; state the criterion"),
    ("efficient", "not measurable; state the resource limit"),
    ("sufficient", "vague; state the amount"),
    ("normally", "unclear; state the condition or the exception"),
    ("usually", "unclear; state the condition or the exception"),
];

/// Built-in modal verbs other than "shall" (FR-118).
pub const MODAL_VERBS: &[&str] = &["should", "may", "can", "could", "might", "will"];

/// Checks that honour `allow` lists and inline markers (FR-119).
pub const ALLOW_CHECKS: &[&str] = &["weak-word", "modal-verb", "glossary-alias"];

/// Check IDs that may be configured (FR-117), including those of later
/// milestones, so a configuration stays valid while checks are added.
pub const CHECK_IDS: &[&str] = &[
    "duplicate-id",
    "dangling-ref",
    "unknown-key",
    "invalid-value",
    "link-target",
    "missing-key",
    "uncovered-step",
    "duplicate-step",
    "ref-cycle",
    "status-link",
    "weak-word",
    "no-shall",
    "ears",
    "story-pattern",
    "glossary-alias",
    "placeholder",
    "suspect-link",
    "modal-verb",
    "unused-allow",
];

/// Written by `requimd init` (FR-303). Without the glossary line it is also
/// the source of the built-in defaults (FR-014), so both cannot diverge.
pub const INIT_TEMPLATE: &str = r#"# requimd configuration. Every section is optional; a missing section or
# entry uses the built-in default shown here (FR-014). An entry such as
# [types.quality] replaces the default entry of the same name completely.

[project]
# Files that contain items, relative to this file.
include = ["**/*.md"]
glossary = "glossary.md"

[ids]
# Minimum number of digits of an item number, e.g. FR-007.
digits = 3

# Item types. `meta`: metadata keys, `body`: body field keys, `required`:
# keys every item of the type needs; "a|b" means one of them.
[types.usecase]
prefix = "UC"
meta = ["actor"]
body = ["pre", "post", "trigger"]

[types.functional]
prefix = "FR"

[types.quality]
prefix = "NFR"
meta = ["scale", "meter", "must", "plan"]
required = ["scale", "meter", "must|plan"]

[types.rule]
prefix = "BR"
meta = ["src"]

[types.story]
prefix = "US"
body = ["ac"]

[types.test]
prefix = "TC"

# Link types. `to`: allowed target types ("step" for use case steps);
# without `to`, any item or step is allowed.
[links.impl]
name = "implements"
to = ["usecase", "step"]

[links.ref]
name = "refines"

[links.ver]
name = "verifies"
to = ["functional", "quality", "story"]

[fields]
# Metadata keys allowed on every item type.
meta = ["status", "prio"]

[values]
status = ["draft", "review", "approved", "deprecated"]
prio = ["must", "should", "could", "wont"]

[defaults]
status = "draft"

[checks]
# Severity by check id: "error", "warning", "info" or "off", e.g.
# uncovered-step = "info"

# Language checks: `words` extend the built-in lists, `allow` switches
# words or phrases off (FR-110, FR-118, FR-119). Example:
# [checks.weak-word]
# words = [{ w = "seamless", why = "marketing word; describe the behaviour" }]
# allow = ["Fast Track"]

[lsp]
# Characters of item text shown on hover (FR-205).
hover-length = 500
# Characters of the target title shown after links (FR-209).
hint-length = 40
# File for stubs created by code actions (FR-213); default: current file.
# stub-file = "inbox.md"
"#;

/// An item type, e.g. `functional` with prefix `FR`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemType {
    pub name: String,
    pub prefix: String,
    pub meta: Vec<String>,
    pub body: Vec<String>,
    /// Each entry is met if one of its alternatives is present (FR-105).
    pub required: Vec<Vec<String>>,
}

/// A link type, e.g. `impl`.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkType {
    pub key: String,
    pub name: Option<String>,
    /// Allowed target types; `None` allows any.
    pub to: Option<Vec<String>>,
}

/// Configuration of one check (FR-117).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CheckConfig {
    pub level: Option<Level>,
    /// Additional words of a language check (FR-110, FR-118).
    pub words: Vec<WordEntry>,
    /// Allowed words or phrases (FR-119).
    pub allow: Vec<String>,
}

/// A word or phrase of a language check with its reason (FR-110).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordEntry {
    pub word: String,
    pub why: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub include: Vec<String>,
    pub glossary: Option<String>,
    pub digits: usize,
    pub types: Vec<ItemType>,
    pub links: Vec<LinkType>,
    /// Metadata keys allowed on every type.
    pub common_meta: Vec<String>,
    pub values: BTreeMap<String, Vec<String>>,
    pub defaults: BTreeMap<String, String>,
    pub checks: BTreeMap<String, CheckConfig>,
    /// Characters of item text shown on hover (FR-205).
    pub hover_length: usize,
    /// Characters of an inlay hint (FR-209).
    pub hint_length: usize,
    /// File for new stubs, relative to the root (FR-213).
    pub stub_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub path: Option<PathBuf>,
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(p) => write!(f, "{}: {}", p.display(), self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for ConfigError {}

/// FR-013: the nearest directory, starting at `start`, that contains
/// `requimd.toml`. A file as start means its directory.
pub fn find_root(start: &Path) -> Option<PathBuf> {
    let start = std::path::absolute(start).ok()?;
    let dir = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start
    };
    dir.ancestors()
        .find(|d| d.join(CONFIG_FILE).is_file())
        .map(Path::to_path_buf)
}

impl Config {
    /// Built-in defaults (FR-014).
    pub fn defaults() -> Result<Config, ConfigError> {
        let mut raw = parse_raw(INIT_TEMPLATE, None)?;
        raw.project.glossary = None;
        raw.into_config(None)
    }

    /// Reads `requimd.toml` in `root` and merges it with the defaults.
    pub fn load(root: &Path) -> Result<Config, ConfigError> {
        let path = root.join(CONFIG_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| ConfigError {
            path: Some(path.clone()),
            message: e.to_string(),
        })?;
        Config::from_toml(&text, Some(&path))
    }

    /// Parses configuration text and merges it with the defaults (FR-014).
    pub fn from_toml(text: &str, path: Option<&Path>) -> Result<Config, ConfigError> {
        let mut base = parse_raw(INIT_TEMPLATE, None)?;
        base.project.glossary = None;
        let over = parse_raw(text, path)?;
        base.merge(over).into_config(path)
    }

    pub fn type_by_prefix(&self, prefix: &str) -> Option<&ItemType> {
        self.types.iter().find(|t| t.prefix == prefix)
    }

    pub fn type_by_name(&self, name: &str) -> Option<&ItemType> {
        self.types.iter().find(|t| t.name == name)
    }

    pub fn link(&self, key: &str) -> Option<&LinkType> {
        self.links.iter().find(|l| l.key == key)
    }

    /// Metadata and link keys allowed on items of type `ty` (FR-102).
    pub fn meta_keys<'a>(&'a self, ty: &'a ItemType) -> Vec<&'a str> {
        ty.meta
            .iter()
            .chain(&self.common_meta)
            .map(String::as_str)
            .chain(self.links.iter().map(|l| l.key.as_str()))
            .collect()
    }

    pub fn is_meta_key(&self, ty: &ItemType, key: &str) -> bool {
        self.meta_keys(ty).contains(&key)
    }

    /// Configured setting of a check, if any (FR-117).
    pub fn check_level(&self, id: &str) -> Option<Level> {
        self.checks.get(id).and_then(|c| c.level)
    }

    /// Built-in plus configured weak words (FR-110).
    pub fn weak_words(&self) -> Vec<WordEntry> {
        let mut words: Vec<WordEntry> = WEAK_WORDS
            .iter()
            .map(|(w, why)| WordEntry {
                word: w.to_string(),
                why: why.to_string(),
            })
            .collect();
        words.extend(
            self.checks
                .get("weak-word")
                .into_iter()
                .flat_map(|c| c.words.clone()),
        );
        words
    }

    /// Built-in plus configured modal verbs (FR-118).
    pub fn modal_verbs(&self) -> Vec<String> {
        let mut words: Vec<String> = MODAL_VERBS.iter().map(|w| w.to_string()).collect();
        words.extend(
            self.checks
                .get("modal-verb")
                .into_iter()
                .flat_map(|c| c.words.iter().map(|w| w.word.clone())),
        );
        words
    }

    /// Project-wide allow list of a check (FR-119).
    pub fn allow(&self, check: &str) -> &[String] {
        self.checks.get(check).map_or(&[], |c| c.allow.as_slice())
    }
}

fn parse_raw(text: &str, path: Option<&Path>) -> Result<Raw, ConfigError> {
    toml::from_str(text).map_err(|e| ConfigError {
        path: path.map(Path::to_path_buf),
        message: e.to_string().trim_end().to_string(),
    })
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Raw {
    #[serde(default)]
    project: RawProject,
    #[serde(default)]
    ids: RawIds,
    #[serde(default)]
    types: BTreeMap<String, RawType>,
    #[serde(default)]
    links: BTreeMap<String, RawLink>,
    #[serde(default)]
    fields: RawFields,
    #[serde(default)]
    values: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    defaults: BTreeMap<String, String>,
    #[serde(default)]
    checks: BTreeMap<String, toml::Value>,
    #[serde(default)]
    lsp: RawLsp,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct RawLsp {
    hover_length: Option<usize>,
    hint_length: Option<usize>,
    stub_file: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProject {
    include: Option<Vec<String>>,
    glossary: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawIds {
    digits: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawType {
    prefix: String,
    #[serde(default)]
    meta: Vec<String>,
    #[serde(default)]
    body: Vec<String>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLink {
    name: Option<String>,
    to: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFields {
    meta: Option<Vec<String>>,
}

impl Raw {
    /// FR-014: an entry of `over` replaces the entry of the same name.
    fn merge(mut self, over: Raw) -> Raw {
        if over.project.include.is_some() {
            self.project.include = over.project.include;
        }
        if over.project.glossary.is_some() {
            self.project.glossary = over.project.glossary;
        }
        if over.ids.digits.is_some() {
            self.ids.digits = over.ids.digits;
        }
        if over.fields.meta.is_some() {
            self.fields.meta = over.fields.meta;
        }
        if over.lsp.hover_length.is_some() {
            self.lsp.hover_length = over.lsp.hover_length;
        }
        if over.lsp.hint_length.is_some() {
            self.lsp.hint_length = over.lsp.hint_length;
        }
        if over.lsp.stub_file.is_some() {
            self.lsp.stub_file = over.lsp.stub_file;
        }
        self.types.extend(over.types);
        self.links.extend(over.links);
        self.values.extend(over.values);
        self.defaults.extend(over.defaults);
        self.checks.extend(over.checks);
        self
    }

    fn into_config(self, path: Option<&Path>) -> Result<Config, ConfigError> {
        let err = |message: String| ConfigError {
            path: path.map(Path::to_path_buf),
            message,
        };

        let digits = self.ids.digits.unwrap_or(3);
        if digits == 0 {
            return Err(err("[ids] digits must be at least 1".into()));
        }

        let mut types: Vec<ItemType> = Vec::new();
        for (name, t) in self.types {
            if t.prefix.is_empty() || !t.prefix.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(err(format!(
                    "[types.{name}] prefix \"{}\" must consist of letters and digits",
                    t.prefix
                )));
            }
            if let Some(other) = types.iter().find(|o| o.prefix == t.prefix) {
                return Err(err(format!(
                    "prefix \"{}\" is used by [types.{}] and [types.{name}]",
                    t.prefix, other.name
                )));
            }
            types.push(ItemType {
                name,
                prefix: t.prefix,
                meta: t.meta,
                body: t.body,
                required: t
                    .required
                    .iter()
                    .map(|r| r.split('|').map(|k| k.trim().to_string()).collect())
                    .collect(),
            });
        }

        let links: Vec<LinkType> = self
            .links
            .into_iter()
            .map(|(key, l)| LinkType {
                key,
                name: l.name,
                to: l.to,
            })
            .collect();
        for l in &links {
            for t in l.to.iter().flatten() {
                if t != STEP && !types.iter().any(|ty| &ty.name == t) {
                    return Err(err(format!(
                        "[links.{}] target type \"{t}\" is not a configured type",
                        l.key
                    )));
                }
            }
        }

        let common_meta = self.fields.meta.unwrap_or_default();
        for ty in &types {
            for key in ty.required.iter().flatten() {
                let known = ty.meta.contains(key)
                    || ty.body.contains(key)
                    || common_meta.contains(key)
                    || links.iter().any(|l| &l.key == key);
                if !known {
                    return Err(err(format!(
                        "[types.{}] requires \"{key}\", which is not a key of this type",
                        ty.name
                    )));
                }
            }
        }

        let include = self.project.include.unwrap_or_default();
        for pattern in &include {
            if let Err(e) = globset::Glob::new(pattern) {
                return Err(err(format!("include pattern \"{pattern}\": {e}")));
            }
        }

        let mut checks = BTreeMap::new();
        for (id, value) in self.checks {
            if !CHECK_IDS.contains(&id.as_str()) {
                return Err(err(format!(
                    "[checks] unknown check \"{id}\"; known checks: {}",
                    CHECK_IDS.join(", ")
                )));
            }
            let parse_level = |v: &toml::Value| -> Result<Level, ConfigError> {
                v.as_str().and_then(Level::parse).ok_or_else(|| {
                    err(format!(
                        "[checks] {id}: severity must be \"error\", \"warning\", \"info\" or \"off\""
                    ))
                })
            };
            let check = match value {
                toml::Value::Table(mut options) => {
                    let level = match options.remove("sev") {
                        Some(v) => Some(parse_level(&v)?),
                        None => None,
                    };
                    let language = ALLOW_CHECKS.contains(&id.as_str());
                    let words = match options.remove("words") {
                        Some(v) if language && id != "glossary-alias" => {
                            parse_words(&v).map_err(|m| err(format!("[checks.{id}] words: {m}")))?
                        }
                        Some(_) => {
                            return Err(err(format!("[checks.{id}] has no option \"words\"")));
                        }
                        None => Vec::new(),
                    };
                    let allow = match options.remove("allow") {
                        Some(v) if language => parse_strings(&v).ok_or_else(|| {
                            err(format!("[checks.{id}] allow must be a list of strings"))
                        })?,
                        Some(_) => {
                            return Err(err(format!("[checks.{id}] has no option \"allow\"")));
                        }
                        None => Vec::new(),
                    };
                    if let Some(key) = options.keys().next() {
                        return Err(err(format!("[checks.{id}] has no option \"{key}\"")));
                    }
                    CheckConfig {
                        level,
                        words,
                        allow,
                    }
                }
                v => CheckConfig {
                    level: Some(parse_level(&v)?),
                    ..CheckConfig::default()
                },
            };
            checks.insert(id, check);
        }
        if self.lsp.hint_length == Some(0) || self.lsp.hover_length == Some(0) {
            return Err(err("[lsp] lengths must be at least 1".into()));
        }

        Ok(Config {
            include,
            glossary: self.project.glossary,
            digits,
            types,
            links,
            common_meta,
            values: self.values,
            defaults: self.defaults,
            checks,
            hover_length: self.lsp.hover_length.unwrap_or(500),
            hint_length: self.lsp.hint_length.unwrap_or(40),
            stub_file: self.lsp.stub_file,
        })
    }
}

fn parse_strings(v: &toml::Value) -> Option<Vec<String>> {
    v.as_array()?
        .iter()
        .map(|s| s.as_str().map(str::to_string))
        .collect()
}

/// `words` entries: `"word"` or `{ w = "word", why = "reason" }`.
fn parse_words(v: &toml::Value) -> Result<Vec<WordEntry>, String> {
    let Some(list) = v.as_array() else {
        return Err("must be a list".into());
    };
    list.iter()
        .map(|e| match e {
            toml::Value::String(w) => Ok(WordEntry {
                word: w.clone(),
                why: String::new(),
            }),
            toml::Value::Table(t) => {
                if let Some(k) = t.keys().find(|k| *k != "w" && *k != "why") {
                    return Err(format!("unknown key \"{k}\""));
                }
                let word = t
                    .get("w")
                    .and_then(toml::Value::as_str)
                    .ok_or("each entry needs a string \"w\"")?;
                let why = match t.get("why") {
                    None => "",
                    Some(w) => w.as_str().ok_or("\"why\" must be a string")?,
                };
                Ok(WordEntry {
                    word: word.to_string(),
                    why: why.to_string(),
                })
            }
            _ => Err("entries must be strings or { w, why } tables".into()),
        })
        .filter(|r| r.as_ref().map_or(true, |w| !w.word.trim().is_empty()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::Severity;

    #[test]
    fn fr_014_defaults_contain_types_links_and_values() {
        let c = Config::defaults().unwrap();
        let prefixes: Vec<&str> = c.types.iter().map(|t| t.prefix.as_str()).collect();
        for p in ["UC", "FR", "NFR", "BR", "US", "TC"] {
            assert!(prefixes.contains(&p), "missing {p}");
        }
        let keys: Vec<&str> = c.links.iter().map(|l| l.key.as_str()).collect();
        assert_eq!(keys, ["impl", "ref", "ver"]);
        assert_eq!(c.common_meta, ["status", "prio"]);
        assert_eq!(c.values["prio"], ["must", "should", "could", "wont"]);
        assert_eq!(c.include, ["**/*.md"]);
        assert_eq!(c.glossary, None);
        assert_eq!(c.digits, 3);
        assert_eq!(
            c.type_by_prefix("UC").unwrap().body,
            ["pre", "post", "trigger"]
        );
    }

    #[test]
    fn fr_014_defaults_match_reference_config() {
        // The reference requimd.toml adds only project-specific settings.
        let text = include_str!("../../../requimd.toml");
        let mut reference = Config::from_toml(text, None).unwrap();
        let defaults = Config::defaults().unwrap();
        for t in &mut reference.types {
            if t.name == "functional" {
                t.meta.clear();
            }
        }
        reference.values.remove("sev");
        assert_eq!(reference.types, defaults.types);
        assert_eq!(reference.links, defaults.links);
        assert_eq!(reference.values, defaults.values);
        assert_eq!(reference.common_meta, defaults.common_meta);
        assert_eq!(reference.defaults, defaults.defaults);
    }

    #[test]
    fn fr_014_project_entry_replaces_default_entry() {
        let c = Config::from_toml(
            "[types.quality]\nprefix = \"NFR\"\nrequired = [\"scale\"]\nmeta = [\"scale\"]\n",
            None,
        )
        .unwrap();
        let q = c.type_by_name("quality").unwrap();
        assert_eq!(q.meta, ["scale"]);
        assert_eq!(q.required, [vec!["scale".to_string()]]);
        // Other default types stay.
        assert!(c.type_by_prefix("UC").is_some());
    }

    #[test]
    fn fr_013_reads_all_sections() {
        let c = Config::from_toml(
            r#"
[project]
include = ["req/**/*.md"]
glossary = "req/glossary.md"
[ids]
digits = 4
[types.epic]
prefix = "EP"
meta = ["owner"]
[links.dep]
to = ["epic"]
[values]
prio = ["high", "low"]
[defaults]
prio = "low"
[checks]
uncovered-step = "info"
[checks.weak-word]
sev = "off"
allow = ["Fast Track"]
"#,
            None,
        )
        .unwrap();
        assert_eq!(c.include, ["req/**/*.md"]);
        assert_eq!(c.glossary.as_deref(), Some("req/glossary.md"));
        assert_eq!(c.digits, 4);
        assert_eq!(c.type_by_prefix("EP").unwrap().meta, ["owner"]);
        assert_eq!(
            c.link("dep").unwrap().to.as_deref(),
            Some(&["epic".to_string()][..])
        );
        assert_eq!(c.values["prio"], ["high", "low"]);
        assert_eq!(c.defaults["prio"], "low");
        assert_eq!(
            c.check_level("uncovered-step"),
            Some(Level::On(Severity::Info))
        );
        assert_eq!(c.check_level("weak-word"), Some(Level::Off));
        assert_eq!(c.allow("weak-word"), ["Fast Track"]);
        assert_eq!(c.hover_length, 500);
        assert_eq!(c.hint_length, 40);
        assert_eq!(c.stub_file, None);
    }

    #[test]
    fn fr_110_configured_words_extend_built_in_list() {
        let c = Config::from_toml(
            "[checks.weak-word]\nwords = [{ w = \"seamless\", why = \"marketing\" }, \"nice\"]\n\
             [checks.modal-verb]\nwords = [\"must\"]\n",
            None,
        )
        .unwrap();
        let words = c.weak_words();
        assert!(words.iter().any(|w| w.word == "fast" && !w.why.is_empty()));
        assert!(
            words
                .iter()
                .any(|w| w.word == "seamless" && w.why == "marketing")
        );
        assert!(words.iter().any(|w| w.word == "nice" && w.why.is_empty()));
        for required in [
            "fast",
            "quick",
            "quickly",
            "easy",
            "user-friendly",
            "intuitive",
            "flexible",
            "robust",
            "appropriate",
            "adequate",
            "several",
            "etc.",
            "and/or",
            "as needed",
            "if possible",
        ] {
            assert!(words.iter().any(|w| w.word == required), "{required}");
        }
        assert!(WEAK_WORDS.iter().all(|(_, why)| !why.is_empty()));
        assert_eq!(
            c.modal_verbs(),
            ["should", "may", "can", "could", "might", "will", "must"]
        );
    }

    #[test]
    fn fr_119_invalid_language_options_are_config_errors() {
        for bad in [
            "[checks.weak-word]\nwords = \"fast\"\n",
            "[checks.weak-word]\nwords = [{ why = \"x\" }]\n",
            "[checks.weak-word]\nwords = [{ w = \"x\", reason = \"y\" }]\n",
            "[checks.weak-word]\nallow = [1]\n",
            "[checks.weak-word]\ncolour = 1\n",
            "[checks.glossary-alias]\nwords = [\"x\"]\n",
            "[checks.placeholder]\nallow = [\"x\"]\n",
            "[lsp]\nhint-length = 0\n",
        ] {
            assert!(Config::from_toml(bad, None).is_err(), "{bad}");
        }
    }

    #[test]
    fn fr_213_stub_file_and_fr_209_hint_length() {
        let c =
            Config::from_toml("[lsp]\nstub-file = \"inbox.md\"\nhint-length = 20\n", None).unwrap();
        assert_eq!(c.stub_file.as_deref(), Some("inbox.md"));
        assert_eq!(c.hint_length, 20);
    }

    #[test]
    fn fr_205_hover_length_configurable() {
        let c = Config::from_toml("[lsp]\nhover-length = 80\n", None).unwrap();
        assert_eq!(c.hover_length, 80);
        assert!(Config::from_toml("[lsp]\nhover_length = 80\n", None).is_err());
    }

    #[test]
    fn fr_117_invalid_check_settings_are_config_errors() {
        assert!(Config::from_toml("[checks]\nno-such-check = \"info\"\n", None).is_err());
        assert!(Config::from_toml("[checks]\nplaceholder = \"loud\"\n", None).is_err());
        assert!(Config::from_toml("[checks.placeholder]\nsev = 3\n", None).is_err());
    }

    #[test]
    fn fr_013_invalid_configs_are_errors() {
        assert!(Config::from_toml("[types.x]\nprefix = \"FR\"\n", None).is_err());
        assert!(Config::from_toml("[links.x]\nto = [\"nothing\"]\n", None).is_err());
        assert!(Config::from_toml("[project]\ninclud = []\n", None).is_err());
        assert!(
            Config::from_toml("[types.x]\nprefix = \"X\"\nrequired = [\"y\"]\n", None).is_err()
        );
        assert!(Config::from_toml("not toml at all [", None).is_err());
    }

    #[test]
    fn fr_013_finds_root_in_parent_directory() {
        let shop = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/shop");
        let root = find_root(&shop.join("usecases.md")).unwrap();
        assert!(root.join(CONFIG_FILE).is_file());
        assert!(root.ends_with("shop"));
    }

    #[test]
    fn fr_303_init_template_parses_to_defaults_plus_glossary() {
        let mut c = Config::from_toml(INIT_TEMPLATE, None).unwrap();
        assert_eq!(c.glossary.as_deref(), Some("glossary.md"));
        c.glossary = None;
        assert_eq!(c, Config::defaults().unwrap());
    }
}
