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
}

impl<'c> Node<'c> {
    pub fn nil() -> Node<'c> {
        Node {
            parent: internal::null::node(),
            item: internal::null::value(),
            left: internal::null::node(),
            right: internal::null::node(),
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

    pub fn value(&self) -> Value<'c> {
        Value::Nil
    }

    pub fn left(&self) -> Option<&'c Node<'c>> {
        None
    }

    pub fn right(&self) -> Option<&'c Node<'c>> {
        None
    }

    pub fn left_value(&self) -> Value<'c> {
        Value::Nil
    }

    pub fn right_value(&self) -> Value<'c> {
        Value::Nil
    }
}

impl<'c> PartialEq<Node<'c>> for Node<'c> {
    fn eq(&self, other: &Node<'c>) -> bool {
        if self.is_nil() == other.is_nil() {
            true
        } else {
            self.value() == other.value()
                && self.parent.addr() == other.parent.addr()
                && self.left.addr() == other.left.addr()
                && self.right.addr() == other.right.addr()
        }
    }
}
