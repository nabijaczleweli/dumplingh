use self::super::super::Unserialisable;
use dumplingh::ops::save_data;
use dumplingh::Error;


#[test]
fn ok() {
    let mut out = vec![];
    assert_eq!(save_data(&mut out, &&["henlo", "zbingiew", "zbyniu"], true, "save_data::compact::ok()"), Ok(()));
    assert_eq!(&out[..], &b"[\"henlo\",\"zbingiew\",\"zbyniu\"]"[..]);
}

#[test]
fn serialise_error() {
    let mut out = vec![];
    assert_eq!(save_data(&mut out, &Unserialisable, true, "save_data::compact::serialise_error()"),
               Err(Error::Io {
                   desc: "save_data::compact::serialise_error()",
                   op: "serialise",
                   more: None,
               }));
}
