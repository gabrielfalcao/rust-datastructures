use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;
#[rustfmt::skip]
use crate::{color_addr, color_bg, color_bgfg, color_fg, colorize, reset};
use crate::{car, cdr, cons, Value};
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Cell<'c> {
    pub head: Value<'c>,
    pub tail: *const Cell<'c>,
}
impl<'c> Default for Cell<'c> {
    fn default() -> Cell<'c> {
        Cell::nil()
    }
}
impl<'c> Clone for Cell<'c> {
    fn clone(&self) -> Cell<'c> {
        Cell {
            head: self.head.clone(),
            tail: std::ptr::null::<Cell>(),
        }
    }
}
impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell::new(Value::Nil)
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        Cell {
            head: value,
            tail: std::ptr::null::<Cell>(),
        }
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

    pub fn addr(&self) -> String {
        color_addr(self)
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_nil() {
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
        values.push(self.head.clone());
        if let Some(tail) = self.tail() {
            values.extend(tail.values());
        }
        values
    }
}
impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nCell@\x1b[1;38;5;49m{}\x1b[0m[\x1b[1;48;5;{}m\x1b[1;38;5;16m{}\x1b[0m] -> {}\x1b[0m\n",
            &self.addr(),
            match &self.head {
                Value::Nil => 196,
                Value::Symbol(symbol) => match symbol.to_string().as_str() {
                    "head" => 136,
                    "tail" => 33,
                    _ => 196,
                },
            },
            &self.head,
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
#[cfg(test)]
mod cell_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;

    #[test]
    fn test_add_when_tail_is_null() {
        let mut head = Cell::new(Value::from("head"));
        let mut cell = Cell::new(Value::from("cell"));

        assert_equal!(cell.len(), 1);
        assert_equal!(cell.values(), vec![Value::from("cell")]);

        head.add(&cell);

        assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
        assert_equal!(head.len(), 2);

        let mut tail = Cell::new(Value::from("tail"));
        assert_equal!(tail.len(), 1);
        assert_equal!(tail.values(), vec![Value::from("tail")]);

        cell.add(&tail);

        assert_equal!(
            head.values(),
            vec![Value::from("head"), Value::from("cell"), Value::from("tail")]
        );
        assert_equal!(head.len(), 3);
        assert_equal!(cell.values(), vec![Value::from("cell"), Value::from("tail")]);
        assert_equal!(cell.len(), 2);
        assert_equal!(tail.values(), vec![Value::from("tail")]);
        assert_equal!(tail.len(), 1);
    }

    #[test]
    fn test_add_when_tail_is_not_necessarily_null() {
        let mut head = Cell::new(Value::from("head"));
        let mut cell = Cell::new(Value::from("cell"));
        let mut tail = Cell::new(Value::from("tail"));

        assert_equal!(head.values(), vec![Value::from("head")]);
        assert_equal!(head.len(), 1);

        head.add(&cell);
        assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
        assert_equal!(head.len(), 2);

        head.add(&tail);
        assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell"), Value::from("tail")]);
        assert_equal!(head.len(), 3);
    }
}
