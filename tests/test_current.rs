#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_add_when_tail_is_null() {
    let mut head = Cell::new(Value::from("head"));
    let mut cell = Cell::new(Value::from("cell"));

    assert_equal!(head.values(), vec![Value::from("head")]);
    assert_equal!(head.len(), 1);

    head.add(&mut cell);

    assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    assert_equal!(head.len(), 2);
}
