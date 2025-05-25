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

        assert_equal!(node_a.value(), &Value::from("A"));
        assert_equal!(node_b.value(), &Value::from("B"));
        assert_equal!(node_c.value(), &Value::from("C"));
        assert_equal!(node_d.value(), &Value::from("D"));
        assert_equal!(node_e.value(), &Value::from("E"));
        assert_equal!(node_f.value(), &Value::from("F"));

        let mut node_f = node_d.set_left(node_f);
        assert_equal!(node_f.parent(), &node_d);
        assert_equal!(node_f.value(), &Value::from("F"));
        assert_equal!(node_d.left(), &node_f);

        let mut node_c = node_a.set_right(node_c);
        assert_equal!(node_c.parent(), &node_a);
        assert_equal!(node_c.value(), &Value::from("C"));
        assert_equal!(node_a.right(), &node_c);

        let mut node_d = node_b.set_left(node_d);
        assert_equal!(node_d.parent(), &node_b);
        assert_equal!(node_d.value(), &Value::from("D"));
        assert_equal!(node_b.left(), &node_d);

        let mut node_e = node_b.set_right(node_e);
        assert_equal!(node_e.parent(), &node_b);
        assert_equal!(node_f.parent(), &node_d);
        assert_equal!(node_e.value(), &Value::from("E"));
        assert_equal!(node_b.right(), &node_e);

        let mut node_b = node_a.set_left(node_b);
        assert_equal!(node_b.parent(), &node_a);
        assert_equal!(node_f.parent(), &node_d);
        assert_equal!(node_b.value(), &Value::from("B"));
        assert_equal!(node_a.left(), &node_b);

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

// #[test]
// fn test_tree_property_height() {
//     let mut tree = MitCourseWareTree::initial_state();

//     dbg!(&tree.node_a);
//     assert_equal!(tree.node_a.height(), 3);

//     assert_equal!(tree.node_b.height(), 2);
//     assert_equal!(tree.node_c.height(), 2);

//     assert_equal!(tree.node_d.height(), 1);
//     assert_equal!(tree.node_e.height(), 1);

//     assert_equal!(tree.node_f.height(), 0);
// }

// #[test]
// fn test_tree_property_depth() {
//     let mut tree = MitCourseWareTree::initial_state();

//     assert_equal!(tree.node_a.depth(), 0);

//     assert_equal!(tree.node_b.depth(), 1);
//     assert_equal!(tree.node_c.depth(), 1);

//     assert_equal!(tree.node_d.depth(), 2);
//     assert_equal!(tree.node_e.depth(), 2);

//     assert_equal!(tree.node_f.depth(), 3);
// }

// #[test]
// fn test_tree_property_leaf() {
//     let mut tree = MitCourseWareTree::initial_state();

//     assert_equal!(tree.node_a.leaf(), false);

//     assert_equal!(tree.node_b.leaf(), false);
//     assert_equal!(tree.node_c.leaf(), true);

//     assert_equal!(tree.node_d.leaf(), false);
//     assert_equal!(tree.node_e.leaf(), true);

//     assert_equal!(tree.node_f.leaf(), true);
// }
