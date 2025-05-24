use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
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
}

unsafe fn alloc_value<'c>() -> *mut Value<'c> {
    let layout = Layout::new::<Value<'c>>();
    let ptr = unsafe {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    };
    ptr as *mut Value<'c>
}

impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell {
            head: std::ptr::null::<Value<'c>>(),
            tail: std::ptr::null::<Cell<'c>>(),
        }
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        let mut cell = Cell::nil();
        unsafe {
            let layout = Layout::new::<Value<'c>>();
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            let head = ptr as *mut Value<'c>;
            let layout = Layout::new::<Value<'c>>();
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

    pub fn add(&mut self, new: &Cell<'c>) {
        if self.tail.is_null() {
            unsafe {
                let mut new_tail = std::ptr::from_ref::<Cell<'c>>(new);
                self.tail = new_tail;
            }
        } else {
            unsafe {
                let mut tail = &mut *self.tail.cast_mut();
                tail.add(new);
            }
        }
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
// impl<'c> From<u64> for Cell<'c> {
//     fn from(value: u64) -> Cell<'c> {
//         if value < u8::MAX.into() {
//             Cell::new(Value::Byte(value as u8))
//         } else {
//             Cell::new(Value::UInt(value))
//         }
//     }
// }
// impl<'c> From<i32> for Cell<'c> {
//     fn from(value: i32) -> Cell<'c> {
//         if let Ok(value) = TryInto::<u64>::try_into(value) {
//             Cell::new(Value::UInt(value))
//         } else {
//             Cell::new(Value::Int(value.into()))
//         }
//     }
// }
// impl<'c> From<i64> for Cell<'c> {
//     fn from(value: i64) -> Cell<'c> {
//         Cell::new(Value::from(value))
//     }
// }

impl<'c> PartialEq<Cell<'c>> for Cell<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        if self.head.is_null() == other.head.is_null() {
            true
        } else if let Some(mine) = self.head() {
            if let Some(theirs) = other.head() {
                return mine == theirs;
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
            cell.head.cast_mut().write(self.head.read());
            cell.tail.cast_mut().write(self.tail.read());
        }
        cell
    }
}
impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cell@{}[head:{} | tail:{}]",
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

// impl std::fmt::Debug for Cell<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let head = self.head();
//         write!(
//             f,
//             "\nCell@\x1b[1;38;5;49m{}\x1b[0m[\x1b[1;48;5;{}m\x1b[1;38;5;16m{}\x1b[0m] -> {}\x1b[0m\n",
//             &self.addr(),
//             match &head {
//                 Some(Value::Nil) => 196,
//                 Some(Value::String(symbol)) => match symbol.to_string().as_str() {
//                     "head" => 136,
//                     "tail" => 33,
//                     _ => 196,
//                 },
//                 Some(Value::UInt(_)) => 39,
//                 Some(Value::Int(_)) => 74,
//                 Some(Value::Byte(_)) => 79,
//                 None => 88
//             },
//             head.map(|head|head.to_string()).unwrap_or_default(),
//             {
//                 let bg = match self.tail.addr() {
//                     0 => 16,
//                     8 => 232,
//                     _ => match self.tail() {
//                         Some(_) => 202,
//                         None => 54,
//                     },
//                 };
//                 let fg = match self.tail.addr() {
//                     0 => 255,
//                     8 => 202,
//                     _ => 160,
//                 };
//                 format!(
//                     "[\x1b[1;48;5;{}mtail:\x1b[1;38;5;{}m{}]",
//                     bg,
//                     fg,
//                     match self.tail() {
//                         Some(tail) => {
//                             color::addr(tail)
//                         },
//                         None => {
//                             format!("None")
//                         },
//                     }
//                 )
//             }
//         )
//     }
// }
impl<'c> Drop for Cell<'c> {
    fn drop(&mut self) {
        eprintln!(
            "{}",
            color::reset(color::bgfg(
                format!(
                    "{}{} {}{}: {}",
                    crate::color::fg("dropping ", 237),
                    crate::color::fg("cell", 136),
                    color::bgfg(format!(" @ "), 231, 16),
                    color::ptr_inv(self),
                    color::fore(format!("{:#?}", self), 201),
                ),
                197,
                16,
            ))
        )
    }
}
