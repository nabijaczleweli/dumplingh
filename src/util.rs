//! Module containing various utility functions.


use reqwest::mime::Mime;


/// App name and version to use with User-Agent request header.
pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    /// Custom media type for the `Accept` header to get [projects](https://developer.github.com/v3/projects/) from the GH API.
    pub static ref GITHUB_PROJECTS_ACCEPT_MIMETYPE: Mime = "application/vnd.github.inertia-preview+json".parse().unwrap();
}


/// Uppercase the first character of the supplied string.
///
/// Based on http://stackoverflow.com/a/38406885/2851815
///
/// # Examples
///
/// ```
/// # use dumplingh::util::uppercase_first;
/// assert_eq!(&uppercase_first("abolish"), "Abolish");
/// ```
pub fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
