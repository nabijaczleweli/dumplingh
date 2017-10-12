use dumplingh::ops::RepoSlug;
use std::io::Write;

mod from_str;


#[test]
fn display() {
    let mut out = Vec::new();
    write!(out,
           "{}",
           RepoSlug {
               username: "nabijaczleweli".to_string(),
               repository: "dumplingh".to_string(),
           })
        .unwrap();
    assert_eq!(&out[..], b"nabijaczleweli/dumplingh");

    assert_eq!(RepoSlug {
                       username: "nabijaczleweli".to_string(),
                       repository: "dumplingh".to_string(),
                   }
                   .to_string(),
               "nabijaczleweli/dumplingh");
}

#[test]
fn filename() {
    assert_eq!(RepoSlug {
                       username: "nabijaczleweli".to_string(),
                       repository: "dumplingh".to_string(),
                   }
                   .filename(),
               "nabijaczleweli-dumplingh");
}
