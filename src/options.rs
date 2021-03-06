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
    /// File to write milestones to, if any.
    ///
    /// Default: `"./<slug>-milestones.json"`.
    pub out_milestones: Option<(String, PathBuf)>,
    /// File to write projects to, if any.
    ///
    /// Default: `"./<slug>-projects.json"`.
    pub out_projects: Option<(String, PathBuf)>,
    /// Directory to write comments to, if any.
    ///
    /// Default: `"./<slug>-comments/"`.
    pub out_comments: Option<(String, PathBuf)>,
    /// Whether to compact-print, as opposed to pretty-print, exported JSON.
    ///
    /// Default: `false`.
    pub compact: bool,
    /// GitHub OAuth2 token.
    ///
    /// Required for: projects.
    ///
    /// Default: `None`.
    pub github_token: Option<String>,
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
            .arg(Arg::from_usage("-p --pulls [PULLS_FILE] 'File to write pull requests to. Default: <slug>-pulls.json'")
                .validator(|s| Options::out_file_validator(s, "Pulls"))
                .conflicts_with("no-pulls"))
            .arg(Arg::from_usage("-l --labels [LABELS_FILE] 'File to write labels to. Default: <slug>-labels.json'")
                .validator(|s| Options::out_file_validator(s, "Labels"))
                .conflicts_with("no-labels"))
            .arg(Arg::from_usage("-m --milestones [MILESTONES_FILE] 'File to write milestones to. Default: <slug>-milestones.json'")
                .validator(|s| Options::out_file_validator(s, "Milestones"))
                .conflicts_with("no-milestones"))
            .arg(Arg::from_usage("--projects [PROJECTS_FILE] 'File to write projects to. Default: <slug>-projects.json'")
                .validator(|s| Options::out_file_validator(s, "Projects"))
                .conflicts_with("no-projects")
                .requires("auth"))
            .arg(Arg::from_usage("--comments [COMMENTS_DIR] 'Directory to write comments to. Default: <slug>-comments/'")
                .validator(|s| Options::out_file_validator(s, "comments"))
                .conflicts_with("no-comments"))
            .arg(Arg::from_usage("--no-issues 'Don't export issues'").conflicts_with("issues"))
            .arg(Arg::from_usage("--no-pulls 'Don't export pulls'").conflicts_with("pulls"))
            .arg(Arg::from_usage("--no-labels 'Don't export labels'").conflicts_with("labels"))
            .arg(Arg::from_usage("--no-milestones 'Don't export milestones'").conflicts_with("milestones"))
            .arg(Arg::from_usage("--no-projects 'Don't export projects'").conflicts_with("projects"))
            .arg(Arg::from_usage("--no-comments 'Don't export comments'").conflicts_with("comments"))
            .arg(Arg::from_usage("-f --force 'Overwrite existing files'"))
            .arg(Arg::from_usage("-c --compact 'Don't pretty-print exported JSON'"))
            .arg(Arg::from_usage("-a --auth [AUTH_TOKEN] 'GitHub OAuth2 token. Required for projects'"))
            .get_matches();

        let force = matches.is_present("force");
        let slug: RepoSlug = matches.value_of("REPO_SLUG").unwrap().parse().unwrap();
        let slug_prefix = slug.filename();
        let token = matches.value_of("auth").map(String::from);
        Options {
            slug: slug,
            out_issues: Options::out_file(force,
                                          &slug_prefix,
                                          ".json",
                                          "issues",
                                          matches.is_present("no-issues"),
                                          matches.value_of("issues")),
            out_pull_requests: Options::out_file(force, &slug_prefix, ".json", "pulls", matches.is_present("no-pulls"), matches.value_of("pulls")),
            out_labels: Options::out_file(force,
                                          &slug_prefix,
                                          ".json",
                                          "labels",
                                          matches.is_present("no-labels"),
                                          matches.value_of("labels")),
            out_milestones: Options::out_file(force,
                                              &slug_prefix,
                                              ".json",
                                              "milestones",
                                              matches.is_present("no-milestones"),
                                              matches.value_of("milestones")),
            out_projects: if token.is_some() {
                Options::out_file(force,
                                  &slug_prefix,
                                  ".json",
                                  "projects",
                                  matches.is_present("no-projects"),
                                  matches.value_of("projects"))
            } else {
                None
            },
            out_comments: Options::out_file(force,
                                            &slug_prefix,
                                            "",
                                            "comments",
                                            matches.is_present("no-comments"),
                                            matches.value_of("comments")),
            compact: matches.is_present("compact"),
            github_token: token,
        }
    }

    fn out_file(force: bool, slug_prefix: &str, arg: &str, ext: &str, no_file: bool, file_val: Option<&str>) -> Option<(String, PathBuf)> {
        if !no_file {
            let arg_s = file_val.map(String::from).unwrap_or_else(|| format!("./{}-{}{}", slug_prefix, ext, arg));
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
                let mut arg_can = fs::canonicalize(if !arg.is_absolute() {
                        PathBuf::from(".")
                    } else {
                        arg
                    })
                    .unwrap();
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
        if !p.is_absolute() {
            p = PathBuf::from(format!("./{}", s));
        }
        if p.parent().is_some() {
            p.pop();
            fs::canonicalize(&p).map_err(|_| format!("{}'s parent directory \"{}\" nonexistant", desc, p.display())).and_then(|f| if !f.is_file() {
                Ok(())
            } else {
                Err(format!("{}'s parent directory \"{}\" actually a file", desc, p.display()))
            })
        } else {
            Ok(())
        }
    }
}
