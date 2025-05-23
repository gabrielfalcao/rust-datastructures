#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn value_from_static_str() {
    let value = "static-str";
    assert_equal!(Value::from(value).to_string(), "static-str");
    let value = "static-str";
    assert_display_equal!(Value::from(value), "static-str");
    let value = "static-str";
    assert_debug_equal!(Value::from(value), "'static-str");
}
#[test]
fn value_from_str() {
    let value = "str".to_string().leak();
    assert_equal!(Value::from(value).to_string(), "str");
    let value = "str".to_string().leak();
    assert_display_equal!(Value::from(value), "str");
    let value = "str".to_string().leak();
    assert_debug_equal!(Value::from(value), "'str");
}
#[test]
fn value_from_string() {
    let value = "string".to_string();
    assert_equal!(Value::from(value).to_string(), "string");
    let value = "string".to_string();
    assert_display_equal!(Value::from(value), "string");
    let value = "string".to_string();
    assert_debug_equal!(Value::from(value), "'string");
}
#[test]
fn value_display_nil() {
    assert_display_equal!(Value::Nil, "nil");
}
#[test]
fn value_debug_nil() {
    assert_debug_equal!(Value::Nil, "nil");
}
