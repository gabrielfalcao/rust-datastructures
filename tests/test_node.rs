#![allow(unused)]
use ds::*;
use k9::assert_equal;

#[test]
fn test_node_nil() {
    let node = Node::nil();

    assert_equal!(node.is_nil(), true);
    assert_equal!(node.parent(), None);
    assert_equal!(node.value(), None);
    assert_equal!(node.left(), None);
    assert_equal!(node.right(), None);
    assert_equal!(node.left_value(), None);
    assert_equal!(node.right_value(), None);

    let expected = {
        let node = Node::nil();
        node
    };
    assert_equal!(node, expected);
    assert_equal!(node, Node::nil());
}

#[test]
fn test_node_new() {
    let node = Node::new(Value::from("value"));
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.parent(), None);
    assert_equal!(node.left(), None);
    assert_equal!(node.right(), None);
    assert_equal!(node.left_value(), None);
    assert_equal!(node.right_value(), None);

    assert_equal!(node.value(), Some(Value::from("value")));

    let expected = {
        let node = Node::new(Value::from("value"));
        node
    };
    assert_equal!(node, expected);
    assert_equal!(node, Node::new(Value::from("value")));
}

#[test]
fn test_set_left() {
    let mut node = Node::new(Value::from("value"));
    let mut left = Node::new(Value::from("left"));

    node.set_left(&mut left);

    assert_equal!(left.parent(), Some(&node));

    assert_equal!(node.is_nil(), false);
    assert_equal!(left.parent_value(), Some(Value::from("value")));
    assert_equal!(left.parent(), Some(&node));
    assert_equal!(node.value(), Some(Value::from("value")));
    assert_equal!(node.parent(), None);
    assert_equal!(node.left_value(), Some(Value::from("left")));
    assert_equal!(node.refs(), 3);
    assert_equal!(left.refs(), 2);
    assert_equal!(node.left(), Some(&left));
    assert_equal!(node.right_value(), None);
    assert_equal!(node.right(), None);

    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("left"));
        node.set_left(&mut left);
        node
    };
    assert_equal!(node, expected);

    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("left"));
        node.set_left(&mut left);
        left
    };
    assert_equal!(left, expected);
}
#[test]
fn test_set_right() {
    let mut node = Node::new(Value::from("value"));
    let mut right = Node::new(Value::from("right"));

    node.set_right(&mut right);

    assert_equal!(right.parent(), Some(&node));

    assert_equal!(node.is_nil(), false);
    assert_equal!(right.parent_value(), Some(Value::from("value")));
    assert_equal!(right.parent(), Some(&node));
    assert_equal!(node.value(), Some(Value::from("value")));
    assert_equal!(node.parent(), None);
    assert_equal!(node.right_value(), Some(Value::from("right")));
    assert_equal!(node.right(), Some(&right));
    assert_equal!(node.left_value(), None);
    assert_equal!(node.left(), None);

    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("right"));
        node.set_right(&mut left);
        node
    };
    assert_equal!(node, expected);

    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("right"));
        node.set_right(&mut left);
        left
    };
    assert_equal!(right, expected);
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

    node.set_left(&mut left);
    node.set_right(&mut right);

    assert_equal!(node.parent(), None);
    assert_equal!(node.is_nil(), false);
    assert_equal!(node.left(), Some(&left));
    assert_equal!(node.right(), Some(&right));
    assert_equal!(node.left_value(), Some(Value::from("left")));
    assert_equal!(node.right_value(), Some(Value::from("right")));

    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("left"));
        let mut right = Node::new(Value::from("right"));

        node.set_left(&mut left);
        node.set_right(&mut right);
        node
    };
    assert_equal!(node, expected);
    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut left = Node::new(Value::from("left"));
        node.set_left(&mut left);
        left
    };
    assert_equal!(left, expected);
    let expected = {
        let mut node = Node::new(Value::from("value"));
        let mut right = Node::new(Value::from("right"));
        node.set_right(&mut right);
        right
    };
    assert_equal!(right, expected);

    let mut tree = node.clone();
    assert_equal!(node, tree);
}
