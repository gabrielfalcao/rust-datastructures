use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{car, cdr, color, cons, internal, step, Value};

#[derive(Debug)]
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
        self.item.is_null() && self.left.is_null() && self.right.is_null()
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
            // step!(format!("NON-NULL: {:#?}", parent));
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

    pub fn set_left(&mut self, node: &'c Node<'c>) -> Option<&'c Node<'c>> {
        let mut left = Node::nil();
        let value = node.value() as *const Value<'c>;
        let old = self.left();
        unsafe {
            let item = internal::alloc::value();
            item.write(value.read());
            left.item = item;
            let mut node = internal::alloc::node();
            node.write(left);
            self.left = node;
        }
        old
    }

    pub fn left(&self) -> Option<&'c Node<'c>> {
        if self.left.is_null() {
            None
        } else {
            unsafe { self.left.as_ref() }
        }
    }

    pub fn left_value(&self) -> Value<'c> {
        self.left().map(|node| node.value().clone()).unwrap_or_default()
    }

    pub fn set_right(&mut self, node: &'c Node<'c>) -> Option<&'c Node<'c>> {
        let mut right = Node::nil();
        let value = node.value() as *const Value<'c>;
        let old = self.right();
        unsafe {
            let item = internal::alloc::value();
            item.write(value.read());
            right.item = item;
            let mut node = internal::alloc::node();
            node.write(right);
            self.right = node;
        }
        old
    }

    pub fn right(&self) -> Option<&'c Node<'c>> {
        if self.right.is_null() {
            None
        } else {
            unsafe { self.right.as_ref() }
        }
    }

    pub fn right_value(&self) -> Value<'c> {
        self.right().map(|node| node.value().clone()).unwrap_or_default()
    }

    // ICAgIGZuIGluY3JfcmVmKCZtdXQgc2VsZikgewogICAgICAgIHNlbGYucmVmcyArPSAxOwogICAgICAgIGlmICFzZWxmLnBhcmVudC5pc19udWxsKCkgewogICAgICAgICAgICB1bnNhZmUgewogICAgICAgICAgICAgICAgbGV0IG11dCBwYXJlbnQgPSBzZWxmLnBhcmVudCBhcyAqbXV0IE5vZGU8J2M+OwogICAgICAgICAgICAgICAgaWYgbGV0IFNvbWUobXV0IHBhcmVudCkgPSBwYXJlbnQuYXNfbXV0KCkgewogICAgICAgICAgICAgICAgICAgIHBhcmVudC5yZWZzICs9IDE7CiAgICAgICAgICAgICAgICB9CiAgICAgICAgICAgIH0KICAgICAgICB9CiAgICB9Cg==
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
