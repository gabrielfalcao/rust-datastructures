#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_node_nil() {
    let node = Node::nil();

    assert_equal!(node.is_nil(), true);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.value(), &Value::Nil);
    assert_equal!(node.left(), &Node::nil());
    assert_equal!(node.right(), &Node::nil());
    assert_equal!(node.left_value(), Value::Nil);
    assert_equal!(node.right_value(), Value::Nil);
}

#[test]
fn test_node_new() {
    let node = Node::new(Value::from("value"));
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.left(), &Node::nil());
    assert_equal!(node.right(), &Node::nil());
    assert_equal!(node.left_value(), Value::Nil);
    assert_equal!(node.right_value(), Value::Nil);

    assert_equal!(node.value(), &Value::from("value"));
}

#[test]
fn test_set_left() {
    let mut node = Node::new(Value::from("value"));
    let mut left = Node::new(Value::from("left"));

    let mut left = node.set_left(left);

    assert_equal!(left.parent(), &node);

    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.left(), &mut left);
    assert_equal!(node.right(), &Node::nil());
    assert_equal!(node.left_value(), Value::from("left"));
    assert_equal!(node.right_value(), Value::Nil);
}

#[test]
fn test_set_right() {
    let mut node = Node::new(Value::from("value"));
    let mut right = Node::new(Value::from("right"));

    let mut right = node.set_right(right);

    assert_equal!(right.parent(), &node);

    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.right(), &mut right);
    assert_equal!(node.left(), &Node::nil());
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
    let mut left = Node::new(Value::from("left"));
    let mut right = Node::new(Value::from("right"));

    let mut left = node.set_left(left);
    let mut right = node.set_right(right);

    let mut tree = node.clone();

    assert_equal!(node.parent(), &Node::nil());
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.left(), &mut left);
    assert_equal!(node.right(), &mut right);
    assert_equal!(node.left_value(), Value::from("left"));
    assert_equal!(node.right_value(), Value::from("right"));
    assert_equal!(node, tree);
}
