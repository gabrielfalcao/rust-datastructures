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

use crate::{Cell, Node, Value};
pub(crate) struct UniquePointer<'c, T> {
    ptr: Pin<&'c mut *mut T>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> UniquePointer<'c, T> {
    pub fn new(mut ptr: *mut T) -> UniquePointer<'c, T> {
        // let mut mut_ptr: &'c mut *mut T = unsafe {&mut *&mut ptr};
        UniquePointer {
            ptr: Pin::new(unsafe { std::mem::transmute::<*mut T, &mut *mut T>(ptr)}),
            _marker: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        // self.ptr.is_null()
        unsafe { self.into_inner() }.is_null()
    }

    pub fn set(&mut self, mut ptr: *mut T) {
        // // self.ptr = ptr;
        // let mut mut_ptr: &'c mut *mut T = unsafe {&mut *&mut ptr};
        self.ptr = Pin::new(unsafe { std::mem::transmute::<*mut T, &mut *mut T>(ptr)});
    }

    pub unsafe fn as_ref(&self) -> Option<&'c T> {
        unsafe { self.cast_const().as_ref() }
    }

    pub unsafe fn as_mut(&self) -> Option<&'c mut T> {
        unsafe { self.cast_mut().as_mut() }
    }

    pub unsafe fn cast_const(&self) -> *const T {
        unsafe { self.into_inner().cast_const() }
    }

    pub unsafe fn cast_mut(&self) -> *mut T {
        unsafe { self.into_inner() }
    }

    pub unsafe fn into_inner(&self) -> *mut T {
        unsafe {
            // let mut mut_ptr: &'c mut *mut T = &mut self.ptr;
            // Pin::<&'c mut *mut T>::into_inner(mut_ptr)
            // let mut ptr = ManuallyDrop::new(&mut ptr);
            // let mut mut_ptr: &'c mut *mut T = ManuallyDrop::<&'c mut *mut T>::into_inner(ptr);
            self.ptr.clone()
            // let ptr = std::mem::transmute::<Pin<&mut *mut T>, Pin<&mut *mut T>>(self.ptr.clone());
            // std::mem::transmute::<&mut *mut T, *mut T>(Pin::into_inner(ptr))
        }
    }

    pub unsafe fn read(&self) -> T {
        unsafe { self.into_inner().read() }
    }

    pub unsafe fn write(&self, value: T) {
        unsafe {
            self.cast_mut().write(value);
        }
    }

    pub fn with_addr(&self, addr: usize) -> *mut T {
        unsafe { self.into_inner().with_addr(addr) }
    }

    pub fn addr(&self) -> usize {
        unsafe { self.into_inner().addr() }
    }
}

impl<'c, T> Deref for UniquePointer<'c, T> {
    type Target = T;

    fn deref(&self) -> &'c T {
        unsafe { self.as_ref().unwrap() }
    }
}
impl<'c, T> DerefMut for UniquePointer<'c, T> {
    fn deref_mut(&mut self) -> &'c mut T {
        unsafe { self.as_mut().unwrap() }
    }
}
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
    pub(crate) unsafe fn node<'c>(node: *const Node<'c>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("node", 28), crate::color::ptr_inv(node));
        unsafe { self::free::<Node<'c>>(node) }
    }
}
