// vim: tw=80

use mockall::*;

#[test]
#[should_panic(expected = "No matching expectation found")]
fn no_expectations() {
    let e = Expectations::<i32, i32>::new();
    e.call(5);
}


/// Like Mockers, calls should use the most matching recent expectation, if
/// multiple expectations match
#[test]
fn lifo_order() {
    let mut e = Expectations::<i32, i32>::new();
    e.expect()
        .with(predicate::always())
        .returning(|_| 42);
    e.expect()
        .with(predicate::eq(5))
        .returning(|_| 99);

    assert_eq!(99, e.call(5));
}

#[test]
#[should_panic(expected = "No matching expectation found")]
fn nothing_matches() {
    let mut e = Expectations::<i32, i32>::new();
    e.expect()
        .with(predicate::eq(5))
        .returning(|_| 99);

    e.call(6);
}

#[test]
fn one_match() {
    let mut e = Expectations::<i32, i32>::new();
    e.expect()
        .with(predicate::eq(4))
        .returning(|_| 42);
    e.expect()
        .with(predicate::eq(5))
        .returning(|_| 99);

    assert_eq!(42, e.call(4));
}

#[test]
fn ref_expectations() {
    let mut e = RefExpectations::<i32, i32>::new();
    e.expect()
        .with(predicate::eq(4))
        .return_const(42);
    e.expect()
        .with(predicate::eq(5))
        .return_const(99);

    assert_eq!(42, *e.call(4));
}

#[test]
fn ref_mut_expectations() {
    let mut e = RefMutExpectations::<i32, i32>::new();
    e.expect()
        .with(predicate::eq(4))
        .return_var(42);
    e.expect()
        .with(predicate::eq(5))
        .return_var(99);

    assert_eq!(42, *e.call_mut(4));
}

#[test]
#[should_panic(expected = "Method sequence violation")]
fn sequence_fail() {
    let mut seq = Sequence::new();
    let mut e = Expectations::<i32, i32>::new();
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(1))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(3))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(2))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(4))
        .returning(|_| 42);

    e.call(1);
    e.call(2);
    e.call(3);
    e.call(4);
}

#[test]
fn sequence_ok() {
    let mut seq = Sequence::new();
    let mut e = Expectations::<i32, i32>::new();
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(1))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(2))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(3))
        .returning(|_| 42);
    e.expect()
        .times(1)
        .in_sequence(&mut seq)
        .with(predicate::eq(4))
        .returning(|_| 42);

    e.call(1);
    e.call(2);
    e.call(3);
    e.call(4);
}
