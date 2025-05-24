use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{car, cdr, color, cons, step, Value};

pub struct Cell<'c> {
    head: *const Value<'c>,
    tail: *const Cell<'c>,
    refs: usize,
}

impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell {
            head: internal::null::value(),
            tail: internal::null::cell(),
            refs: 0,
        }
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        let mut cell = Cell::nil();
        unsafe {
            let head = internal::alloc::value();
            head.write(value);
            cell.head = head;
        }
        cell
    }

    pub fn head(&self) -> Option<Value<'c>> {
        let value = if self.head.is_null() {
            None
        } else {
            let value = unsafe {
                let value = self.head.read();
                value
            };
            Some(value)
        };
        value
    }

    pub fn add(&mut self, mut new: &mut Cell<'c>) {
        new.refs += 1;
        self.refs += 1;
        // crate::step!(format!("\nadding\n->{:#?}\n  to\n    ->{:#?}\n", new, self));
        if self.tail.is_null() {
            unsafe {
                let mut new_tail = std::ptr::from_mut::<Cell<'c>>(new);
                // eprintln!("\ncopying new_tail cell {}\n", crate::color::ptr(new_tail));
                // eprintln!("\nallocating for tail {}\n", crate::color::ptr(self.tail));
                // let new_tail = internal::alloc::cell();
                // let new_tail = new as *const Cell<'c>;
                // new_tail.write(new_tail.read());
                // eprintln!("\nnew tail is {}\n", crate::color::ptr(new_tail));
                self.tail = new_tail;
            }
        } else {
            unsafe {
                let mut tail = &mut *self.tail.cast_mut();
                tail.add(new);
            }
        }
        // crate::step!(format!("\nnew tail\n  ->{:#?}\n    -> {:#?}\n", self, new));
    }

    pub fn pop(&mut self) -> bool {
        if !self.tail.is_null() {
            unsafe {
                self.tail = std::ptr::null::<Cell>();
            }
            true
        } else if !self.head.is_null() {
            self.head = std::ptr::null::<Value>();
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_null() {
            len += 1
        }
        if let Some(tail) = self.tail() {
            len += tail.len();
        }
        len
    }

    pub fn tail(&self) -> Option<&'c Cell<'c>> {
        if self.tail.is_null() {
            None
        } else {
            unsafe {
                if let Some(tail) = self.tail.as_ref() {
                    Some(tail)
                } else {
                    None
                }
            }
        }
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        let mut values = Vec::<Value>::new();
        if let Some(head) = self.head() {
            values.push(head.clone());
        }
        if let Some(tail) = self.tail() {
            values.extend(tail.values());
        }
        values
    }
}

impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(value: Value<'c>) -> Cell<'c> {
        Cell::new(value)
    }
}
impl<'c> From<&'c str> for Cell<'c> {
    fn from(value: &'c str) -> Cell<'c> {
        let value = Value::from(value);
        Cell::new(value)
    }
}
impl<'c> From<u8> for Cell<'c> {
    fn from(value: u8) -> Cell<'c> {
        Cell::new(Value::Byte(value))
    }
}
impl<'c> From<u64> for Cell<'c> {
    fn from(value: u64) -> Cell<'c> {
        if value < u8::MAX.into() {
            Cell::new(Value::Byte(value as u8))
        } else {
            Cell::new(Value::UInt(value))
        }
    }
}
impl<'c> From<i32> for Cell<'c> {
    fn from(value: i32) -> Cell<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            Cell::new(Value::UInt(value))
        } else {
            Cell::new(Value::Int(value.into()))
        }
    }
}
impl<'c> From<i64> for Cell<'c> {
    fn from(value: i64) -> Cell<'c> {
        Cell::new(Value::from(value))
    }
}

impl<'c> PartialEq<Cell<'c>> for Cell<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        if self.head.is_null() == other.head.is_null() {
            true
        } else if let Some(head) = self.head() {
            if let Some(other) = other.head() {
                return head == other;
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'c> Default for Cell<'c> {
    fn default() -> Cell<'c> {
        Cell::nil()
    }
}
impl<'c> Clone for Cell<'c> {
    fn clone(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        unsafe {
            let head = internal::alloc::value();
            head.write(self.head.read());
            let tail = internal::alloc::cell();
            tail.write(self.tail.read());
            cell.head = head;
            cell.tail = tail;
        }
        cell
    }
}
impl<'c> Drop for Cell<'c> {
    fn drop(&mut self) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("dropping ",196),crate::color::fg("cell",49),color::bgfg(format!("@"),231,16),color::ptr_inv(self),color::fore(format!("{:#?}",self),201)),197,16)));

        if self.refs > 0 {
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",crate::color::fg("decrementing refs of ",220),crate::color::fg("cell",49),color::bgfg(format!("@"),231,16),color::ptr_inv(self),color::fore(format!("{:#?}",self),201)),197,16)));
            self.refs -= 1;
        } else {
            unsafe {
                internal::dealloc::value(self.head);
                internal::dealloc::cell(self.tail);
            }
        }
    }
}

impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}[head:{} | tail:{}]",
            crate::color::reset(""),
            crate::color::fg("Cell", 87),
            crate::color::fg("@", 231),
            crate::color::ptr_inv(self),
            if self.head.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.head)
                // color::fore(format!("{:016x}", self.head.addr()), 37)
            },
            if self.tail.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.tail)
                // color::fore(format!("{:016x}", self.tail.addr()), 48)
            },
        )
    }
}

mod internal {
    pub(self) use super::{Cell, Value};
    pub(super) mod null {
        use super::{Cell, Value};
        pub(in crate::cell) fn value<'c>() -> *const Value<'c> {
            std::ptr::null::<Value<'c>>()
        }
        pub(in crate::cell) fn cell<'c>() -> *const Cell<'c> {
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
        pub(in crate::cell) unsafe fn value<'c>() -> *mut Value<'c> {
            unsafe { self::new::<Value<'c>>() }
        }
        pub(in crate::cell) unsafe fn cell<'c>() -> *mut Cell<'c> {
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
        pub(in crate::cell) unsafe fn value<'c>(value: *const Value<'c>) {
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("value", 136), crate::color::ptr_inv(value));
            unsafe { self::free::<Value<'c>>(value) }
        }
        pub(in crate::cell) unsafe fn cell<'c>(cell: *const Cell<'c>) {
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{} {} {}", crate::color::fg("freeing", 9), crate::color::fg("cell", 137), crate::color::ptr_inv(cell));
            unsafe { self::free::<Cell<'c>>(cell) }
        }
    }
}
