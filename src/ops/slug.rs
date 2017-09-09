use self::super::super::Error;
use std::fmt::{self, Write};
use std::str::FromStr;


/// A repository "slug", which is a fancy word for username-reponame pair.
///
/// A slug take the form of `"uname/repo"`,
/// IOW the string you can stick after `https://github.com/`.
///
/// # Examples
///
/// ```
/// # use dumplingh::ops::RepoSlug;
/// let slug: RepoSlug = "nabijaczleweli/dumplingh".parse().unwrap();
/// assert_eq!(slug, RepoSlug {
///     username: "nabijaczleweli".to_string(),
///     repository: "dumplingh".to_string(),
/// });
/// assert_eq!(&slug.to_string(), "nabijaczleweli/dumplingh");
/// ```
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RepoSlug {
    /// Username the repository belongs to.
    pub username: String,
    /// Repository name.
    pub repository: String,
}

impl FromStr for RepoSlug {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn field_err(field: &'static str, more: &'static str) -> Error {
            Error::Parse {
                tp: field,
                wher: "repository slug",
                more: Some(more),
            }
        }


        let mut itr = s.split('/');
        let username = itr.next();
        let repository = itr.next();

        if itr.next().is_none() {
            let username = username.ok_or_else(|| field_err("username", "missing"))?;
            let repository = repository.ok_or_else(|| field_err("repository", "missing"))?;

            match (username, repository) {
                ("", "") => Err(field_err("username and repository", "empty")),
                ("", _) => Err(field_err("username", "empty")),
                (_, "") => Err(field_err("repository", "empty")),
                (u, r) => {
                    Ok(RepoSlug {
                        username: u.to_string(),
                        repository: r.to_string(),
                    })
                }
            }
        } else {
            Err(field_err("epilogue", "extraneous segments"))
        }
    }
}

impl fmt::Display for RepoSlug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.username)?;
        f.write_char('/')?;
        f.write_str(&self.repository)?;

        Ok(())
    }
}
