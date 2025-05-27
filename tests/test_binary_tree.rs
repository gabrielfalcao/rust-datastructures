#![allow(unused)]
use ds::*;
use k9::assert_equal;

struct MitCourseWareTree<'t> {
    pub node_a: Node<'t>,
    pub node_b: Node<'t>,
    pub node_c: Node<'t>,
    pub node_d: Node<'t>,
    pub node_e: Node<'t>,
    pub node_f: Node<'t>,
}
impl<'t> MitCourseWareTree<'t> {
    pub fn initial_state() -> MitCourseWareTree<'t> {
        ///|||||||||||||||||||||||||||||||||||||||||||||\\\
        ///                                             \\\
        ///              INITIAL TREE STATE             \\\
        ///                                             \\\
        ///                     A                       \\\
        ///                    / \                      \\\
        ///                   /   \                     \\\
        ///                  B     C                    \\\
        ///                 / \                         \\\
        ///                /   \                        \\\
        ///               D     E                       \\\
        ///              /                              \\\
        ///             /                               \\\
        ///            F                                \\\
        ///                                             \\\
        ///                                             \\\
        let mut node_a = Node::new(Value::from("A"));
        let mut node_b = Node::new(Value::from("B"));
        let mut node_c = Node::new(Value::from("C"));
        let mut node_d = Node::new(Value::from("D"));
        let mut node_e = Node::new(Value::from("E"));
        let mut node_f = Node::new(Value::from("F"));

        assert_equal!(node_a.value(), Some(Value::from("A")));
        assert_equal!(node_b.value(), Some(Value::from("B")));
        assert_equal!(node_c.value(), Some(Value::from("C")));
        assert_equal!(node_d.value(), Some(Value::from("D")));
        assert_equal!(node_e.value(), Some(Value::from("E")));
        assert_equal!(node_f.value(), Some(Value::from("F")));

        // set D as in left of B
        node_b.set_left(&mut node_d);

        // set B as in left of A before setting E as right of B

        // so as to test that memory references are set correctly*
        node_a.set_left(&mut node_b);

        // set C as left of A
        node_a.set_right(&mut node_c);

        // set E in right of B*
        node_b.set_right(&mut node_e);

        node_d.set_left(&mut node_f);

        assert_equal!(node_b.parent_value(), node_a.value());
        assert_equal!(node_c.parent_value(), node_a.value());
        assert_equal!(node_d.parent_value(), node_b.value());
        assert_equal!(node_e.parent_value(), node_b.value());
        assert_equal!(node_f.parent_value(), node_d.value());

        assert_equal!(node_b.parent(), Some(&node_a));
        assert_equal!(node_c.parent(), Some(&node_a));
        assert_equal!(node_d.parent(), Some(&node_b));
        assert_equal!(node_e.parent(), Some(&node_b));
        assert_equal!(node_f.parent(), Some(&node_d));

        assert_equal!(node_a.left(), Some(&node_b));
        assert_equal!(node_a.right(), Some(&node_c));
        assert_equal!(node_a.parent(), None);

        assert_equal!(node_b.left(), Some(&node_d));
        assert_equal!(node_b.right(), Some(&node_e));
        assert_equal!(node_b.parent(), Some(&node_a));
        assert_equal!(node_b.parent().unwrap().parent(), None);

        assert_equal!(node_c.left(), None);
        assert_equal!(node_c.right(), None);
        assert_equal!(node_c.parent(), Some(&node_a));
        assert_equal!(node_c.parent().unwrap().parent(), None);

        assert_equal!(node_d.left(), Some(&node_f));
        assert_equal!(node_d.right(), None);
        assert_equal!(node_d.parent(), Some(&node_b));
        assert_equal!(node_d.parent().unwrap().parent(), Some(&node_a));
        assert_equal!(node_d.parent().unwrap().parent().unwrap().parent(), None);

        assert_equal!(node_f.left(), None);
        assert_equal!(node_f.right(), None);
        assert_equal!(node_f.parent(), Some(&node_d));
        assert_equal!(node_f.parent().unwrap().parent(), Some(&node_b));
        assert_equal!(node_f.parent().unwrap().parent().unwrap().parent(), Some(&node_a));
        assert_equal!(node_f.parent().unwrap().parent().unwrap().parent().unwrap().parent(), None);


        assert_equal!(node_a.refs(), 6);
        assert_equal!(node_b.refs(), 6);
        assert_equal!(node_c.refs(), 1);
        assert_equal!(node_d.refs(), 3);
        assert_equal!(node_e.refs(), 1);
        assert_equal!(node_f.refs(), 1);

