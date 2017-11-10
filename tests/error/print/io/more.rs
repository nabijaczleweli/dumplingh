use dumplingh::Error;


#[test]
fn normal_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "read",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"Reading input file failed: stream ended.\n"[..]);
}

#[test]
fn normal_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "output file",
            op: "create",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"Creating output file failed: stream ended.\n"[..]);
}

#[test]
fn single_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "C",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"Cing input file failed: stream ended.\n"[..]);
}

#[test]
fn single_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "e",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"ing input file failed: stream ended.\n"[..]);
}

#[test]
fn empty() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"ing input file failed: stream ended.\n"[..]);
}
