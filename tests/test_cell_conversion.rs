#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_cell_from_u8() {
    let cell = Cell::from(0xF1u8);
    let head = cell.head();
    assert_equal!(head, Some(Value::Byte(0xF1u8)));
}
#[test]
fn test_cell_from_str() {
    let cell = Cell::from("head");
    let head = cell.head();
    assert_equal!(head, Some(Value::from("head")));
}

#[test]
fn test_cell_from_u64() {
    let cell = Cell::from(0xBEEFu64);
    assert_equal!(cell.head(), Some(Value::UInt(0xBEEF)));
}
#[test]
fn test_cell_from_i64() {
    let cell = Cell::from(47i64);
    assert_equal!(cell.head(), Some(Value::Int(47)));
}

#[test]
fn test_cell_from_value() {
    let cell = Cell::from(Value::Nil);
    assert_equal!(cell.head(), Some(Value::Nil));
    let cell = Cell::from(Value::from("string"));
    assert_equal!(cell.head(), Some(Value::from("string")));
    let cell = Cell::new(Value::from(0xF1u8));
    let head = cell.head();
    assert_equal!(head, Some(Value::Byte(0xF1u8)));
}
