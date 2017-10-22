//! Issue/PR exporter for GitHub.
//!
//! # dumplingh as а library
//!
//! ## Data flow
//!
//! ```plaintext
//! Options::parse()
//! |> list_{pull_requests,issues}()
//! |> save_{to_file,data}()
//! ```
//!
//! ## Example
//!
//! ```
//! # use dumplingh::ops::{save_to_file, list_pull_requests};
//! # use std::env::temp_dir;
//! # use std::fs;
//! # /*
//! let out_path = "pulls.json";
//! # */
//! # let mut out_path = temp_dir().join("dumplingh-doctest");
//! # fs::create_dir_all(&out_path).unwrap();
//! # out_path.push("pulls.json");
//! let repo = "nabijaczleweli/cargo-update".parse().unwrap();
//! let pulls = list_pull_requests(&repo, None).unwrap();
//! save_to_file(out_path, &pulls, false, "pull requests").unwrap();
//! ```
//!
//! # dumplingh as аn executable
//!
//! This is just a very short synopsis of
//! [the manpage](https://cdn.rawgit.com/nabijaczleweli/dumplingh/man/dumplingh.1.html),
//! so consult that for more data.
//!
//! ## OPTIONS
//!
//! | Option                               | Description                                                              |
//! |--------------------------------------|--------------------------------------------------------------------------|
//! | &lt;REPO_SLUG&gt;                    | Repository to export issues and PRs for in the form `<username>/<repo>`. |
//! | --issues &lt;ISSUES_FILE&gt;         | File to export issues to, or `./<slug>-issues.json` by default.          |
//! | --pulls  &lt;PULLS_FILE&gt;          | File to export pull requests to, or `./<slug>-pulls.json` by default.    |
//! | --labels &lt;LABELS_FILE&gt;         | File to export labels to, or `./<slug>-labels.json` by default.          |
//! | --milestones &lt;MILESTONES_FILE&gt; | File to export milestones to, or `./<slug>-milestones.json` by default.  |
//! | --projects &lt;PROJECTS_FILE&gt;     | File to export projects to, or `./<slug>-projects.json` by default.      |
//! | --auth [AUTH_TOKEN]                  | GitHub OAuth2 token, required for projects.                              |
//! | --no-issues                          | Don't export issues.                                                     |
//! | --no-pulls                           | Don't export pull requests.                                              |
//! | --no-labels                          | Don't export labels.                                                     |
//! | --no-milestones                      | Don't export milestones.                                                 |
//! | --no-projects                        | Don't export projects.                                                   |
//! | --force                              | Override existing files.                                                 |
//! | --compact                            | Don't pretty-print exported JSON.                                        |


#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate clap;

mod error;
mod options;

pub mod ops;
pub mod util;

pub use self::error::Error;
pub use self::options::Options;
