use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{Cell, Value};

pub(super) mod null {
    use super::{Cell, Value};
    pub(crate) fn value<'c>() -> *const Value<'c> {
        std::ptr::null::<Value<'c>>()
    }
    pub(crate) fn cell<'c>() -> *const Cell<'c> {
        std::ptr::null::<Cell<'c>>()
    }
}
pub(super) mod alloc {
    use std::alloc::Layout;

    use super::{Cell, Value};
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
}
pub(super) mod dealloc {
    use std::alloc::Layout;

    use super::{Cell, Value};
    unsafe fn free<T>(ptr: *const T) {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = ptr as *mut u8;
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("ptr", 231), crate::color::ptr_inv(ptr));
            std::alloc::dealloc(ptr, layout);
        };
    }
    pub(crate) unsafe fn value<'c>(value: *const Value<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("value", 136), crate::color::ptr_inv(value));
        unsafe { self::free::<Value<'c>>(value) }
    }
    pub(crate) unsafe fn cell<'c>(cell: *const Cell<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("cell", 137), crate::color::ptr_inv(cell));
        unsafe { self::free::<Cell<'c>>(cell) }
    }
}
