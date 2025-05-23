use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;
#[rustfmt::skip]
use crate::{color_addr, color_bg, color_bgfg, color_fg, colorize, reset};
use crate::{car, cdr, cons, Value};
#[derive(PartialOrd, Ord, PartialEq, Eq, Copy)]
pub struct Cell<'c> {
    head: *const Value<'c>,
    tail: *const Cell<'c>,
}
// impl<'c> Drop for Cell<'c> {
//     fn drop(&mut self) {
//         std::mem::drop(self.tail);
//     }
// }
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
impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell {
            head: std::ptr::null::<Value<'c>>(),
            tail: std::ptr::null::<Cell<'c>>(),
        }
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        crate::step!("new");
        unsafe {
            let mut cell = Cell::nil();
            cell.head.cast_mut().write(value);
        }
        crate::step!("new");

        cell
    }

    pub fn head(&self) -> Option<&'c Value<'c>> {
        // crate::step!("head");
        dbg!(self.head.addr());
        if self.head.is_null() {
            crate::step!("head");
            None
        } else {
            // crate::step!("head");
            unsafe {
                // crate::step!("head");
                dbg!(self.head.addr());
                if let Some(head) = self.head.as_ref() {
                    // crate::step!("head");
                    Some(head)
                } else {
                    // crate::step!("head");
                    None
                }
            }
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

    pub fn pop(&mut self) -> bool {
        // if !self.tail.is_null() {
        //     unsafe {
        //         // self.as_mut().drop_in_place();
        //         self.tail = std::ptr::null::<Cell>();
        //     }
        //     true
        // } else if !self.head.is_null() {
        //     self.head = std::ptr::null::<Value>();
        //     true
        // } else {
        //     false
        // }
        true
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
            // crate::step!("values");
            values.push(head.clone());
            crate::step!("values");
        }
        if let Some(tail) = self.tail() {
            // crate::step!("values");
            values.extend(tail.values());
            crate::step!("values");
        }
        crate::step!("values");
        values
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
#[cfg(test)]
mod cell_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;

    #[test]
    fn test_cell_head() {
        let cell = Cell::new(Value::from("head"));

        assert_equal!(cell.head(), Some(&Value::from("head")));
    }
    // #[test]
    // fn test_add_when_tail_is_null() {
    //     let mut head = Cell::new(Value::from("head"));
    //     let mut cell = Cell::new(Value::from("cell"));

    //     assert_equal!(cell.len(), 1);
    //     assert_equal!(cell.values(), vec![Value::from("cell")]);
    //     crate::step!();

    //     head.add(&cell);

    //     assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    //     assert_equal!(head.len(), 2);
    //     crate::step!();

    //     let mut tail = Cell::new(Value::from("tail"));
    //     assert_equal!(tail.len(), 1);
    //     assert_equal!(tail.values(), vec![Value::from("tail")]);
    //     crate::step!();

    //     cell.add(&tail);

    //     assert_equal!(
    //         head.values(),
    //         vec![Value::from("head"), Value::from("cell"), Value::from("tail")]
    //     );
    //     assert_equal!(head.len(), 3);
    //     assert_equal!(cell.values(), vec![Value::from("cell"), Value::from("tail")]);
    //     assert_equal!(cell.len(), 2);
    //     assert_equal!(tail.values(), vec![Value::from("tail")]);
    //     assert_equal!(tail.len(), 1);
    //     crate::step!();
    // }

    // #[test]
    // fn test_add_when_tail_is_not_necessarily_null() {
    //     let mut head = Cell::new(Value::from("head"));
    //     let mut cell = Cell::new(Value::from("cell"));
    //     let mut tail = Cell::new(Value::from("tail"));

    //     assert_equal!(head.values(), vec![Value::from("head")]);
    //     assert_equal!(head.len(), 1);

    //     head.add(&cell);
    //     assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    //     assert_equal!(head.len(), 2);

    //     head.add(&tail);
    //     assert_equal!(
    //         head.values(),
    //         vec![Value::from("head"), Value::from("cell"), Value::from("tail")]
    //     );
    //     assert_equal!(head.len(), 3);
    // }
    // #[test]
    // fn test_add_and_pop() {
    //     let mut head = Cell::new(Value::from("head"));
    //     let mut cell = Cell::new(Value::from("cell"));

    //     crate::step!();
    //     assert_equal!(head.len(), 1);
    //     crate::step!();
    //     assert_equal!(head.values(), vec![Value::from("head")]);
    //     crate::step!();

    //     head.add(&cell);
    //     crate::step!();

    //     assert_equal!(head.values(), vec![Value::from("head"), Value::from("cell")]);
    //     assert_equal!(head.len(), 2);
    //     crate::step!();

    //     assert_equal!(head.pop(), true);
    //     crate::step!();

    //     assert_equal!(head.values(), vec![Value::from("head")]);
    //     assert_equal!(head.len(), 1);
    //     crate::step!();

    //     assert_equal!(head.pop(), true);
    //     crate::step!();

    //     assert_equal!(head.values(), vec![]);
    //     assert_equal!(head.len(), 0);
    //     crate::step!();

    //     assert_equal!(head.pop(), false);
    //     assert_equal!(head.values(), vec![]);
    //     assert_equal!(head.len(), 0);
    //     crate::step!();
    // }
}
