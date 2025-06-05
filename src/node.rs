use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::{AsMut, AsRef};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::Pin;
use std::ptr::NonNull;

use crate::{
    cast_node_mut, cast_node_ref, color, decr_ref_nonzero, internal, step, step_test, warn,
    warn_inv, RefCounter, UniquePointer, Value,
};

pub struct Node<'c> {
    pub parent: UniquePointer<'c, Node<'c>>,
    pub left: UniquePointer<'c, Node<'c>>,
    pub right: UniquePointer<'c, Node<'c>>,
    pub item: UniquePointer<'c, Value<'c>>,
    refs: RefCounter,
}

impl<'c> Node<'c> {
    pub fn nil() -> Node<'c> {
        Node {
            parent: UniquePointer::<'c, Node<'c>>::null(),
            left: UniquePointer::<'c, Node<'c>>::null(),
            right: UniquePointer::<'c, Node<'c>>::null(),
            item: UniquePointer::<'c, Value<'c>>::null(),
            refs: RefCounter::new(),
        }
    }

    pub fn is_nil(&self) -> bool {
        self.item.is_null()
            && self.left.is_null()
            && self.right.is_null()
            && self.parent.is_null()
            && self.refs <= 1
    }

    pub fn new(value: Value<'c>) -> Node<'c> {
        let mut node = Node::nil();
        unsafe {
            node.item.write(value);
        }
        node
    }

    pub fn parent(&self) -> Option<&'c Node<'c>> {
        self.parent.as_ref()
    }

    pub fn parent_mut(&mut self) -> Option<&'c mut Node<'c>> {
        self.parent.as_mut()
    }

    pub fn item(&self) -> Value<'c> {
        self.value().unwrap_or_default()
    }

    pub fn id(&self) -> String {
        format!(
            "{}{}",
            if self.item.is_null() {
                format!("Null Node {:p}", self)
            } else {
                format!("Node {}", self.item())
            },
            format!(" ({} referefences)", self.refs)
        )
    }

    pub fn value(&self) -> Option<Value<'c>> {
        if self.item.is_null() {
            None
        } else {
            unsafe {
                if let Some(value) = self.item.as_ref() {
                    Some(value.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn parent_value(&self) -> Option<Value<'c>> {
        if let Some(parent) = self.parent() {
            parent.value()
        } else {
            None
        }
    }

    pub fn set_left(&mut self, left: &mut Node<'c>) {
        self.incr_ref();
        left.parent = self.ptr();
        self.left = left.ptr();
        left.incr_ref();
    }

    pub fn set_right(&mut self, right: &mut Node<'c>) {
        self.incr_ref();
        right.parent = self.ptr();
        self.right = right.ptr();
        right.incr_ref();
    }

    pub fn delete_left(&mut self) {
        if self.left.is_null() {
            return;
        }
        let mut left = self.left.inner_mut();
        left.decr_ref();
        self.left.dealloc(true);
        self.left = UniquePointer::null();
    }

    pub fn left(&self) -> Option<&'c Node<'c>> {
        let left = self.left.as_ref();
        left
    }

    pub fn left_mut(&mut self) -> Option<&'c mut Node<'c>> {
        self.left.as_mut()
    }

    pub fn left_value(&self) -> Option<Value<'c>> {
        if let Some(left) = self.left() {
            left.value()
        } else {
            None
        }
    }

    pub fn delete_right(&mut self) {
        if self.right.is_null() {
            return;
        }
        let mut right = self.right.inner_mut();
        right.decr_ref();
        self.right.dealloc(true);
        self.right = UniquePointer::null();
    }

    pub fn right(&self) -> Option<&'c Node<'c>> {
        self.right.as_ref()
    }

    pub fn right_mut(&mut self) -> Option<&'c mut Node<'c>> {
        self.right.as_mut()
    }

    pub fn right_value(&self) -> Option<Value<'c>> {
        if let Some(right) = self.right() {
            right.value()
        } else {
            None
        }
    }

    pub fn height(&self) -> usize {
        let mut node = self;
        let mut vertices = 0;

        while !node.left.is_null() {
            node = node.left.inner_ref();
            vertices += 1;
        }
        vertices
    }

    pub fn depth(&self) -> usize {
        let mut node = self;
        if self.parent.is_null() {
            return 0;
        }
        let mut vertices = 0;

        while !node.parent.is_null() {
            node = node.parent.inner_ref();
            vertices += 1;
        }
        vertices
    }

    pub fn leaf(&self) -> bool {
        self.left.is_null() && self.right.is_null()
    }

    pub fn addr(&self) -> usize {
        (self as *const Node<'c>).addr()
    }

    pub fn left_addr(&self) -> usize {
        self.left.addr()
    }

    pub fn right_addr(&self) -> usize {
        self.right.addr()
    }

    pub fn parent_addr(&self) -> usize {
        self.parent.addr()
    }

    pub fn refs(&self) -> usize {
        *self.refs
    }

    pub fn subtree_first(&self) -> &'c Node<'c> {
        if self.left.is_null() {
            let node = self as *const Node<'c>;
            return unsafe { node.as_ref().unwrap() };
        }

        let mut subtree_first = self.left.cast_mut();

        loop {
            unsafe {
                let node = &*subtree_first;
                if node.left.is_null() {
                    break;
                }
                subtree_first = node.left.cast_mut()
            }
        }
        unsafe { subtree_first.as_mut().unwrap() }
    }

    pub fn successor(&self) -> &'c Node<'c> {
        if !self.right.is_null() {
            return unsafe { self.right.as_ref().unwrap() }.subtree_first();
        }

        if let Some(parent) = self.parent() {
            /// node.parent is root but node.right is null, so successor is node.subtree_first()
            if parent.parent.is_null() {
                return self.subtree_first();
            }
        }
        let mut successor = self as *const Node<'c>;
        let mut node = unsafe { &*successor };
        loop {
            if node.left() == Some(self) {
                break;
            }
            if !node.parent.is_null() {
                successor = node.parent.cast_const();
                node = unsafe { &*successor };
            } else {
                break;
            };
        }
        unsafe { &*successor }
    }

    pub fn subtree_first_mut(&mut self) -> &'c mut Node<'c> {
        if self.left.is_null() {
            let node = self as *mut Node<'c>;
            return cast_node_mut!(node, noincr);
        }

        let mut subtree_first = &mut self.left;

        loop {
            unsafe {
                let node = subtree_first.inner_mut();
                if node.left.is_null() {
                    break;
                }
                subtree_first = &mut node.left;
            }
        }

        subtree_first.inner_mut()
    }

    pub fn successor_mut(&mut self) -> &'c mut Node<'c> {
        if !self.right.is_null() {
            return self.right.inner_mut().subtree_first_mut();
        }

        if let Some(parent) = self.parent() {
            /// node.parent is root but node.right is null, so successor is node.subtree_first_mut()
            if parent.parent.is_null() {
                return self.subtree_first_mut();
            }
        }
        let mut successor = self as *mut Node<'c>;
        let mut node = cast_node_mut!(successor, noincr);

        loop {
            if node.left() == Some(self) {
                break;
            }
            if !node.parent.is_null() {
                successor = node.parent.cast_mut();
                node = cast_node_mut!(successor, noincr);
            } else {
                break;
            };
        }
        cast_node_mut!(successor, noincr)
    }

    pub fn subtree_insert_after(&mut self, new: &mut Node<'c>) {
        if self.right.is_null() {
            self.set_right(new);
        } else {
            let mut successor = self.successor_mut();
            successor.set_left(new);
        }
    }

    pub fn predecessor(&self) -> &'c Node<'c> {
        let mut predecessor = self as *const Node<'c>;
        let mut node = cast_node_ref!(predecessor);

        loop {
            if !node.left.is_null() {
                predecessor = node.left.cast_const();
                node = cast_node_ref!(predecessor);
                if !node.right.is_null() {
                    predecessor = node.right.cast_const();
                    node = cast_node_ref!(predecessor);
                }
                break;
            } else if !node.parent.is_null() {
                predecessor = node.parent.cast_const();
                node = cast_node_ref!(predecessor);
                if let Some(right) = node.right() {
                    if right == self {
                        break;
                    }
                }
            }
        }
        node = cast_node_ref!(predecessor);
        node
    }

    pub fn predecessor_mut(&mut self) -> &'c mut Node<'c> {
        let mut predecessor = UniquePointer::<'c, Node<'c>>::from_ref_mut(self);
        let mut node = predecessor.inner_mut();

        loop {
            if !node.left.is_null() {
                predecessor = node.left.clone();
                node = predecessor.inner_mut();
                if !node.right.is_null() {
                    predecessor = node.right.clone();
                    node = predecessor.inner_mut();
                }
                break;
            } else if !node.parent.is_null() {
                predecessor = node.parent.clone();
                node = predecessor.inner_mut();

                if let Some(right) = node.right() {
                    if right == self {
                        break;
                    }
                }
            }
        }
        predecessor.inner_mut()
    }

    pub fn disconnect(&mut self) {
        if !self.left.is_null() {
            unsafe {
                let node = self.left.inner_mut();
                decr_ref_nonzero!(node);
            }
        }
        if !self.right.is_null() {
            unsafe {
                let node = self.right.inner_mut();
                decr_ref_nonzero!(node);
            }
        }
        if !self.parent.is_null() {
            unsafe {
                let mut parent = self.parent.inner_mut();
                let delete_left = if let Some(parents_left_child) = parent.left() {
                    parents_left_child == self
                } else {
                    false
                };
                if delete_left {
                    parent.left.dealloc(true);
                    parent.left = UniquePointer::null();
                } else {
                    parent.right.dealloc(true);
                    parent.right = UniquePointer::null();
                }
                parent.decr_ref();
            }
            self.parent.dealloc(true);
            self.parent = UniquePointer::null();
        }
    }

    pub fn dealloc(&mut self) {
        if self.refs > 0 {
            self.decr_ref();
            // if let Some(parent) = self.parent_mut() {
            //     if let Some(node_left) = parent.left_mut() {
            //         if node_left == self {
            //             // step!("delete left of {:#?}", &parent);
            //             parent.delete_left();
            //         }
            //     } else if let Some(node_right) = parent.right_mut() {
            //         if node_right == self {
            //             // step!("delete right of {:#?}", &parent);
            //             parent.delete_right();
            //         }
            //     }
            // }
        } else {
            if !self.parent.is_null() {
                self.parent.dealloc(true);
                self.parent = UniquePointer::null();
            }
            if !self.left.is_null() {
                self.left.dealloc(true);
                self.left = UniquePointer::null();
            }
            if !self.right.is_null() {
                self.right.dealloc(true);
                self.right = UniquePointer::null();
            }
            if !self.item.is_null() {
                self.item.dealloc(false);
                self.item = UniquePointer::null();
            }
        }
    }

    pub fn swap_item(&mut self, other: &mut Node<'c>) {
        self.item = unsafe {
            let item = other.item.propagate();
            other.item = self.item.propagate();
            item
        };
    }
}

pub fn subtree_delete<'c>(node: &mut Node<'c>) {
    if node.leaf() {
        node.decr_ref();
        if node.parent.is_not_null() {
            unsafe {
                let mut parent = node.parent.inner_mut();
                let delete_left = if let Some(parents_left_child) = parent.left() {
                    parents_left_child == node
                } else {
                    false
                };
                if delete_left {
                    parent.left.dealloc(true);
                    parent.left = UniquePointer::null();
                } else {
                    parent.right.dealloc(true);
                    parent.right = UniquePointer::null();
                }
            }
            node.parent.dealloc(true);
            node.parent = UniquePointer::null();
        } else {
            // unreachable!("leaf node {} should have a parent", node);
        }
        node.refs.reset();
        node.parent = UniquePointer::<'c, Node<'c>>::null();
        // node.dealloc();
        return;
    } else {
        let mut predecessor = node.predecessor_mut();
        predecessor.swap_item(node);
        subtree_delete(predecessor);
    }
}

