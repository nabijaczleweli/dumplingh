use dumplingh::util::uppercase_first;


#[test]
fn simple() {
    assert_eq!(&uppercase_first("alfa"), "Alfa");
    assert_eq!(&uppercase_first("delta"), "Delta");
    assert_eq!(&uppercase_first("golf"), "Golf");
    assert_eq!(&uppercase_first("juliett"), "Juliett");
    assert_eq!(&uppercase_first("mike"), "Mike");
    assert_eq!(&uppercase_first("papa"), "Papa");
    assert_eq!(&uppercase_first("whiskey"), "Whiskey");
}

#[test]
fn already_upper() {
    assert_eq!(&uppercase_first("Tango"), "Tango");
    assert_eq!(&uppercase_first("Zulu"), "Zulu");
    assert_eq!(&uppercase_first("India"), "India");
    assert_eq!(&uppercase_first("Sierra"), "Sierra");
}

#[test]
fn single() {
    assert_eq!(&uppercase_first("c"), "C");
    assert_eq!(&uppercase_first("u"), "U");
}

#[test]
fn single_already_upper() {
    assert_eq!(&uppercase_first("R"), "R");
    assert_eq!(&uppercase_first("N"), "N");
}

#[test]
fn multi() {
    assert_eq!(&uppercase_first("bravo foxtrot X-ray yankee."), "Bravo foxtrot X-ray yankee.");
}

#[test]
fn multi_already_upper() {
    assert_eq!(&uppercase_first("Echo kilo hotel Victor."), "Echo kilo hotel Victor.");
}

#[test]
fn empty() {
    assert!(uppercase_first("").is_empty());
}
