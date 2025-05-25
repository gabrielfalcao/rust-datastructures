use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::pin::Pin;
use std::ptr::NonNull;

use crate::{color, internal, step, Value};

pub struct Node<'c> {
    parent: *const Node<'c>,
    item: *const Value<'c>,
    left: *const Node<'c>,
    right: *const Node<'c>,
    refs: usize,
}

impl<'c> Node<'c> {
    pub fn nil() -> Node<'c> {
        Node {
            parent: internal::null::node(),
            item: internal::null::value(),
            left: internal::null::node(),
            right: internal::null::node(),
            refs: 0,
        }
    }

    pub fn is_nil(&self) -> bool {
        self.item.is_null() && self.left.is_null() && self.right.is_null() && self.parent.is_null()
    }

    pub fn new(value: Value<'c>) -> Node<'c> {
        let mut node = Node::nil();
        unsafe {
            let item = internal::alloc::value();
            item.write(value);
            node.item = item;
        }
        node
    }

    pub fn parent(&self) -> &'c Node<'c> {
        if self.parent.is_null() {
            let parent = Node::nil();
            let ptr = &parent as *const Node<'c>;
            let parent = unsafe { ptr.as_ref() }.unwrap();
            parent
        } else {
            let parent = unsafe { self.parent.as_ref() }.unwrap();
            parent
        }
    }

    pub fn value(&self) -> &'c Value<'c> {
        if let Some(ptr) = unsafe { self.item.as_ref() } {
            ptr
        } else {
            let value = Value::Nil;
            let ptr = &value as *const Value<'c>;
            unsafe { ptr.as_ref() }.unwrap()
        }
    }

    pub fn set_left(&mut self, node: &'c mut Node<'c>) -> &'c Node<'c> {
        let mut left = Node::nil();
        let value = node.value() as *const Value<'c>;
        unsafe {
            let item = internal::alloc::value();
            item.write(value.read());
            left.item = item;
            left.set_parent(self);
            let mut node = internal::alloc::node();
            let left_ref = &left as *const Node<'c>;
            node.write(left);
            self.left = node;
            left_ref.as_ref().unwrap()
        }
    }

    pub fn left(&self) -> &'c Node<'c> {
        if self.left.is_null() {
            let left = Node::nil();
            let ptr = &left as *const Node<'c>;
            let left = unsafe { ptr.as_ref() }.unwrap();
            left
        } else {
            let left = unsafe { self.left.as_ref() }.unwrap();
            left
        }
    }

    pub fn left_value(&self) -> Value<'c> {
        if self.left.is_null() {
            Value::Nil
        } else {
            self.left().value().clone()
        }
    }

    pub fn set_right(&mut self, node: &'c mut Node<'c>) -> &'c Node<'c> {
        let mut right = Node::nil();
        let value = node.value() as *const Value<'c>;
        unsafe {
            let item = internal::alloc::value();
            item.write(value.read());
            right.item = item;
            right.set_parent(self);
            let mut node = internal::alloc::node();
            let right_ref = &right as *const Node<'c>;
            node.write(right);
            self.right = node;
            right_ref.as_ref().unwrap()
        }
    }

    pub fn right(&self) -> &'c Node<'c> {
        if self.right.is_null() {
            let right = Node::nil();
            let ptr = &right as *const Node<'c>;
            let right = unsafe { ptr.as_ref() }.unwrap();
            right
        } else {
            let right = unsafe { self.right.as_ref() }.unwrap();
            right
        }
    }

    pub fn right_value(&self) -> Value<'c> {
        if self.right.is_null() {
            Value::Nil
        } else {
            self.right().value().clone()
        }
    }

    fn set_parent(&mut self, parent: *const Node<'c>) {
        if !self.parent.is_null() {}
        self.parent = parent;
        self.incr_ref();
    }

    fn incr_ref(&mut self) {
        self.refs += 1;
        if !self.parent.is_null() {
            unsafe {
                let mut parent = self.parent as *mut Node<'c>;
                if let Some(mut parent) = parent.as_mut() {
                    parent.refs += 1;
                }
            }
        }
    }
}

impl<'c> PartialEq<Node<'c>> for Node<'c> {
    fn eq(&self, other: &Node<'c>) -> bool {
        if self.is_nil() == other.is_nil() {
            true
        } else {
            dbg!(self.value() == other.value())
                && dbg!(self.parent.addr() == other.parent.addr())
                && self.left.addr() == other.left.addr()
                && self.right.addr() == other.right.addr()
        }
    }
}

impl<'c> Clone for Node<'c> {
    fn clone(&self) -> Node<'c> {
        let mut node = Node::nil();
        node.refs = self.refs;
        unsafe {
            if !self.item.is_null() {
                let item = internal::alloc::value();
                item.write(self.item.read());
                node.item = item;
            }
            if !self.parent.is_null() {
                let parent = internal::alloc::node();
                parent.write(self.parent.read());
                node.parent = parent;
            }
            if !self.left.is_null() {
                let left = internal::alloc::node();
                left.write(self.left.read());
                node.left = left;
            }
            if !self.right.is_null() {
                let right = internal::alloc::node();
                right.write(self.right.read());
                node.right = right;
            }
        }
        node
    }
}

impl std::fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}(parent:{})[left:{} | right:{}][item={}]",
            crate::color::reset(""),
            crate::color::fg("Node", 87),
            crate::color::fg("@", 231),
            crate::color::ptr(self),
            crate::color::ptr(self.parent),
            crate::color::ptr(self.left),
            crate::color::ptr(self.right),
            if self.item.is_null() {
                color::fore("null", 196)
            } else {
                color::fore(format!("{:#?}", self.value()), 220)
            },
        )
    }
}

// impl<'c> Drop for Node<'c> {
//     fn drop(&mut self) {
//         #[rustfmt::skip]//#[cfg(feature="debug")]
//         eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("dropping ",196),crate::color::fg("node",49),color::bgfg(format!("@"),231,16),color::ptr(self),color::fore(format!("{:#?}",self),201)),197,16)));
//
//         if self.refs > 0 {
//             #[rustfmt::skip]//#[cfg(feature="debug")]
//             eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("decrementing refs of ",220),crate::color::fg("node",49),color::bgfg(format!("@"),231,16),color::ptr(self),color::fore(format!("{:#?}",self),201)),197,16)));
//             self.refs -= 1;
//         } else {
//             unsafe {
//                 internal::dealloc::value(self.item);
//                 internal::dealloc::node(self.parent);
//                 internal::dealloc::node(self.left);
//                 internal::dealloc::node(self.right);
//             }
//         }
//     }
// }
//
