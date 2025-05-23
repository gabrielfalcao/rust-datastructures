use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

#[rustfmt::skip]
use crate::{color_addr, color_bg, color_bgfg, color_fg, colorize, reset};
use crate::{car, cdr, cons, step, Value};

pub struct Cell<'c> {
    head: *const Value<'c>,
    tail: *const Cell<'c>,
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
            cell.head = std::ptr::from_ref::<Value<'c>>(&value);
        }
        cell
    }

    pub fn head(&self) -> Option<Value<'c>> {
        if self.head.is_null() {
            step!("safe head null");
            None
        } else {
            step!("enter unsafe");
            let value = unsafe {
                step!("within unsafe");
                Some(self.head.read())
            };
            step!("exit unsafe");

            value
        }
    }

    pub fn add(&mut self, new: &Cell<'c>) {
        if self.tail.is_null() {
            step!("tail null: enter unsafe add");
            unsafe {
                step!("tail null: within unsafe");
                let mut new_tail = std::ptr::from_ref::<Cell<'c>>(new);
                self.tail = new_tail;
            }
            step!("tail null: exit unsafe add");
        } else {
            step!("tail NOT NULL: enter unsafe add");
            unsafe {
                step!("tail NOT NULL: within unsafe");
                let mut tail = &mut *self.tail.cast_mut();
                tail.add(new);
            }
            step!("tail NOT NULL: exit unsafe");
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

    pub fn addr(&self) -> String {
        color_addr(self)
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
        Cell::new(Value::from(value))
    }
}
// impl<'c> From<u8> for Cell<'c> {
//     fn from(value: u8) -> Cell<'c> {
//         Cell::new(Value::Byte(value))
//     }
// }
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
            step!();
            true
        } else if let Some(mine) = self.head() {
            step!();
            if let Some(theirs) = other.head() {
                step!();
                return mine == theirs;
            } else {
                step!();
                false
            }
        } else {
            step!();
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
            "\nCell@\x1b[1;38;5;49m{}\x1b[0m[\x1b[1;48;5;{}m\x1b[1;38;5;16m{}\x1b[0m] -> {}\x1b[0m\n",
            &self.addr(),
            match self.head() {
                Some(Value::Nil) => 196,
                Some(Value::Symbol(symbol)) => match symbol.to_string().as_str() {
                    "head" => 136,
                    "tail" => 33,
                    _ => 196,
                },
                Some(Value::UInt(_)) => 39,
                Some(Value::Int(_)) => 74,
                Some(Value::Byte(_)) => 79,
                None => 88
            },
            self.head().map(|head|head.to_string()).unwrap_or_default(),
            {
                let bg = match self.tail.addr() {
                    0 => 16,
                    8 => 232,
                    _ => match self.tail() {
                        Some(_) => 202,
                        None => 54,
                    },
                };
                let fg = match self.tail.addr() {
                    0 => 255,
                    8 => 202,
                    _ => 160,
                };
                format!(
                    "[\x1b[1;48;5;{}mtail:\x1b[1;38;5;{}m{}]",
                    bg,
                    fg,
                    match self.tail() {
                        Some(tail) => {
                            color_addr(tail)
                        },
                        None => {
                            format!("None")
                        },
                    }
                )
            }
        )
    }
}
