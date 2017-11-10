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


use clap::{AppSettings, Arg};


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Repository slug to export.
    pub slug: String,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<REPO_SLUG> 'Repository slug to export'").validator(Options::slug_validator).required(true))
            .get_matches();

        Options { slug: matches.value_of("REPO_SLUG").unwrap().to_string() }
    }

    // TODO: extract this to a `RepoSlug` struct
    fn slug_validator(s: String) -> Result<(), String> {
        if s.chars().filter(|&c| c == '/').count() == 1 {
            Ok(())
        } else {
            Err(format!("Slug string \"{}\" not actualy a slug", s))
        }
    }
}
