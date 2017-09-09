use dumplingh::ops::RepoSlug;
use dumplingh::Error;


#[test]
fn valid() {
    assert_eq!("nabijaczleweli/dumplingh".parse(),
               Ok(RepoSlug {
                   username: "nabijaczleweli".to_string(),
                   repository: "dumplingh".to_string(),
               }));
}

#[test]
fn too_many_segments() {
    let err: Result<RepoSlug, Error> = Err(Error::Parse {
        tp: "epilogue",
        wher: "repository slug",
        more: Some("extraneous segments"),
    });
    assert_eq!("nabijaczleweli/dumplingh/issue".parse(), err);
    assert_eq!("nabijaczleweli/dumplingh/issue/12".parse(), err);
}

#[test]
fn no_repository() {
    assert_eq!("nabijaczleweli".parse::<RepoSlug>(),
               Err(Error::Parse {
                   tp: "repository",
                   wher: "repository slug",
                   more: Some("missing"),
               }));
}

#[test]
fn empty_username_and_repository() {
    assert_eq!("/".parse::<RepoSlug>(),
               Err(Error::Parse {
                   tp: "username and repository",
                   wher: "repository slug",
                   more: Some("empty"),
               }));
}

#[test]
fn empty_username() {
    assert_eq!("/dumplingh".parse::<RepoSlug>(),
               Err(Error::Parse {
                   tp: "username",
                   wher: "repository slug",
                   more: Some("empty"),
               }));
}

#[test]
fn empty_repository() {
    assert_eq!("nabijaczleweli/".parse::<RepoSlug>(),
               Err(Error::Parse {
                   tp: "repository",
                   wher: "repository slug",
                   more: Some("empty"),
               }));
}

#[test]
fn round_trip() {
    assert_eq!(&"nabijaczleweli/dumplingh".parse::<RepoSlug>().unwrap().to_string(), "nabijaczleweli/dumplingh");
}
