use self::super::super::Unserialisable;
use dumplingh::ops::save_to_file;
use std::fs::{self, File};
use std::env::temp_dir;
use dumplingh::Error;
use std::io::Read;


#[test]
fn ok() {
    let mut tf = temp_dir().join("dumplingh-test").join("ops-save_to_file-pretty-ok");
    fs::create_dir_all(&tf).unwrap();
    tf.push("issues.json");

    assert_eq!(save_to_file(&tf, &&["henlo", "zbingiew", "zbyniu"], false, "save_to_file::pretty::ok()"), Ok(()));

    let mut buf = String::new();
    File::open(tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf,
               "\
[\n\
\x20\x20\"henlo\",\n\
\x20\x20\"zbingiew\",\n\
\x20\x20\"zbyniu\"\n\
]\
");
}

#[test]
fn serialise_error() {
    let mut tf = temp_dir().join("dumplingh-test").join("ops-save_to_file-pretty-serialise_error");
    fs::create_dir_all(&tf).unwrap();
    tf.push("issues.json");

    assert_eq!(save_to_file(&tf, &Unserialisable, false, "save_to_file::pretty::serialise_error()"),
               Err(Error::Io {
                   desc: "save_to_file::pretty::serialise_error()",
                   op: "serialise",
                   more: None,
               }));

    let mut buf = String::new();
    File::open(tf).unwrap().read_to_string(&mut buf).unwrap();
    assert_eq!(buf, "");
}

#[test]
fn write_error() {
    let tf = temp_dir().join("dumplingh-test").join("ops-save_to_file-pretty-write_error").join("issues.json");
    fs::create_dir_all(&tf).unwrap();

    assert_eq!(save_to_file(&tf, &&["henlo", "zbingiew", "zbyniu"], false, "save_to_file::pretty::write_error()"),
               Err(Error::Io {
                   desc: "save_to_file::pretty::write_error()",
                   op: "create",
                   more: None,
               }));
}