        MitCourseWareTree {
            node_a,
            node_b,
            node_c,
            node_d,
            node_e,
            node_f,
        }
    }
}
#[test]
fn test_tree_initial_state() {
    MitCourseWareTree::initial_state();
}
#[test]
fn test_tree_property_height() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_c.height(), 0); // leaf
    assert_equal!(tree.node_e.height(), 0); // leaf
    assert_equal!(tree.node_f.height(), 0); // leaf

    assert_equal!(tree.node_a.height(), 3);

    assert_equal!(tree.node_b.height(), 2);

    assert_equal!(tree.node_d.height(), 1);
}

#[test]
fn test_tree_property_depth() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_a.depth(), 0);

    assert_equal!(tree.node_b.depth(), 1);
    assert_equal!(tree.node_c.depth(), 1);

    assert_equal!(tree.node_e.depth(), 2);
    assert_equal!(tree.node_d.depth(), 2);

    assert_equal!(tree.node_f.depth(), 3);
}

#[test]
fn test_tree_property_leaf() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_a.leaf(), false);

    assert_equal!(tree.node_b.leaf(), false);
    assert_equal!(tree.node_c.leaf(), true);

    assert_equal!(tree.node_d.leaf(), false);
    assert_equal!(tree.node_e.leaf(), true);

    assert_equal!(tree.node_f.leaf(), true);
}


#[test]
fn test_tree_operation_subtree_first() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_a.subtree_first(), &tree.node_f);
    assert_equal!(tree.node_b.subtree_first(), &tree.node_f);
    assert_equal!(tree.node_d.subtree_first(), &tree.node_f);
    assert_equal!(tree.node_f.subtree_first(), &tree.node_f);

    assert_equal!(tree.node_e.subtree_first(), &tree.node_e);
    assert_equal!(tree.node_c.subtree_first(), &tree.node_c);
}


#[test]
fn test_tree_operation_successor() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_e.successor(), &tree.node_a);
    assert_equal!(tree.node_f.successor(), &tree.node_d);
    assert_equal!(tree.node_b.successor(), &tree.node_e);
    assert_equal!(tree.node_d.successor(), &tree.node_b);
    assert_equal!(tree.node_a.successor(), &tree.node_c);
    assert_equal!(tree.node_c.successor(), &tree.node_c);
}


#[test]
fn test_tree_operation_successor_of_c() {
    let mut tree = MitCourseWareTree::initial_state();

    let mut node_g = Node::new(Value::from("G"));
    tree.node_c.set_left(&mut node_g);

    assert_equal!(tree.node_c.successor(), &node_g);
}

//////////////////////////////////////////////
// MUT


#[test]
fn test_tree_operation_subtree_first_mut() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_a.subtree_first_mut(), &mut tree.node_f);
    assert_equal!(tree.node_b.subtree_first_mut(), &mut tree.node_f);
    assert_equal!(tree.node_d.subtree_first_mut(), &mut tree.node_f);
    assert_equal!(tree.node_f.subtree_first_mut(), &mut tree.node_f);

    assert_equal!(tree.node_e.subtree_first_mut(), &mut tree.node_e);
    assert_equal!(tree.node_c.subtree_first_mut(), &mut tree.node_c);
}


#[test]
fn test_tree_operation_successor_mut() {
    let mut tree = MitCourseWareTree::initial_state();

    assert_equal!(tree.node_e.successor_mut(), &mut tree.node_a);
    assert_equal!(tree.node_f.successor_mut(), &mut tree.node_d);
    assert_equal!(tree.node_b.successor_mut(), &mut tree.node_e);
    assert_equal!(tree.node_d.successor_mut(), &mut tree.node_b);
    assert_equal!(tree.node_a.successor_mut(), &mut tree.node_c);
    assert_equal!(tree.node_c.successor_mut(), &mut tree.node_c);
}


#[test]
fn test_tree_operation_successor_mut_of_c() {
    let mut tree = MitCourseWareTree::initial_state();

    let mut node_g = Node::new(Value::from("G"));
    tree.node_c.set_left(&mut node_g);

    assert_equal!(tree.node_c.successor_mut(), &mut node_g);
}


#[test]
fn test_tree_operation_subtree_insert_after_node_when_node_left_is_null() {
    let mut tree = MitCourseWareTree::initial_state();

    let mut node_g = Node::new(Value::from("G"));
    tree.node_c.subtree_insert_after(&mut node_g);

    assert_equal!(node_g.parent(), Some(&tree.node_c));
}


#[test]
fn test_tree_operation_subtree_insert_after_node_when_node_right_is_non_null() {
    let mut tree = MitCourseWareTree::initial_state();

    let mut node_g = Node::new(Value::from("G"));
    tree.node_a.subtree_insert_after(&mut node_g);

    assert_equal!(node_g.parent(), tree.node_a.right());
    // TODO: assert_equal!(node_g.parent(), Some(&tree.node_c));
}
