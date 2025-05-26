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
        self.item.is_null()
            && self.left.is_null()
            && self.right.is_null()
            && self.parent.is_null()
            && self.refs == 0
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

    pub fn parent(&self) -> Option<&'c Node<'c>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { self.parent.as_ref() }
        }
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
        assert!(self.left.is_null());
        assert_ne!((left as *const Node<'c>).addr(), (self as *const Node<'c>).addr());
        // step!(color::fore( format!("setting left of {} to {}", color::node_ptr(self), color::node_ptr(left)), 112 ));
        left.set_parent(self);
        self.refs += 1;
        // unsafe {
        //     let mut node = internal::alloc::node();
        //     node.write((left as *const Node<'c>).read());
        //     assert_eq!(left.parent.addr(), (self as *const Node<'c>).addr());
        //     self.left = node;
        // }
        self.left = self.left.with_addr(left.addr());
        assert!(self.left_addr() == left.addr());
        // step!(color::fore(format!("set: left of {}", color::node_ptr(self)), 81));
    }

    pub fn left(&self) -> Option<&'c Node<'c>> {
        if self.left.is_null() {
            None
        } else {
            unsafe { self.left.as_ref() }
        }
    }

    pub fn left_value(&self) -> Option<Value<'c>> {
        if let Some(left) = self.left() {
            left.value()
        } else {
            None
        }
    }

    pub fn set_right(&mut self, right: &mut Node<'c>) {
        assert!(self.right.is_null());
        assert_ne!((right as *const Node<'c>).addr(), (self as *const Node<'c>).addr());
        // step!(color::fore( format!("setting right of {} to {}", color::node_ptr(self), color::node_ptr(right)), 112 ));
        right.set_parent(self);
        self.refs += 1;
        // unsafe {
        //     let mut node = internal::alloc::node();
        //     node.write((right as *const Node<'c>).read());
        //     assert_eq!(right.parent.addr(), (self as *const Node<'c>).addr());
        //     self.right = node;
        // }
        self.right = self.right.with_addr(right.addr());
        assert!(self.right_addr() == right.addr());
        // step!(color::fore(format!("set: right of {}", color::node_ptr(self)), 81));
    }

    pub fn right(&self) -> Option<&'c Node<'c>> {
        if self.right.is_null() {
            None
        } else {
            unsafe { self.right.as_ref() }
        }
    }

    pub fn right_value(&self) -> Option<Value<'c>> {
        if let Some(right) = self.right() {
            right.value()
        } else {
            None
        }
    }

    // binary tree "properties"
    pub fn height(&self) -> usize {
        let mut node = self;
        if self.left.is_null() {
            return 0;
        }
        // dbg!(&node);
        let mut vertices = 0;
        // step!(format!("self = {:#?}", self.value().unwrap_or_default()));

        while !node.left.is_null() {
            node = unsafe { node.left.as_ref().unwrap() };
            vertices += 1;
            // dbg!(&node, node == self);
        }
        // step!(format!(
        //     "node[{:#?}] == self[{:#?}]: {}",
        //     node.value().unwrap_or_default(),
        //     self.value().unwrap_or_default(),
        //     node == self
        // ));
        vertices
    }

    // binary tree "properties"
    pub fn depth(&self) -> usize {
        if self.parent().is_none() {
            return 0;
        }
        let mut depth = 0;
        let mut parent = self.parent();
        while parent.is_some() {
            depth += 1;
            parent = parent.unwrap().parent();
        }
        depth
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
}

