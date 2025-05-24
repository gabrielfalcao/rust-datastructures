#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_cell_head() {
    let cell = Cell::new(Value::from("head"));
    let head = cell.head();

    assert_equal!(head, Some(Value::from("head")));
}
#[test]
fn test_clone() {
    let mut head = Cell::new(Value::from("head"));
    let mut tail = Cell::new(Value::from("tail"));
    head.add(&mut tail);

    let cell = head.clone();
    assert_equal!(head, cell);
}

#[test]
fn test_add_when_tail_is_null() {
    let mut head = Cell::new(Value::from("head"));
    let mut cell = Cell::new(Value::from("cell"));

    assert_equal!(head.values(), vec![Value::from("head")]);
    assert_equal!(head.len(), 1);

    head.add(&mut cell);

    assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    assert_equal!(head.len(), 2);

    let mut tail = Cell::new(Value::from("tail"));

    assert_equal!(tail.len(), 1);
    assert_equal!(tail.values(), vec![Value::from("tail")]);

    assert_equal!(cell.values(), vec![Value::from("cell")]);
    assert_equal!(cell.len(), 1);
    cell.add(&mut tail);

    assert_equal!(
        head.values(),
        vec![Value::from("head"), Value::from("cell"), Value::from("tail")]
    );
    assert_equal!(head.len(), 3);
    assert_equal!(cell.values(), vec![Value::from("cell"), Value::from("tail")]);
    assert_equal!(cell.len(), 2);
    assert_equal!(tail.values(), vec![Value::from("tail")]);
    assert_equal!(tail.len(), 1);
}

#[test]
fn test_add_when_tail_is_not_necessarily_null() {
    let mut head = Cell::new(Value::from("head"));
    let mut cell = Cell::new(Value::from("cell"));
    let mut tail = Cell::new(Value::from("tail"));

    assert_equal!(head.values(), vec![Value::from("head")]);
    assert_equal!(head.len(), 1);

    head.add(&mut cell);
    assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    assert_equal!(head.len(), 2);

    head.add(&mut tail);
    assert_equal!(
        head.values(),
        vec![Value::from("head"), Value::from("cell"), Value::from("tail")]
    );
    assert_equal!(head.len(), 3);
}
#[test]
fn test_add_and_pop() {
    let mut head = Cell::new(Value::from("head"));
    let mut cell = Cell::new(Value::from("cell"));

    assert_equal!(head.len(), 1);
    assert_equal!(head.values(), vec![Value::from("head")]);

    head.add(&mut cell);

    assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    assert_equal!(head.len(), 2);

    assert_equal!(head.pop(), true);

    assert_equal!(head.values(), vec![Value::from("head")]);
    assert_equal!(head.len(), 1);

    assert_equal!(head.pop(), true);

    assert_equal!(head.values(), vec![]);
    assert_equal!(head.len(), 0);

    assert_equal!(head.pop(), false);
    assert_equal!(head.values(), vec![]);
    assert_equal!(head.len(), 0);

    assert_equal!(head.pop(), false);
    assert_equal!(head.values(), vec![]);
    assert_equal!(head.len(), 0);
}
