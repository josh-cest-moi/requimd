//! The `requimd` binary (FR-300 ff.).

mod commands;
mod export;

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use requimd_core::config::{CONFIG_FILE, INIT_TEMPLATE};
use requimd_core::lock::{LOCK_FILE, Lock};
use requimd_core::{Config, Diagnostic, Project, Severity, find_root};
use serde::Serialize;

#[derive(Parser)]
#[command(name = "requimd", version, about = "Requirements in Markdown")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check the project and print one line per diagnostic.
    Check {
        /// Only print diagnostics for these files or directories.
        paths: Vec<PathBuf>,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// Treat warnings as errors for the exit code.
        #[arg(long)]
        strict: bool,
        /// Do not write requimd.lock; fail if it is out of date (for CI).
        #[arg(long)]
        locked: bool,
    },
    /// Create requimd.toml and an empty glossary in the current directory.
    Init,
    /// Assign IDs to placeholders and normalise metadata separators.
    Fmt {
        /// Change nothing; exit with 1 if files would change.
        #[arg(long)]
        check: bool,
    },
    /// Print an item or step with its incoming and outgoing references.
    Show {
        /// Item ID or step reference, e.g. FR-104 or UC-012.s2.
        id: String,
    },
    /// Print the coverage of use case steps by `impl` links.
    Coverage,
    /// Mark suspect links from or to an item as reviewed.
    Review {
        /// Item ID or step reference.
        #[arg(required_unless_present = "all", conflicts_with = "all")]
        id: Option<String>,
        /// Mark every suspect link of the project as reviewed.
        #[arg(long)]
        all: bool,
    },
    /// Export the project as static site, traceability matrix or JSON.
    Export {
        #[command(subcommand)]
        what: ExportCommand,
    },
    /// Start the language server on stdin and stdout.
    Lsp,
}

#[derive(Subcommand)]
enum ExportCommand {
    /// Static website for reading without git or an editor (FR-400).
    Site {
        /// Output directory.
        dir: PathBuf,
        /// Only items with these statuses, e.g. review,approved.
        #[arg(long, value_delimiter = ',')]
        status: Vec<String>,
    },
    /// Traceability matrix as CSV (FR-401).
    Matrix {
        /// Write to this file instead of standard output.
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
    /// The item graph as JSON (FR-402).
    Json {
        /// Write to this file instead of standard output.
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// FR-300: exit codes.
const OK: u8 = 0;
const FOUND: u8 = 1;
const USAGE: u8 = 2;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let code = match cli.command {
        Command::Check {
            paths,
            format,
            strict,
            locked,
        } => check(&paths, format, strict, locked),
        Command::Init => init(),
        // FR-304
        Command::Fmt { check } => with_project(|p, cwd| match commands::fmt(p, check, cwd) {
            Ok((out, changed)) => {
                print!("{out}");
                if check && changed { FOUND } else { OK }
            }
            Err(e) => fail(&e),
        }),
        // FR-305
        Command::Show { id } => with_project(|p, cwd| match commands::show(p, &id, cwd) {
            Ok(out) => {
                print!("{out}");
                OK
            }
            Err((code, msg)) => {
                eprintln!("requimd: {msg}");
                code
            }
        }),
        // FR-306
        Command::Coverage => with_project(|p, _| {
            print!("{}", commands::coverage(p));
            OK
        }),
        // FR-307
        Command::Review { id, all } => {
            with_project(|p, _| match commands::review(p, id.as_deref(), all) {
                Ok(out) => {
                    print!("{out}");
                    OK
                }
                Err((code, msg)) => {
                    eprintln!("requimd: {msg}");
                    code
                }
            })
        }
        // FR-400 to FR-402
        Command::Export { what } => with_project(|p, _| export(p, what)),
        // FR-308
        Command::Lsp => match requimd_lsp::run_stdio() {
            Ok(()) => OK,
            Err(e) => fail(&format!("language server: {e}")),
        },
    };
    ExitCode::from(code)
}

/// FR-400 to FR-402.
fn export(p: &Project, what: ExportCommand) -> u8 {
    let (output, text) = match what {
        ExportCommand::Site { dir, status } => {
            if let Some(allowed) = p.config.values.get("status")
                && let Some(bad) = status.iter().find(|s| !allowed.contains(s))
            {
                return fail(&format!(
                    "unknown status \"{bad}\"; allowed: {}",
                    allowed.join(", ")
                ));
            }
            let filter = export::site::Filter((!status.is_empty()).then_some(status));
            return match export::site::write_site(p, filter, &dir) {
                Ok(n) => {
                    println!("wrote {n} pages to {}", dir.display());
                    OK
                }
                Err(e) => fail(&e),
            };
        }
        ExportCommand::Matrix { output } => (output, export::matrix::csv(p)),
        ExportCommand::Json { output } => (output, export::json::json(p)),
    };
    match output {
        Some(path) => match std::fs::write(&path, text) {
            Ok(()) => OK,
            Err(e) => fail(&format!("{}: {e}", path.display())),
        },
        None => {
            print!("{text}");
            OK
        }
    }
}

/// Loads the project around the working directory and runs `f` on it.
fn with_project(f: impl FnOnce(&Project, &Path) -> u8) -> u8 {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => return fail(&format!("current directory: {e}")),
    };
    let Some(root) = find_root(&cwd) else {
        return fail(&format!(
            "no {CONFIG_FILE} found in {} or its parents; run `requimd init`",
            cwd.display()
        ));
    };
    let project = match Config::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|c| Project::load(&root, c).map_err(|e| e.to_string()))
    {
        Ok(p) => p,
        Err(e) => return fail(&e),
    };
    f(&project, &cwd)
}

