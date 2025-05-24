#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_cons() {
    let cell = cons("head", &mut Cell::from("tail"));
    assert_equal!(cell.values(), vec![Value::from("head"), Value::from("tail")]);
}

#[test]
fn test_list() {
    let cell = list!("head", "middle", 33u8, "tail");
    assert_equal!(
        cell.values(),
        vec![
            Value::from("head"),
            Value::from("middle"),
            Value::Byte(33),
            Value::from("tail"),
        ]
    );
}

#[test]
fn test_car() {
    let cell = list!("head", "middle", 33u8, "tail");
    assert_equal!(cell.head(), Some(Value::from("head")));
    assert_equal!(car(&cell), Value::from("head"));
}


#[test]
fn test_cdr() {
    let cell = list!("head", "middle", 33u8, "tail");
    assert_equal!(cdr(&cell), list!("middle", 33u8, "tail"));
}
