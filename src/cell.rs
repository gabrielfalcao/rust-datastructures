use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;
use std::str::FromStr;

use crate::color;

use crate::{car, cdr, cons, internal, step, Value};

/// Rust implementation of lisp's cons cell.
pub struct Cell<'c, T: Value + 'c> {
    head: *mut T,
    tail: *mut Cell<'c, T>,
    refs: usize,
}

impl<'c, T: Value + 'c> Cell<'c, T> {
    pub fn nil() -> Cell<'c, T> {
        Cell {
            head: internal::null::ptr::<T>(),
            tail: internal::null::cell(),
            refs: 0,
        }
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_null() && self.tail.is_null()
    }

    pub fn new(value: T) -> Cell<'c, T> {
        let mut cell = Cell::nil();
        unsafe {
            let head = internal::alloc::new::<T>();
            head.write(value);
            cell.head = head;
        }
        cell
    }

    pub fn head(&self) -> Option<T> {
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

    pub fn add(&mut self, mut new: &mut Cell<'c, T>) {
        if self.head.is_null() {
            unsafe {
                if !new.head.is_null() {
                    self.head = internal::alloc::new::<T>();
                    std::ptr::swap(self.head as *mut T, new.head as *mut T);
                }

                if !new.tail.is_null() {
                    let refs = new.refs;
                    let mut tail = new.tail.read();
                    let head = internal::alloc::new::<T>();
                    if !tail.head.is_null() {
                        head.write(tail.head.read());
                    }
                    new.head = head;
                    self.refs = refs;
                }
            }
        } else {
            new.incr_ref();
            self.incr_ref();
            if self.tail.is_null() {
                unsafe {
                    let mut new_tail = std::ptr::from_mut::<Cell<'c, T>>(new);
                    self.tail = new_tail;
                }
            } else {
                unsafe {
                    let mut tail = &mut *self.tail;
                    tail.add(new);
                }
            }
        }
    }

    pub fn pop(&mut self) -> bool {
        if !self.tail.is_null() {
            unsafe {
                self.tail = internal::null::cell();
            }
            true
        } else if !self.head.is_null() {
            self.head = internal::null::ptr::<T>();
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

    pub fn tail(&self) -> Option<&'c Cell<'c, T>> {
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

    pub fn values(&self) -> Vec<T> {
        let mut values = Vec::<T>::new();
        if let Some(head) = self.head() {
            values.push(head.clone());
        }
        if let Some(tail) = self.tail() {
            values.extend(tail.values());
        }
        values
    }

    fn incr_ref(&mut self) {
        self.refs += 1;
        if !self.tail.is_null() {
            unsafe {
                let mut tail = self.tail as *mut Cell<'c, T>;
                if let Some(mut tail) = tail.as_mut() {
                    tail.refs += 1;
                }
            }
        }
    }
}

impl<'c, T: Value + 'c> From<T> for Cell<'c, T> {
    fn from(value: T) -> Cell<'c, T> {
        Cell::new(value)
    }
}
impl<'c, T: Value + 'c> PartialEq<Cell<'c, T>> for Cell<'c, T> {
    fn eq(&self, other: &Cell<'c, T>) -> bool {
        if self.head.is_null() == other.head.is_null() {
            true
        } else if let Some(head) = self.head() {
            if let Some(value) = other.head() {
                return head == value && (self.tail() == other.tail());
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'c, T: Value + 'c> Default for Cell<'c, T> {
    fn default() -> Cell<'c, T> {
        Cell::nil()
    }
}
impl<'c, T: Value + 'c> Clone for Cell<'c, T> {
    fn clone(&self) -> Cell<'c, T> {
        let mut cell = Cell::nil();
        unsafe {
            if !self.head.is_null() {
                let head = internal::alloc::new::<T>();
                head.write(self.head.read());
                cell.head = head;
            }
            if !self.tail.is_null() {
                let tail = internal::alloc::cell();
                tail.write(self.tail.read());
                cell.refs = self.refs;
                cell.tail = tail;
            }
        }
        cell
    }
}
impl<'c, T: Value + 'c> Drop for Cell<'c, T> {
    fn drop(&mut self) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",color::fg("dropping ",196),color::fg("cell",49),color::bgfg(format!("@"),231,16),color::ptr_inv(self),color::fore(format!("{:#?}",self),201)),197,16)));

        if self.refs > 0 {
            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{}",color::reset(color::bgfg(format!("{}{}{}{}:{}",color::fg("decrementing refs of ",220),color::fg("cell",49),color::bgfg(format!("@"),231,16),color::ptr_inv(self),color::fore(format!("{:#?}",self),201)),197,16)));
            self.refs -= 1;
        } else {
            unsafe {
                internal::dealloc::free::<T>(self.head);
                internal::dealloc::cell(self.tail);
            }
        }
    }
}

impl<'c, T: Value + 'c> std::fmt::Debug for Cell<'c, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}[head:{} | tail:{}]",
            color::reset(""),
            color::fg("Cell", 87),
            color::fg("@", 231),
            color::ptr_inv(self),
            if self.head.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.head)
            },
            if self.tail.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.tail)
            },
        )
    }
}
