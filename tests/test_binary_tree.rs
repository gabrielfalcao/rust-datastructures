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
        // Scenario: Create nodes and test the equality of its items
        //
        // Given that I create disconnected nodes with values A through F
        let mut node_a = Node::new(Value::from("A"));
        let mut node_b = Node::new(Value::from("B"));
        let mut node_c = Node::new(Value::from("C"));
        let mut node_d = Node::new(Value::from("D"));
        let mut node_e = Node::new(Value::from("E"));
        let mut node_f = Node::new(Value::from("F"));

        // Then each node has its corresponding value
        assert_equal!(node_a.value(), Some(Value::from("A")));
        assert_equal!(node_b.value(), Some(Value::from("B")));
        assert_equal!(node_c.value(), Some(Value::from("C")));
        assert_equal!(node_d.value(), Some(Value::from("D")));
        assert_equal!(node_e.value(), Some(Value::from("E")));
        assert_equal!(node_f.value(), Some(Value::from("F")));

        /// /////////////////////////////////////////////////////////////////// ///
        /// Scenario: Connect nodes and check the equality of the items parents ///
        ///                                                                     ///
        /// Given that I set D as in left of B                                  ///
        node_b.set_left(&mut node_d);
        ///
        ///                                                                     ///
        /// And that I set B as in left of A before setting E as right of B     ///
        /// so as to test that memory references are set correctly*             ///
        node_a.set_left(&mut node_b);
        ///
        ///                                                                     ///
        /// And that I set C as left of A                                       ///
        node_a.set_right(&mut node_c);
        ///
        ///                                                                     ///
        /// And that I set E in right of B*                                     ///
        node_b.set_right(&mut node_e);
        ///
        ///                                                                     ///
        /// And that I set F in left of D                                       ///
        node_d.set_left(&mut node_f);
        ///
        ///                                                                     ///
        /// Then the parent of node B parent has value "A"                      ///
        assert_equal!(node_b.parent_value(), node_a.value());
        ///
        /// And the parent of node C parent has value "A"                       ///
        assert_equal!(node_c.parent_value(), node_a.value());
        ///
        /// And the parent of node D parent has value "B"                       ///
        assert_equal!(node_d.parent_value(), node_b.value());
        ///
        /// And the parent of node E parent has value "B"                       ///
        assert_equal!(node_e.parent_value(), node_b.value());
        ///
        ///                                                                     ///
        /// And the parent of node F parent has value "D"                       ///
        assert_equal!(node_f.parent_value(), node_d.value());
        ///

        /// //////////////////////////////////////////////// ///
        /// Scenario: Check the equality of parent nodes     ///
        /// (i.e.: `impl PartialEq for Node')                ///
        ///                                                  ///
        /// Given that all nodes have been connected         ///
        ///                                                  ///
        /// Then the parent of node B is node A              ///
        assert_equal!(node_b.parent(), Some(&node_a));
        ///
        ///                                                  ///
        /// And the parent of node C is node A               ///
        assert_equal!(node_c.parent(), Some(&node_a));
        ///
        ///                                                  ///
        ///                                                  ///
        /// And the parent of node D is node B               ///
        assert_equal!(node_d.parent(), Some(&node_b));
        ///
        ///                                                  ///
        /// And the parent of node E is node B               ///
        assert_equal!(node_e.parent(), Some(&node_b));
        ///
        ///                                                  ///
        /// And the parent of node F is node D               ///
        assert_equal!(node_f.parent(), Some(&node_d));
        ///
        ///                                                  ///

        /// ////////////////////////////////////////////////////////////////////////////////////// ///
        /// Scenario: Check the equality of left and right nodes                                   ///
        /// (i.e.: `impl PartialEq for Node')                                                      ///
        ///                                                                                        ///
        /// Given that all nodes have been connected                                               ///
        ///                                                                                        ///
        /// Then the left of node A is node B                                                      ///
        assert_equal!(node_a.left(), Some(&node_b));
        ///
        ///                                                                                        ///
        /// And the right of node A is node C                                                      ///
        assert_equal!(node_a.right(), Some(&node_c));
        ///
        ///                                                                                        ///
        /// And node A is the root node (no parent)                                                ///
        assert_equal!(node_a.parent(), None);
        ///
        ///                                                                                        ///
        ///                                                                                        ///
        /// And the left of node B is node D                                                       ///
        assert_equal!(node_b.left(), Some(&node_d));
        ///
        ///                                                                                        ///
        /// And the right of node B is node E                                                      ///
        assert_equal!(node_b.right(), Some(&node_e));
        ///
        ///                                                                                        ///
        /// And the parent of node B is node A                                                     ///
        assert_equal!(node_b.parent(), Some(&node_a));
        ///
        ///                                                                                        ///
        /// And node B has no grand-parent                                                         ///
        assert_equal!(node_b.parent().unwrap().parent(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_c.left(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_c.right(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_c.parent(), Some(&node_a));
        ///
        ///                                                                                        ///
        assert_equal!(node_c.parent().unwrap().parent(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_d.left(), Some(&node_f));
        ///
        ///                                                                                        ///
        assert_equal!(node_d.right(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_d.parent(), Some(&node_b));
        ///
        ///                                                                                        ///
        assert_equal!(node_d.parent().unwrap().parent(), Some(&node_a));
        ///
        ///                                                                                        ///
        assert_equal!(node_d.parent().unwrap().parent().unwrap().parent(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_f.left(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_f.right(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_f.parent(), Some(&node_d));
        ///
        ///                                                                                        ///
        assert_equal!(node_f.parent().unwrap().parent(), Some(&node_b));
        ///
        ///                                                                                        ///
        assert_equal!(node_f.parent().unwrap().parent().unwrap().parent(), Some(&node_a));
        ///
        ///                                                                                        ///
        assert_equal!(node_f.parent().unwrap().parent().unwrap().parent().unwrap().parent(), None);
        ///
        ///                                                                                        ///
        assert_equal!(node_a.refs(), 8);
        ///
        ///                                                                                        ///
        assert_equal!(node_b.refs(), 7);
        ///
        ///                                                                                        ///
        assert_equal!(node_c.refs(), 1);
        ///
        ///                                                                                        ///
        assert_equal!(node_d.refs(), 3);
        ///
        ///                                                                                        ///
        assert_equal!(node_e.refs(), 1);
        ///
        ///                                                                                        ///
        assert_equal!(node_f.refs(), 1);
        ///
        ///                                                                                        ///
        let tree = unsafe {
            MitCourseWareTree {
                node_a,
                node_b,
                node_c,
                node_d,
                node_e,
                node_f,
            }
        };
        assert_equal!(tree.node_a.refs(), 8);
        assert_equal!(tree.node_b.refs(), 7);
        assert_equal!(tree.node_c.refs(), 1);
        assert_equal!(tree.node_d.refs(), 3);
        assert_equal!(tree.node_e.refs(), 1);
        assert_equal!(tree.node_f.refs(), 1);
        unsafe { std::mem::transmute::<MitCourseWareTree, MitCourseWareTree<'t>>(tree) }
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
//
// #[test]
// fn test_tree_property_depth() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.depth(), 0);
//
//     assert_equal!(tree.node_b.depth(), 1);
//     assert_equal!(tree.node_c.depth(), 1);
//
//     assert_equal!(tree.node_e.depth(), 2);
//     assert_equal!(tree.node_d.depth(), 2);
//
//     assert_equal!(tree.node_f.depth(), 3);
// }
//
// #[test]
// fn test_tree_property_leaf() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.leaf(), false);
//
//     assert_equal!(tree.node_b.leaf(), false);
//     assert_equal!(tree.node_c.leaf(), true);
//
//     assert_equal!(tree.node_d.leaf(), false);
//     assert_equal!(tree.node_e.leaf(), true);
//
//     assert_equal!(tree.node_f.leaf(), true);
// }
//
// #[test]
// fn test_tree_operation_subtree_first() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.subtree_first(), &tree.node_f);
//     assert_equal!(tree.node_b.subtree_first(), &tree.node_f);
//     assert_equal!(tree.node_d.subtree_first(), &tree.node_f);
//     assert_equal!(tree.node_f.subtree_first(), &tree.node_f);
//
//     assert_equal!(tree.node_e.subtree_first(), &tree.node_e);
//     assert_equal!(tree.node_c.subtree_first(), &tree.node_c);
// }
//
// #[test]
// fn test_tree_operation_successor() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_e.successor(), &tree.node_a);
//     assert_equal!(tree.node_f.successor(), &tree.node_d);
//     assert_equal!(tree.node_b.successor(), &tree.node_e);
//     assert_equal!(tree.node_d.successor(), &tree.node_b);
//     assert_equal!(tree.node_a.successor(), &tree.node_c);
//     assert_equal!(tree.node_c.successor(), &tree.node_c);
// }
//
// #[test]
// fn test_tree_operation_successor_of_c() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_c.set_left(&mut node_g);
//
//     assert_equal!(tree.node_c.successor(), &node_g);
// }
//
// //////////////////////////////////////////////
// // MUT
//
// #[test]
// fn test_tree_operation_subtree_first_mut() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.subtree_first_mut(), &mut tree.node_f);
//     assert_equal!(tree.node_b.subtree_first_mut(), &mut tree.node_f);
//     assert_equal!(tree.node_d.subtree_first_mut(), &mut tree.node_f);
//     assert_equal!(tree.node_f.subtree_first_mut(), &mut tree.node_f);
//
//     assert_equal!(tree.node_e.subtree_first_mut(), &mut tree.node_e);
//     assert_equal!(tree.node_c.subtree_first_mut(), &mut tree.node_c);
// }
//
// #[test]
// fn test_tree_operation_successor_mut() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_e.successor_mut(), &mut tree.node_a);
//     assert_equal!(tree.node_f.successor_mut(), &mut tree.node_d);
//     assert_equal!(tree.node_b.successor_mut(), &mut tree.node_e);
//     assert_equal!(tree.node_d.successor_mut(), &mut tree.node_b);
//     assert_equal!(tree.node_a.successor_mut(), &mut tree.node_c);
//     assert_equal!(tree.node_c.successor_mut(), &mut tree.node_c);
// }
//
// #[test]
// fn test_tree_operation_successor_mut_of_c() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_c.set_left(&mut node_g);
//
//     assert_equal!(tree.node_c.successor_mut(), &mut node_g);
// }
//
// #[test]
// fn test_tree_operation_subtree_insert_after_node_when_node_left_is_null() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_c.subtree_insert_after(&mut node_g);
//
//     assert_equal!(node_g.parent(), Some(&tree.node_c.clone()));
// }
//
// #[test]
// fn test_tree_operation_subtree_insert_after_node_when_node_right_is_non_null() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_a.subtree_insert_after(&mut node_g);
//
//     assert_equal!(node_g.parent(), tree.node_a.right());
//
//     // // <TODO>
//     // // // <without cheat>
//     // // tree.node_c = node_g.parent().unwrap().clone();
//     // // // </without cheat>
//     // assert_equal!(node_g.parent(), Some(&tree.node_c));
//     // // </TODO>
// }
//
// #[test]
// fn test_tree_operation_predecessor() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.predecessor(), &tree.node_e);
//     assert_equal!(tree.node_d.predecessor(), &tree.node_f);
//     assert_equal!(tree.node_c.predecessor(), &tree.node_a);
//     assert_equal!(tree.node_e.predecessor(), &tree.node_b);
//     assert_equal!(tree.node_b.predecessor(), &tree.node_d);
// }
//
// #[test]
// fn test_tree_operation_predecessor_of_g_as_right_of_e() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_e.set_right(&mut node_g);
//
//     assert_equal!(node_g.predecessor(), &tree.node_e);
// }
//
// #[test]
// fn test_tree_operation_predecessor_mut() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     assert_equal!(tree.node_a.predecessor_mut(), &mut tree.node_e);
//     assert_equal!(tree.node_d.predecessor_mut(), &mut tree.node_f);
//     assert_equal!(tree.node_c.predecessor_mut(), &mut tree.node_a);
//     assert_equal!(tree.node_e.predecessor_mut(), &mut tree.node_b);
//     assert_equal!(tree.node_b.predecessor_mut(), &mut tree.node_d);
// }
//
// #[test]
// fn test_tree_operation_predecessor_mut_of_g_as_right_of_e() {
//     let mut tree = MitCourseWareTree::initial_state();
//
//     let mut node_g = Node::new(Value::from("G"));
//     tree.node_e.set_right(&mut node_g);
//
//     assert_equal!(node_g.predecessor_mut(), &mut tree.node_e);
// }
//
// #[test]
// fn test_tree_operation_swap_item() {
//     // Given the test tree in its initial state
//     let mut tree = MitCourseWareTree::initial_state();
//
//     // When I swap item of node A with item of node E
//     tree.node_a.swap_item(&mut tree.node_e);
//
//     // Then node A has the value E
//     assert_equal!(tree.node_a.value(), Some(Value::from("E")));
//     // And node E has the value A
//     assert_equal!(tree.node_e.value(), Some(Value::from("A")));
//
//     // And all other nodes remain with their values unmodified
//     assert_equal!(tree.node_b.value(), Some(Value::from("B")));
//     assert_equal!(tree.node_c.value(), Some(Value::from("C")));
//     assert_equal!(tree.node_d.value(), Some(Value::from("D")));
//     assert_equal!(tree.node_f.value(), Some(Value::from("F")));
// }
//
// #[test]
// fn test_tree_operation_subtree_delete_leaf_nodes() {
//     // Given the test tree in its initial state
//     let mut tree = MitCourseWareTree::initial_state();
//
//     // Then node D has 3 references
//     assert_equal!(tree.node_d.refs(), 3);
//     assert_equal!(tree.node_a.refs(), 8);
//     assert_equal!(tree.node_b.refs(), 7);
//     assert_equal!(tree.node_c.refs(), 1);
//     assert_equal!(tree.node_d.refs(), 3);
//     assert_equal!(tree.node_e.refs(), 1);
//
//     // When I subtree_delete node F
//     subtree_delete(&mut tree.node_f);
//
//
//     // Then node F has no more references
//     assert_equal!(tree.node_f.refs(), 0);
//
//     // And node D has no node in its left
//     assert_equal!(tree.node_d.left(), None);
//
//     // And node D has 2 references
//     assert_equal!(tree.node_d.left(), None);
//
//     // And the references of all ancestors of D are decremented
//     assert_equal!(tree.node_a.refs(), 7);
//     assert_equal!(tree.node_b.refs(), 6);
//
//     // And the references of the other leaf nodes remains unchanged
//     assert_equal!(tree.node_c.refs(), 1);
//     assert_equal!(tree.node_e.refs(), 1);
// }
//
// #[test]
// fn test_tree_operation_subtree_delete_root_node() {
//     // Given the test tree in its initial state
//     let mut tree = MitCourseWareTree::initial_state();
//
//     // Then node A has 8 references
//     assert_equal!(tree.node_a.refs(), 8);
//     // And node B is to the left of node A
//     assert_equal!(tree.node_a.left(), Some(tree.node_b.as_ref()));
//     // And node C is to the right of node A
//     assert_equal!(tree.node_a.right(), Some(tree.node_c.as_ref()));
//
//     // When I subtree_delete node A
//     subtree_delete(&mut tree.node_a);
//
//     // Then node A becomes node E
//     assert_equal!(tree.node_e.value(), Some(Value::from("A")));
//
//     // And node A (which has become E) has no more references
//     assert_equal!(tree.node_e.refs(), 0);
//
//     // And node E becomes node A
//     assert_equal!(tree.node_a.value(), Some(Value::from("E")));
//
//     // And node E (which has become A) has 7 references
//     assert_equal!(tree.node_a.refs(), 7);
//
//     // And node B is to the left of node E
//     assert_equal!(tree.node_a.left(), Some(tree.node_b.as_ref()));
//     // And node C is to the right of node E
//     assert_equal!(tree.node_a.right(), Some(tree.node_c.as_ref()));
// }