/// FR-300 to FR-302.
fn check(paths: &[PathBuf], format: Format, strict: bool, locked: bool) -> u8 {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => return fail(&format!("current directory: {e}")),
    };
    let mut filters = Vec::new();
    for p in paths {
        if !p.exists() {
            return fail(&format!("{}: no such file or directory", p.display()));
        }
        match std::path::absolute(p) {
            Ok(a) => filters.push(a),
            Err(e) => return fail(&format!("{}: {e}", p.display())),
        }
    }
    let start = filters.first().cloned().unwrap_or_else(|| cwd.clone());
    let Some(root) = find_root(&start) else {
        return fail(&format!(
            "no {CONFIG_FILE} found in {} or its parents; run `requimd init`",
            start.display()
        ));
    };
    for f in &filters {
        if find_root(f).as_deref() != Some(root.as_path()) {
            return fail(&format!(
                "{} does not belong to the project at {}",
                f.display(),
                root.display()
            ));
        }
    }
    let config = match Config::load(&root) {
        Ok(c) => c,
        Err(e) => return fail(&e.to_string()),
    };
    let project = match Project::load(&root, config) {
        Ok(p) => p,
        Err(e) => return fail(&e.to_string()),
    };

    let diags: Vec<Diagnostic> = requimd_checks::check(&project)
        .into_iter()
        .filter(|d| {
            filters.is_empty()
                || project
                    .files
                    .get(d.file)
                    .is_some_and(|f| filters.iter().any(|p| f.path.starts_with(p)))
        })
        .collect();

    let out = render(&project, &diags, &cwd, format);
    if let Err(e) = io::stdout().lock().write_all(out.as_bytes()) {
        // A closed pipe is not worth a message.
        if e.kind() != io::ErrorKind::BrokenPipe {
            return fail(&e.to_string());
        }
    }

    // FR-116, FR-300: record new links and IDs, or with --locked verify.
    let updated = project.lock.updated(&project);
    let lock_path = project.root.join(LOCK_FILE);
    let mut stale = false;
    if updated != project.lock || (!lock_path.exists() && updated != Lock::default()) {
        if locked {
            eprintln!(
                "requimd: {LOCK_FILE} is out of date; run `requimd check` and commit {LOCK_FILE}"
            );
            stale = true;
        } else if let Err(e) = std::fs::write(&lock_path, updated.to_toml()) {
            return fail(&format!("{}: {e}", lock_path.display()));
        }
    }

    let failing = diags
        .iter()
        .any(|d| d.severity == Severity::Error || (strict && d.severity == Severity::Warning));
    if failing || stale { FOUND } else { OK }
}

#[derive(Serialize)]
struct JsonDiagnostic<'a> {
    file: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    severity: &'static str,
    check: &'static str,
    message: &'a str,
}

/// FR-300: `file:line:column: severity: message [check-id]`, or a JSON array
/// for `--format json` (FR-301).
fn render(project: &Project, diags: &[Diagnostic], cwd: &Path, format: Format) -> String {
    let rows: Vec<JsonDiagnostic> = diags
        .iter()
        .filter_map(|d| {
            let f = project.files.get(d.file)?;
            let (line, column) = f.line_col(d.span.start);
            let (end_line, end_column) = f.line_col(d.span.end);
            Some(JsonDiagnostic {
                file: display_path(&f.path, cwd),
                line,
                column,
                end_line,
                end_column,
                severity: d.severity.as_str(),
                check: d.check,
                message: &d.message,
            })
        })
        .collect();
    match format {
        Format::Text => rows
            .iter()
            .map(|r| {
                format!(
                    "{}:{}:{}: {}: {} [{}]\n",
                    r.file, r.line, r.column, r.severity, r.message, r.check
                )
            })
            .collect(),
        Format::Json => match serde_json::to_string_pretty(&rows) {
            Ok(s) => s + "\n",
            Err(_) => "[]\n".into(),
        },
    }
}

/// Path relative to the working directory if below it, with `/` as
/// separator on every platform (NFR-004).
fn display_path(path: &Path, cwd: &Path) -> String {
    let shown = path.strip_prefix(cwd).unwrap_or(path);
    shown.to_string_lossy().replace('\\', "/")
}

/// FR-303: creates the configuration and an empty glossary; never
/// overwrites a file.
fn init() -> u8 {
    let glossary = "glossary.md";
    for (name, content) in [(CONFIG_FILE, INIT_TEMPLATE), (glossary, "")] {
        match OpenOptions::new().write(true).create_new(true).open(name) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(content.as_bytes()) {
                    return fail(&format!("{name}: {e}"));
                }
                println!("created {name}");
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                println!("{name} exists, left unchanged");
            }
            Err(e) => return fail(&format!("{name}: {e}")),
        }
    }
    OK
}

fn fail(message: &str) -> u8 {
    eprintln!("requimd: {message}");
    USAGE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfr_004_output_paths_use_forward_slashes() {
        let cwd = Path::new("project");
        let path = Path::new("project").join("requirements").join("a.md");
        assert_eq!(display_path(&path, cwd), "requirements/a.md");
    }
}