/// Node private methods
impl<'c> Node<'c> {
    pub fn ptr(&self) -> UniquePointer<'c, Node<'c>> {
        let ptr =
            UniquePointer::copy_from_ref(self, *self.refs, UniquePointer::raw_addr_of_ref(self));
        ptr
    }

    fn incr_ref(&mut self) {
        self.refs += 1;
        let mut node = self;
        while !node.parent.is_null() {
            unsafe {
                node = node.parent.peek_mut();
                node.refs += 1;
            }
        }
    }

    fn decr_ref(&mut self) {
        decr_ref_nonzero!(self);
        let mut node = self;
        while !node.parent.is_null() {
            unsafe {
                node = node.parent.inner_mut();
                decr_ref_nonzero!(node);
            }
        }
    }

    fn item_eq(&self, other: &Node<'c>) -> bool {
        if self.item.addr() == other.item.addr() {
            self.item.addr() == other.item.addr()
        } else {
            self.value() == other.value()
        }
    }

    fn left_eq(&self, other: &Node<'c>) -> bool {
        if self.left.addr() == other.left.addr() {
            self.left.addr() == other.left.addr()
        } else {
            self.left_value() == other.left_value()
        }
    }

    fn right_eq(&self, other: &Node<'c>) -> bool {
        if self.right.addr() == other.right.addr() {
            self.right.addr() == other.right.addr()
        } else {
            self.right_value() == other.right_value()
        }
    }

