use dumplingh::util::width_for;


#[test]
fn simple() {
    for i in 1..10 {
        assert_eq!(width_for(i), 1);
    }
    for i in 10..100 {
        assert_eq!(width_for(i), 2);
    }
    for i in 100..1000 {
        assert_eq!(width_for(i), 3);
    }
    for i in 1000..10_000 {
        assert_eq!(width_for(i), 4);
    }
    for i in 10_000..100_000 {
        assert_eq!(width_for(i), 5);
    }
    for i in 100_000..1_000_000 {
        assert_eq!(width_for(i), 6);
    }
    for i in 1_000_000..10_000_000 {
        assert_eq!(width_for(i), 7);
    }
}

#[test]
fn zero() {
    assert_eq!(width_for(0), 1);
}
