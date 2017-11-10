use dumplingh::Error;


#[test]
fn no_more() {
    let mut out = Vec::new();
    Error::Parse {
            tp: "URL",
            wher: "book element",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"Failed to parse URL for book element.\n"[..]);
}

#[test]
fn more() {
    let mut out = Vec::new();
    Error::Parse {
            tp: "datetime",
            wher: "book element",
            more: Some("not RFC3339"),
        }
        .print_error(&mut out);
    assert_eq!(&out[..], &b"Failed to parse datetime for book element: not RFC3339.\n"[..]);
}