    fn parent_eq(&self, other: &Node<'c>) -> bool {
        if self.parent.addr() == other.parent.addr() {
            self.parent.addr() == other.parent.addr()
        } else {
            self.parent_value() == other.parent_value()
        }
    }
}

impl<'c> PartialEq<Node<'c>> for Node<'c> {
    fn eq(&self, other: &Node<'c>) -> bool {
        if self.item_eq(other) {
            let eq = self.value().unwrap_or_default() == other.value().unwrap_or_default();
            eq
        } else {
            false
        }
    }
}

impl<'c> PartialEq<&mut Node<'c>> for Node<'c> {
    fn eq(&self, other: &&mut Node<'c>) -> bool {
        let other = unsafe { &**other };
        if self.item_eq(other) {
            let eq = self.value().unwrap_or_default() == other.value().unwrap_or_default();
            eq
        } else {
            false
        }
    }
}

// impl<'c> Drop for Node<'c> {
//     fn drop(&mut self) {
//         step!("deallocating {:#?}", self);
//         self.dealloc();
//     }
// }

impl<'c> Clone for Node<'c> {
    fn clone(&self) -> Node<'c> {
        let mut node = Node::nil();
        node.refs = self.refs.clone();
        if self.parent.is_not_null() {
            node.parent = self.parent.clone();
        }
        if self.left.is_not_null() {
            node.left = self.left.clone();
        }
        if self.right.is_not_null() {
            node.right = self.right.clone();
        }
        if !self.item.is_null() {
            node.item = self.item.clone();
        }
        node
    }
}

