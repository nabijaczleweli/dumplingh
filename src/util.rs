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

/// Get how many characters wide the string representation of a number is.
///
/// In other words, the max padding for filenames to make sense.
///
/// # Examples
///
/// ```
/// # use dumplingh::util::width_for;
/// assert_eq!(width_for(7), 1);
/// assert_eq!(width_for(10), 2);
/// assert_eq!(width_for(579), 3);
/// ```
pub fn width_for(len: usize) -> usize {
    if len == 0 {
        1
    } else {
        (len as f64 + 1f64).log10().ceil() as usize
    }
}
