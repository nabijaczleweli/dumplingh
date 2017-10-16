use self::super::super::Error;
use serde::Serialize;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use serde_json;


/// Save the specified data to the specified file, serialised as JSON.
///
/// Convenience function for wrapping [`save_data(File::create())`](fn.save_data.html).
///
/// # Examples
///
/// ```
/// # use dumplingh::ops::save_to_file;
/// # use std::env::temp_dir;
/// # use std::fs;
/// # let mut issues_file = temp_dir().join("dumplingh-doctest").join("ops-save_to_file");
/// # fs::create_dir_all(&issues_file).unwrap();
/// # issues_file.push("issues.json");
/// # let issues = "issues.json";
/// assert_eq!(save_to_file(&issues_file, &issues, "issues"), Ok(()));
/// ```
pub fn save_to_file<P: AsRef<Path>, S: Serialize>(out_p: P, val: &S, what: &'static str) -> Result<(), Error> {
    save_data(&mut File::create(out_p).map_err(|_| {
            Error::Io {
                desc: what,
                op: "create",
                more: None,
            }
        })?,
              val,
              what)
}

/// Write the specified data to the output stream, serialised as JSON.
///
/// # Examples
///
/// ```
/// # use dumplingh::ops::save_data;
/// # /*
/// let issues = [];
/// # */
/// # let issues: [bool; 0] = [];
/// let mut out = vec![];
/// assert_eq!(save_data(&mut out, &issues, "issues"), Ok(()));
/// assert_eq!(out, b"[]");
/// ```
pub fn save_data<W: Write, S: Serialize>(out: &mut W, val: &S, what: &'static str) -> Result<(), Error> {
    serde_json::to_writer_pretty(out, val).map_err(|_| {
        Error::Io {
            desc: what,
            op: "serialise",
            more: None,
        }
    })
}