impl<'c> AsRef<Node<'c>> for Node<'c> {
    fn as_ref(&self) -> &'c Node<'c> {
        cast_node_ref!(self as *const Node<'c>)
    }
}
impl<'c> AsMut<Node<'c>> for Node<'c> {
    fn as_mut(&mut self) -> &'c mut Node<'c> {
        cast_node_mut!(self as *mut Node<'c>, incr)
    }
}
impl<'c> std::fmt::Display for Node<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}
impl<'c> std::fmt::Debug for Node<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::color::reset(
                [
                    crate::color::fg("Node@", 231),
                    format!("{:016x}", self.addr()),
                    format!("[refs={}]", *self.refs),
                    if self.item.is_null() {
                        color::fg("null", 196)
                    } else {
                        format!(
                            "[item={}]",
                            self.value()
                                .map(|value| color::fg(format!("{:#?}", value), 220))
                                .unwrap_or_else(|| format!("empty"))
                        )
                    },
                    if self.parent.is_null() {
                        String::new()
                    } else {
                        format!(
                            "(parent:{})",
                            if self.parent.is_null() {
                                color::fg("null", 196)
                            } else {
                                self.parent_value()
                                    .map(|parent_value| {
                                        color::fg(format!("{:#?}", parent_value), 220)
                                    })
                                    .unwrap_or_else(|| format!("empty"))
                            }
                        )
                    },
                    if self.left.is_null() && self.right.is_null() {
                        String::new()
                    } else {
                        format!(
                            "[left:{} | right:{}]",
                            if self.left.is_null() {
                                color::fg("null", 196)
                            } else {
                                self.left_value()
                                    .map(|left_value| color::fg(format!("{:#?}", left_value), 220))
                                    .unwrap_or_else(|| format!("empty"))
                            },
                            if self.right.is_null() {
                                color::fg("null", 196)
                            } else {
                                self.right_value()
                                    .map(|right_value| {
                                        color::fg(format!("{:#?}", right_value), 220)
                                    })
                                    .unwrap_or_else(|| format!("empty"))
                            }
                        )
                    }
                ]
                .join("")
            )
        )
    }
}
