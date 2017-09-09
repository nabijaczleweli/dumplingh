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


use self::super::ops::RepoSlug;
use clap::{AppSettings, Arg};
use std::str;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Repository slug to export.
    pub slug: RepoSlug,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<REPO_SLUG> 'Repository slug to export'").validator(Options::slug_validator).required(true))
            .get_matches();

        Options { slug: matches.value_of("REPO_SLUG").unwrap().parse().unwrap() }
    }

    fn slug_validator(s: String) -> Result<(), String> {
        s.parse::<RepoSlug>().map(|_| ()).map_err(|e| {
            let mut err = vec![];
            e.print_error(&mut err);
            let err = str::from_utf8(&err).unwrap();
            format!("{}; from string \"{}\"", &err[..err.len() - 2], s)
        })
    }
}
