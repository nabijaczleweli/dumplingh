use self::super::Unserialisable;
use dumplingh::ops::save_data;
use dumplingh::Error;


#[test]
fn ok() {
    let mut out = vec![];
    assert_eq!(save_data(&mut out, &&["henlo", "zbingiew", "zbyniu"], "save_data::ok()"), Ok(()));
    assert_eq!(&out[..],
               &b"\
[\n\
\x20\x20\"henlo\",\n\
\x20\x20\"zbingiew\",\n\
\x20\x20\"zbyniu\"\n\
]\
"[..]);
}

#[test]
fn serialise_error() {
    let mut out = vec![];
    assert_eq!(save_data(&mut out, &Unserialisable, "save_data::serialise_error()"),
               Err(Error::Io {
                   desc: "save_data::serialise_error()",
                   op: "serialise",
                   more: None,
               }));
}