/// Node private methods
impl<'c> Node<'c> {
    fn set_parent(&mut self, parent: &Node<'c>) {
        assert!(self.parent.is_null());
        // step!(color::fore( format!("setting parent of {} to {}", color::node_ptr(self), color::node_ptr(parent)), 178 ));
        self.parent = self.parent.with_addr(parent.addr());
        self.refs += 1;
        // step!(color::fore(format!("set: parent of {}", color::node_ptr(self)), 81));
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

    fn item_eq(&self, other: &Node<'c>) -> bool {
        if self.item.addr() == other.item.addr() {
            self.item.addr() == other.item.addr()
        } else {
            // step!("{} != {}", color::ptr_inv(self.item), color::ptr_inv(other.item));
            self.value() == other.value()
        }
    }

    fn left_eq(&self, other: &Node<'c>) -> bool {
        if self.left.addr() == other.left.addr() {
            self.left.addr() == other.left.addr()
        } else {
            // step!("{} != {}", color::ptr_inv(self.left), color::ptr_inv(other.left));
            self.left_value() == other.left_value()
        }
    }

    fn right_eq(&self, other: &Node<'c>) -> bool {
        if self.right.addr() == other.right.addr() {
            self.right.addr() == other.right.addr()
        } else {
            // step!("{} != {}", color::ptr_inv(self.right), color::ptr_inv(other.right));
            self.right_value() == other.right_value()
        }
    }

    fn parent_eq(&self, other: &Node<'c>) -> bool {
        if self.parent.addr() == other.parent.addr() {
            self.parent.addr() == other.parent.addr()
        } else {
            // step!("{} != {}", color::ptr_inv(self.parent), color::ptr_inv(other.parent));
            self.parent_value() == other.parent_value()
        }
    }

    fn refs_eq(&self, other: &Node<'c>) -> bool {
        if self.refs == other.refs {
            self.refs == other.refs
        } else {
            eprintln!("");
            dbg!(self.refs) == dbg!(other.refs)
        }
    }
}

impl<'c> PartialEq<Node<'c>> for Node<'c> {
    fn eq(&self, other: &Node<'c>) -> bool {
        if self.parent_eq(other)
            && self.item_eq(other)
            && self.left_eq(other)
            && self.right_eq(other)
        {
            self.value().unwrap_or_default() == other.value().unwrap_or_default()
                && self.parent_value() == other.parent_value()
                && self.left_value() == other.left_value()
                && self.right_value() == other.right_value()
        } else {
            false
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
            "{}",
            [
                crate::color::fore("Node", 231),
                // crate::color::fg("@", 231),
                // crate::color::ptr_inv(self),
                if self.item.is_null() {
                    color::fore("null", 196)
                } else {
                    format!(
                        "[item={}]",
                        self.value()
                            .map(|value| color::fore(format!("{:#?}", value), 220))
                            .unwrap_or_else(|| format!("empty"))
                    )
                },
                if self.parent.is_null() {
                    String::new()
                } else {
                    format!(
                        "(parent:{})",
                        if self.parent.is_null() {
                            color::fore("null", 196)
                        } else {
                            self.parent_value()
                                .map(|parent_value| {
                                    color::fore(format!("{:#?}", parent_value), 220)
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
                            color::fore("null", 196)
                        } else {
                            self.left_value()
                                .map(|left_value| color::fore(format!("{:#?}", left_value), 220))
                                .unwrap_or_else(|| format!("empty"))
                        },
                        if self.right.is_null() {
                            color::fore("null", 196)
                        } else {
                            self.right_value()
                                .map(|right_value| color::fore(format!("{:#?}", right_value), 220))
                                .unwrap_or_else(|| format!("empty"))
                        }
                    )
                }
            ]
            .join("")
        )
    }
}
impl<'c> Drop for Node<'c> {
    fn drop(&mut self) {
        // #[rustfmt::skip]//#[cfg(feature="debug")]
        // eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("dropping ",196),crate::color::fg("node",49),color::bgfg(format!("@"),231,16),color::ptr(self),color::fore(format!("{:#?}",self),201)),197,16)));

        if self.refs > 0 {
            // #[rustfmt::skip]//#[cfg(feature="debug")]
            // eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("decrementing refs of ",220),crate::color::fg("node",49),color::bgfg(format!("@"),231,16),color::ptr(self),color::fore(format!("{:#?}",self),201)),197,16)));
            self.refs -= 1;
        } else {
            unsafe {
                internal::dealloc::value(self.item);
                internal::dealloc::node(self.parent);
                internal::dealloc::node(self.left);
                internal::dealloc::node(self.right);
            }
        }
    }
}

// aW1wbCBzdGQ6OmZtdDo6RGVidWcgZm9yIE5vZGU8J18+IHsKICAgIGZuIGZtdCgmc2VsZiwgZjogJm11dCBzdGQ6OmZtdDo6Rm9ybWF0dGVyKSAtPiBzdGQ6OmZtdDo6UmVzdWx0IHsKICAgICAgICB3cml0ZSEoCiAgICAgICAgICAgIGYsCiAgICAgICAgICAgICJ7fXt9W2l0ZW09e31de317fShwYXJlbnQ6e30pW2xlZnQ6e30gfCByaWdodDp7fV0iLAogICAgICAgICAgICBjcmF0ZTo6Y29sb3I6OnJlc2V0KCIiKSwKICAgICAgICAgICAgY3JhdGU6OmNvbG9yOjpmb3JlKCJOb2RlIiwgMjMxKSwKICAgICAgICAgICAgaWYgc2VsZi5pdGVtLmlzX251bGwoKSB7CiAgICAgICAgICAgICAgICBjb2xvcjo6Zm9yZSgibnVsbCIsIDE5NikKICAgICAgICAgICAgfSBlbHNlIHsKICAgICAgICAgICAgICAgIHNlbGYudmFsdWUoKQogICAgICAgICAgICAgICAgICAgIC5tYXAofHZhbHVlfCBjb2xvcjo6Zm9yZShmb3JtYXQhKCJ7OiM/fSIsIHZhbHVlKSwgMjIwKSkKICAgICAgICAgICAgICAgICAgICAudW53cmFwX29yX2Vsc2UofHwgZm9ybWF0ISgiZW1wdHkiKSkKICAgICAgICAgICAgfSwKICAgICAgICAgICAgY3JhdGU6OmNvbG9yOjpmZygiQCIsIDIzMSksCiAgICAgICAgICAgIGNyYXRlOjpjb2xvcjo6cHRyX2ludihzZWxmKSwKICAgICAgICAgICAgaWYgc2VsZi5wYXJlbnQuaXNfbnVsbCgpIHsKICAgICAgICAgICAgICAgIGNvbG9yOjpmb3JlKCJudWxsIiwgMTk2KQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgc2VsZi5wYXJlbnRfdmFsdWUoKQogICAgICAgICAgICAgICAgICAgIC5tYXAofHBhcmVudF92YWx1ZXwgY29sb3I6OmZvcmUoZm9ybWF0ISgiezojP30iLCBwYXJlbnRfdmFsdWUpLCAyMjApKQogICAgICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZWxzZSh8fCBmb3JtYXQhKCJlbXB0eSIpKQogICAgICAgICAgICB9LAogICAgICAgICAgICBpZiBzZWxmLmxlZnQuaXNfbnVsbCgpIHsKICAgICAgICAgICAgICAgIGNvbG9yOjpmb3JlKCJudWxsIiwgMTk2KQogICAgICAgICAgICB9IGVsc2UgewogICAgICAgICAgICAgICAgc2VsZi5sZWZ0X3ZhbHVlKCkKICAgICAgICAgICAgICAgICAgICAubWFwKHxsZWZ0X3ZhbHVlfCBjb2xvcjo6Zm9yZShmb3JtYXQhKCJ7OiM/fSIsIGxlZnRfdmFsdWUpLCAyMjApKQogICAgICAgICAgICAgICAgICAgIC51bndyYXBfb3JfZWxzZSh8fCBmb3JtYXQhKCJlbXB0eSIpKQogICAgICAgICAgICB9LAogICAgICAgICAgICBpZiBzZWxmLnJpZ2h0LmlzX251bGwoKSB7CiAgICAgICAgICAgICAgICBjb2xvcjo6Zm9yZSgibnVsbCIsIDE5NikKICAgICAgICAgICAgfSBlbHNlIHsKICAgICAgICAgICAgICAgIHNlbGYucmlnaHRfdmFsdWUoKQogICAgICAgICAgICAgICAgICAgIC5tYXAofHJpZ2h0X3ZhbHVlfCBjb2xvcjo6Zm9yZShmb3JtYXQhKCJ7OiM/fSIsIHJpZ2h0X3ZhbHVlKSwgMjIwKSkKICAgICAgICAgICAgICAgICAgICAudW53cmFwX29yX2Vsc2UofHwgZm9ybWF0ISgiZW1wdHkiKSkKICAgICAgICAgICAgfSwKICAgICAgICApCiAgICB9Cn0K
