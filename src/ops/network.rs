use reqwest::header::{RelationType, UserAgent, LinkValue as HeaderLinkValue, Link as LinkHeader};
use self::super::super::util::USER_AGENT;
use serde::de::DeserializeOwned;
use reqwest::{IntoUrl, Client};
use self::super::super::Error;
use self::super::RepoSlug;
use serde_json::Value;


fn request<U: IntoUrl, T: DeserializeOwned>(url: U, desc: &'static str) -> Result<(T, Vec<HeaderLinkValue>), Error> {
    let mut resp = Client::builder().gzip(true)
        .build()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "client build",
                more: None,
            }
        })?
        .get(url)
        .header(UserAgent::new(USER_AGENT))
        .send()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "request",
                more: None,
            }
        })?;

    let res = resp.json()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "parse",
                more: None,
            }
        })?;

    Ok((res, resp.headers().get().map(|lh: &LinkHeader| lh.values().into()).unwrap_or_else(|| vec![])))
}

fn list_things(repo: &RepoSlug, arg: &'static str) -> Result<Vec<Value>, Error> {
    let mut res = vec![];
    let mut url = format!("https://api.github.com/repos/{}/{}?state=all", repo, arg);
    loop {
        let (iss, links): (Vec<Value>, Vec<HeaderLinkValue>) = request(&url, arg)?;
        res.extend(iss);
        if let Some(next_link) = links.into_iter().find(|l| l.rel().map(|l| l.contains(&RelationType::Next)).unwrap_or(false)) {
            url = next_link.link().to_string();
        } else {
            break;
        }
    }
    Ok(res)
}

/// List all issues from the specified repository.
pub fn list_issues(repo: &RepoSlug) -> Result<Vec<Value>, Error> {
    list_things(repo, "issues")
}

/// List all PRs from the specified repository.
pub fn list_pull_requests(repo: &RepoSlug) -> Result<Vec<Value>, Error> {
    list_things(repo, "pulls")
}

/// List all labels for the specified repository.
pub fn list_labels(repo: &RepoSlug) -> Result<Vec<Value>, Error> {
    list_things(repo, "labels")
}

/// List all milestones for the specified repository.
pub fn list_milestones(repo: &RepoSlug) -> Result<Vec<Value>, Error> {
    list_things(repo, "milestones")
}
