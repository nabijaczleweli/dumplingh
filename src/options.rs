//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use dumplingh::Options;
//! let options = Options::parse();
//! println!("Exporting {}", options.slug);
//! ```


use clap::{self, AppSettings, Arg};
use self::super::ops::RepoSlug;
use std::path::PathBuf;
use std::str;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Repository slug to export.
    pub slug: RepoSlug,
    /// File to write issues to, if any.
    ///
    /// Default: `"./<slug>-issues.json"`.
    pub out_issues: Option<(String, PathBuf)>,
    /// File to write pull requests to, if any.
    ///
    /// Default: `"./<slug>-pulls.json"`.
    pub out_pull_requests: Option<(String, PathBuf)>,
    /// File to write labels to, if any.
    ///
    /// Default: `"./<slug>-labels.json"`.
    pub out_labels: Option<(String, PathBuf)>,
    /// Whether to compact-print, as opposed to pretty-print, exported JSON.
    ///
    /// Default: `false`.
    pub compact: bool,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<REPO_SLUG> 'Repository slug to export'").validator(Options::slug_validator).required(true))
            .arg(Arg::from_usage("-i --issues [ISSUES_FILE] 'File to write issues to. Default: <slug>-issues.json'")
                .validator(|s| Options::out_file_validator(s, "Issues"))
                .conflicts_with("no-issues"))
            .arg(Arg::from_usage("--no-issues 'Don't export issues'").conflicts_with("issues"))
            .arg(Arg::from_usage("-p --pulls [PULLS_FILE] 'File to write pull requests to. Default: <slug>-pulls.json'")
                .validator(|s| Options::out_file_validator(s, "Pulls"))
                .conflicts_with("no-pulls"))
            .arg(Arg::from_usage("--no-pulls 'Don't export pulls'").conflicts_with("pulls"))
            .arg(Arg::from_usage("-l --labels [LABELS_FILE] 'File to write pull requests to. Default: <slug>-labels.json'")
                .validator(|s| Options::out_file_validator(s, "Labels"))
                .conflicts_with("no-labels"))
            .arg(Arg::from_usage("--no-labels 'Don't export labels'").conflicts_with("labels"))
            .arg(Arg::from_usage("-f --force 'Overwrite existing files'"))
            .arg(Arg::from_usage("-c --compact 'Don't pretty-print exported JSON'"))
            .get_matches();

        let force = matches.is_present("force");
        let slug: RepoSlug = matches.value_of("REPO_SLUG").unwrap().parse().unwrap();
        let slug_prefix = slug.filename();
        Options {
            slug: slug,
            out_issues: Options::out_file(force, &slug_prefix, "issues", matches.is_present("no-issues"), matches.value_of("issues")),
            out_pull_requests: Options::out_file(force, &slug_prefix, "pulls", matches.is_present("no-pulls"), matches.value_of("pulls")),
            out_labels: Options::out_file(force, &slug_prefix, "labels", matches.is_present("no-labels"), matches.value_of("labels")),
            compact: matches.is_present("compact"),
        }
    }

    fn out_file(force: bool, slug_prefix: &str, arg: &str, no_file: bool, file_val: Option<&str>) -> Option<(String, PathBuf)> {
        if !no_file {
            let arg_s = file_val.map(String::from).unwrap_or_else(|| format!("./{}-{}.json", slug_prefix, arg));
            let mut arg = PathBuf::from(&arg_s);
            if !force && arg.exists() {
                clap::Error {
                        message: format!("{} exists and --force not specified", arg_s),
                        kind: clap::ErrorKind::ArgumentConflict,
                        info: None,
                    }
                    .exit()
            } else {
                let fname = arg.file_name().unwrap().to_os_string();
                arg.pop();
                let mut arg_can = fs::canonicalize(&arg).unwrap();
                arg_can.push(fname);

                Some((arg_s, arg_can))
            }
        } else {
            None
        }
    }

    fn slug_validator(s: String) -> Result<(), String> {
        s.parse::<RepoSlug>().map(|_| ()).map_err(|e| {
            let mut err = vec![];
            e.print_error(&mut err);
            let err = str::from_utf8(&err).unwrap();
            format!("{}; from string \"{}\"", &err[..err.len() - 2], s)
        })
    }

    fn out_file_validator(s: String, desc: &str) -> Result<(), String> {
        let mut p = PathBuf::from(&s);
        p.pop();
        fs::canonicalize(&p).map_err(|_| format!("{}'s parent directory \"{}\" nonexistant", desc, p.display())).and_then(|f| if !f.is_file() {
            Ok(())
        } else {
            Err(format!("{}'s parent directory \"{}\" actually a file", desc, p.display()))
        })
    }
}
