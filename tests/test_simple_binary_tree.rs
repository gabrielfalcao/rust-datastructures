#![allow(unused)]
use ds::*;
use k9::assert_equal;

pub fn tree<'t>() -> Node<'t> {
    let mut node_a = Node::new(Value::from("A"));
    let mut node_b = Node::new(Value::from("B"));
    let mut node_c = Node::new(Value::from("C"));
    let mut node_d = Node::new(Value::from("D"));

    assert_equal!(node_a.value(), Some(Value::from("A")));
    assert_equal!(node_b.value(), Some(Value::from("B")));
    assert_equal!(node_c.value(), Some(Value::from("C")));
    assert_equal!(node_d.value(), Some(Value::from("D")));

    node_a.set_left(&mut node_b);
    node_a.set_right(&mut node_c);
    node_b.set_left(&mut node_d);

    assert_equal!(node_a.left(), Some(&node_b));
    assert_equal!(node_a.right(), Some(&node_c));
    assert_equal!(node_b.left(), Some(&node_d));

    assert_equal!(node_b.parent_value(), node_a.value());
    assert_equal!(node_c.parent_value(), node_a.value());
    assert_equal!(node_d.parent_value(), node_b.value());

    unsafe { std::mem::transmute::<Node, Node<'t>>(node_a) }
}
#[test]
fn test_tree_initial_state() {
    let mut node_a = tree();

    assert_equal!(node_a.value(), Some(Value::from("A")));
    assert_equal!(node_a.left_value(), Some(Value::from("B")));
}

// #[test]
// fn test_tree_initial_state_inner_refs_memory_access_bad_access() {
//     let mut node_a = tree();
//
//     let mut node_b = node_a.left().expect("Node B as left of A").clone();
//     let mut node_c = node_a.right().expect("Node C as right of A").clone();
//     let mut node_d = node_b.left().expect("Node D as left of B").clone();
//
//     assert_equal!(node_b.parent_value(), node_a.value());
//     assert_equal!(node_c.parent_value(), node_a.value());
//     assert_equal!(node_d.parent_value(), node_b.value());
//
//     assert_equal!(node_b.parent(), Some(&node_a));
//     assert_equal!(node_c.parent(), Some(&node_a));
//     assert_equal!(node_d.parent(), Some(&node_b));
//
//     assert_equal!(node_a.left(), Some(&node_b));
//     assert_equal!(node_a.right(), Some(&node_c));
//     assert_equal!(node_a.parent(), None);
//     assert_equal!(node_b.left(), Some(&node_d));
//     assert_equal!(node_b.parent(), Some(&node_a));
//     assert_equal!(node_b.parent().unwrap().parent(), None);
//     assert_equal!(node_c.left(), None);
//     assert_equal!(node_c.right(), None);
//     assert_equal!(node_c.parent(), Some(&node_a));
//     assert_equal!(node_c.parent().unwrap().parent(), None);
//     assert_equal!(node_d.right(), None);
//     assert_equal!(node_d.parent(), Some(&node_b));
//     assert_equal!(node_d.parent().unwrap().parent(), Some(&node_a));
//     assert_equal!(node_d.parent().unwrap().parent().unwrap().parent(), None);
//     assert_equal!(node_a.refs(), 8);
//     assert_equal!(node_b.refs(), 7);
//     assert_equal!(node_c.refs(), 1);
//     assert_equal!(node_d.refs(), 3);
// }
//
