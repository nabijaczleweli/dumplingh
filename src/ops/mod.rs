//! Main functions doing actual work.
//!
//! Use `str::parse::<RepoSlug>()` to validate your repository slug,
//! `list_{pull_requests,issues}()` to download your data and
//! `save_{to_file,data}()` to save it.


mod network;
mod write;
mod slug;

pub use self::slug::RepoSlug;
pub use self::write::{save_to_file, save_data};
pub use self::network::{list_pull_requests, list_issues};
