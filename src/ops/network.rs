use reqwest::header::{Authorization as AuthorizationHeader, RelationType, UserAgent, LinkValue as HeaderLinkValue, Accept as AcceptHeader, Bearer as BearerAuth,
                      Link as LinkHeader, qitem as quality_item};
use self::super::super::util::{GITHUB_PROJECTS_ACCEPT_MIMETYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use reqwest::{IntoUrl, Client};
use self::super::super::Error;
use self::super::RepoSlug;
use reqwest::mime::Mime;
use serde_json::Value;


fn request<U: IntoUrl, T: DeserializeOwned>(url: U, accept: Option<&Mime>, auth: Option<&str>, desc: &'static str) -> Result<(T, Vec<HeaderLinkValue>), Error> {
    let mut bld = Client::builder()
        .gzip(true)
        .build()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "client build",
                more: None,
            }
        })?
        .get(url);
    bld.header(UserAgent::new(USER_AGENT));
    if let Some(acc) = accept {
        bld.header(AcceptHeader(vec![quality_item(acc.clone())]));
    }
    if let Some(auth) = auth {
        bld.header(AuthorizationHeader(BearerAuth { token: auth.to_string() }));
    }

    let mut resp = bld.send()
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

fn list_things(repo: &RepoSlug, accept: Option<&Mime>, auth: Option<&str>, arg: &'static str) -> Result<Vec<Value>, Error> {
    let mut res = vec![];
    let mut url = format!("https://api.github.com/repos/{}/{}?state=all", repo, arg);
    loop {
        let (iss, links): (Vec<Value>, Vec<HeaderLinkValue>) = request(&url, accept, auth, arg)?;
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
pub fn list_issues(repo: &RepoSlug, auth: Option<&str>) -> Result<Vec<Value>, Error> {
    list_things(repo, None, auth, "issues")
}

/// List all PRs from the specified repository.
pub fn list_pull_requests(repo: &RepoSlug, auth: Option<&str>) -> Result<Vec<Value>, Error> {
    list_things(repo, None, auth, "pulls")
}

/// List all labels for the specified repository.
pub fn list_labels(repo: &RepoSlug, auth: Option<&str>) -> Result<Vec<Value>, Error> {
    list_things(repo, None, auth, "labels")
}

/// List all milestones for the specified repository.
pub fn list_milestones(repo: &RepoSlug, auth: Option<&str>) -> Result<Vec<Value>, Error> {
    list_things(repo, None, auth, "milestones")
}

/// List all projects for the specified repository, authenticating with the supplied token.
pub fn list_projects(repo: &RepoSlug, auth: &str) -> Result<Vec<Value>, Error> {
    list_things(repo, Some(&GITHUB_PROJECTS_ACCEPT_MIMETYPE), Some(auth), "projects")
}
