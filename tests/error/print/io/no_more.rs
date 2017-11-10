use dumplingh::Error;


#[test]
fn normal_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "read",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], b"Reading input file failed.\n");
}

#[test]
fn normal_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "output file",
            op: "create",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], b"Creating output file failed.\n");
}

#[test]
fn single_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "C",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], b"Cing input file failed.\n");
}

#[test]
fn single_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "e",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], b"ing input file failed.\n");
}

#[test]
fn empty() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], b"ing input file failed.\n");
}
