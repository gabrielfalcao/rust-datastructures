use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

use crate::{car, cdr, cons, Value};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Cell<'c> {
    pub head: Value<'c>,
    pub tail: *mut Cell<'c>,
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
            tail: std::ptr::null_mut::<Cell>(),
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
            tail: std::ptr::null_mut::<Cell>(),
        }
    }

    pub fn add_value(&mut self, value: Value<'c>) {
        if self.tail.is_null() {
            let mut new = Cell::new(value);
            unsafe {
                // self.tail = std::ptr::dangling_mut::<Cell>();
                // dbg!(&self, &new);
                let mut new_tail = std::ptr::from_mut::<Cell<'c>>(&mut new);
                dbg!(&new_tail, &self, &new);
                self.tail = new_tail;
                dbg!(&self, &self.tail());
            }
            match self.tail() {
                Some(tail) => {
                    dbg!(&tail, &self);
                },
                None => {
                    dbg!(&self);
                },
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
        // if self.tail.is_null() {
        //     None
        // } else {
        //     Some(Cell::new(Value::Symbol(Cow::from("stub"))))
        // }

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
                            format!("{:p}", &tail)
                            // format!(
                            //     "\x1b[1;38;5;{}mCell({}\x1b[1;38;5;{}m)",
                            //     fg,
                            //         format!(
                            //             "\x1b[1;38;5;49m{:p}[\x1b[1;48;5;16m\x1b[1;38;5;220m{}]",
                            //                 &tail, &tail.head), fg)
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
    // #[test]
    // fn test_nil() {
    //     let cell = Cell::nil();
    //     assert_equal!(cell.len(), 0);
    //     assert_equal!(cell.values(), vec![Value::Nil]);
    // }

    // #[test]
    // fn test_new() {
    //     let cell = Cell::new(Value::from("head"));
    //     assert_equal!(cell.len(), 1);
    //     assert_equal!(cell.values(), vec![Value::from("head")]);
    // }
    #[test]
    fn test_add() {
        let mut cell = Cell::new(Value::from("head"));
        cell.add_value(Value::from("tail"));
        assert_equal!(cell.len(), 2);
        assert_equal!(cell.values(), vec![Value::from("head"), Value::from("tail")]);
    }
}

// #[cfg(test)]
// mod cell_tests {
//     use std::rc::Rc;

//     use k9::assert_equal;

//     use crate::*;

//     #[test]
//     fn from_value_nil() {
//         assert_equal!(
//             Cell::from(Value::Nil),
//             Cell {
//                 head: Value::Nil,
//                 tail: None
//             }
//         );
//     }
//     #[test]
//     fn from_value_symbol() {
//         assert_equal!(
//             Cell::from(Value::from("symbol")),
//             Cell {
//                 head: Value::from("symbol"),
//                 tail: None
//             }
//         );
//     }
//     #[test]
//     fn nil() {
//         assert_equal!(Cell::nil(), Cell::from(Value::Nil));
//         assert_equal!(Cell::nil().split_string(), [None, None]);
//         // assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
//         // assert_display_equal!(Cell::nil(), "()");
//     }
//     #[test]
//     fn from_cell_nil() {
//         assert_equal!(Cell::nil(), Cell::from(Cell::nil()));
//         assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
//         assert_display_equal!(Cell::nil(), "()");
//     }
//     #[test]
//     fn from_cell_head_and_tail_with_head_symbol_tail_nil() {
//         assert_equal!(
//             Cell {
//                 head: Value::from("head"),
//                 tail: Some(Rc::new(Cell {
//                     head: Value::from("tail"),
//                     tail: None
//                 }))
//             }
//             .split_string(),
//             [Some("head".to_string()), Some("tail".to_string()),]
//         );
//         assert_display_equal!(
//             Cell {
//                 head: Value::from("head"),
//                 tail: Some(Rc::new(Cell {
//                     head: Value::from("tail"),
//                     tail: None
//                 }))
//             },
//             "(head tail)"
//         );
//         assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
//     }
//     #[test]
//     fn from_cell_head_and_tail_with_head_nil_tail_head_symbol() {
//         assert_equal!(
//             Cell {
//                 head: Value::from("head"),
//                 tail: Some(Rc::new(Cell {
//                     head: Value::Nil,
//                     tail: Some(Rc::new(Cell {
//                         head: Value::from("tail"),
//                         tail: None
//                     }))
//                 }))
//             }
//             .split_string(),
//             [Some("head".to_string()), Some("tail".to_string()),]
//         );
//         assert_display_equal!(
//             Cell {
//                 head: Value::from("head"),
//                 tail: Some(Rc::new(Cell {
//                     head: Value::Nil,
//                     tail: Some(Rc::new(Cell {
//                         head: Value::from("tail"),
//                         tail: None
//                     }))
//                 }))
//             },
//             "(head tail)"
//         );
//         assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
//     }
// }

pub fn color_addr<T: Sized>(t: &T) -> String {
    let addr = std::ptr::from_ref(t).addr();
    let fg = if addr > 0 {
        addr % 255
    } else {
        16
    };
    let bg = match fg {
        0 | 8 | 16 .. 24 | 232..237 => {
            255
        },
        _ => {
            16
        }
    };
    format!("\x1b[1;48;5;{}m\x1b[1;38;5;{}m{}\x1b[0m", bg, fg, addr)
}
