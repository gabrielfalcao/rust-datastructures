#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_node_nil() {
    let node = Node::nil();

    assert_equal!(node.is_nil(), true);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.value(), &Value::Nil);
    assert_equal!(node.left(), None);
    assert_equal!(node.right(), None);
    assert_equal!(node.left_value(), Value::Nil);
    assert_equal!(node.right_value(), Value::Nil);
}

#[test]
fn test_node_new() {
    let node = Node::new(Value::from("value"));
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.left(), None);
    assert_equal!(node.right(), None);
    assert_equal!(node.left_value(), Value::Nil);
    assert_equal!(node.right_value(), Value::Nil);

    assert_equal!(node.value(), &Value::from("value"));
}

#[test]
fn test_set_left() {
    let mut node = Node::new(Value::from("value"));
    let left = Node::new(Value::from("left"));
    node.set_left(&left);

    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.left(), Some(&left));
    assert_equal!(node.right(), None);
    assert_equal!(node.left_value(), Value::from("left"));
    assert_equal!(node.right_value(), Value::Nil);
}

#[test]
fn test_set_right() {
    let mut node = Node::new(Value::from("value"));
    let right = Node::new(Value::from("right"));
    node.set_right(&right);

    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.right(), Some(&right));
    assert_equal!(node.left(), None);
    assert_equal!(node.left_value(), Value::Nil);
    assert_equal!(node.right_value(), Value::from("right"));
}

#[test]
fn test_clone_null() {
    let node = Node::nil();
    assert_equal!(node.clone(), Node::nil());
}

#[test]
fn test_clone_non_null() {
    let mut node = Node::new(Value::from("value"));
    let left = Node::new(Value::from("left"));
    let right = Node::new(Value::from("right"));

    node.set_left(&left);
    node.set_right(&right);

    let tree = node.clone();

    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.left(), Some(&left));
    assert_equal!(node.right(), Some(&right));
    assert_equal!(node.left_value(), Value::from("left"));
    assert_equal!(node.right_value(), Value::from("right"));
    assert_equal!(node, tree);
}
