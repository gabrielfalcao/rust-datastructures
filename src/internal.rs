use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{Cell, Node, Value};

pub(super) mod null {
    use super::{Cell, Node, Value};
    pub(crate) fn value<'c>() -> *mut Value<'c> {
        std::ptr::null_mut::<Value<'c>>()
    }
    pub(crate) fn cell<'c>() -> *mut Cell<'c> {
        std::ptr::null_mut::<Cell<'c>>()
    }
    pub(crate) fn node<'c>() -> *mut Node<'c> {
        std::ptr::null_mut::<Node<'c>>()
    }
}
pub(super) mod alloc {
    use std::alloc::Layout;

    use super::{Cell, Node, Value};
    unsafe fn new<T>() -> *mut T {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr
        };
        ptr as *mut T
    }
    pub(crate) unsafe fn value<'c>() -> *mut Value<'c> {
        unsafe { self::new::<Value<'c>>() }
    }
    pub(crate) unsafe fn cell<'c>() -> *mut Cell<'c> {
        unsafe { self::new::<Cell<'c>>() }
    }
    pub(crate) unsafe fn node<'c>() -> *mut Node<'c> {
        unsafe { self::new::<Node<'c>>() }
    }
}
pub(super) mod dealloc {
    use std::alloc::Layout;

    use super::{Cell, Node, Value};
    unsafe fn free<T>(ptr: *mut T) {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = ptr as *mut u8;
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("ptr", 231), crate::color::ptr_inv(ptr));
            std::alloc::dealloc(ptr, layout);
        };
    }
    pub(crate) unsafe fn value<'c>(value: *mut Value<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("value", 136), crate::color::ptr_inv(value));
        unsafe { self::free::<Value<'c>>(value) }
    }
    pub(crate) unsafe fn cell<'c>(cell: *mut Cell<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("cell", 137), crate::color::ptr_inv(cell));
        unsafe { self::free::<Cell<'c>>(cell) }
    }
    pub(crate) unsafe fn node<'c>(node: *mut Node<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("node", 28), crate::color::ptr_inv(node));
        unsafe { self::free::<Node<'c>>(node) }
    }
}
