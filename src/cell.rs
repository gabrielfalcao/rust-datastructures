use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

use crate::{car, cdr, cons, Value};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Cell<'c> {
    pub head: Value<'c>,
    pub tail: *mut Cell<'c>,
}
impl<'c> Default for Cell<'c> {
    fn default() -> Cell<'c> {
        Cell::nil()
    }
}
impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell::new(Value::Nil)
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        Cell {
            head: value,
            tail: core::ptr::null_mut::<Cell>(),
        }
    }

    pub fn add_value(&mut self, value: Value<'c>) {
        dbg!(&self);
        if self.tail.is_null() {
            let mut new = Cell::new(value);
            let addr = format!("{:p}", &new);
            dbg!(&new, addr);
            unsafe {
                // let layout = Layout::for_value(&new);
                // let ptr = alloc(layout);
                // if ptr.is_null() {
                //     std::alloc::handle_alloc_error(layout);
                // }
                let mut cell = std::ptr::from_mut::<Cell<'c>>(&mut new);
                // dbg!(&cell);
                // let old_v = std::mem::take(&mut new);
                // dbg!(&old_v);
                // dbg!(&self, &cell);
                dbg!(&self, &cell);
                self.tail = std::ptr::dangling_mut::<Cell>();
                dbg!(&self, &cell);
                // std::ptr::replace(self.tail, new);
                // std::ptr::swap(self.tail, cell);
                // std::ptr::write(self.tail, Cell::nil());
                // dbg!(&self, &cell);
                // dbg!(&self, &cell);
                // let mut cell = core::ptr::from_ref::<Cell>(&new);
                std::ptr::copy::<Cell<'c>>(cell, self.tail, 1);
                dbg!(&self, &cell);

                // std::ptr::write_unaligned::<Cell>(self.tail, Cell::new(value));
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_nil() {
            len += 1
        }
        if !self.tail.is_null() {
            // let tail = unsafe { std::ptr::read::<Cell>(self.tail) };
            // len += tail.len();
        }
        len
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        let mut values = Vec::<Value>::new();
        values.push(self.head.clone());
        values
    }
}

#[cfg(test)]
mod cell_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;
    #[test]
    fn test_nil() {
        let cell = Cell::nil();
        assert_equal!(cell.len(), 0);
        assert_equal!(cell.values(), vec![Value::Nil]);
    }

    #[test]
    fn test_new() {
        let cell = Cell::new(Value::from("head"));
        assert_equal!(cell.len(), 1);
        assert_equal!(cell.values(), vec![Value::from("head")]);
    }
    #[test]
    fn test_add() {
        let mut cell = Cell::new(Value::from("head"));
        cell.add_value(Value::from("tail"));
        assert_equal!(cell.len(), 2);
        assert_equal!(cell.values(), vec![Value::from("head"), Value::from("tail")]);
    }
}
//     pub fn head(&self) -> Value<'c> {
//         match self.head.clone() {
//             Value::Symbol(h) => Value::from(h.into_owned()),
//             Value::Nil => Value::Nil,
//             Value::Cell(cell) => {
//                 let cell = cell.as_ref().clone();
//                 match cell.head {
//                     Value::Symbol(h) => Value::from(h.into_owned()),
//                     Value::Nil => Value::Nil,
//                     Value::Cell(cell) => {
//                         let cell = cell.as_ref().clone();
//                         Value::from(cell)
//                     },
//                 }
//             },
//         }
//     }

//     pub fn tail(&self) -> Value<'c> {
//         match self.tail.clone().map(|rc| rc.as_ref().clone()) {
//             Some(cell) => match cell.head.clone() {
//                 Value::Symbol(h) => Value::from(h.into_owned()),
//                 Value::Cell(cell) => {
//                     let cell = cell.as_ref().clone();
//                     Value::from(cell)
//                 },
//                 Value::Nil => Value::Nil,
//             },
//             None => Value::Nil,
//         }
//     }

//     pub fn is_nil(&self) -> bool {
//         self.head.is_nil() && self.tail().is_nil()
//     }

//     pub fn split_string(&self) -> [Option<String>; 2] {
//         let head = self.head.clone();
//         let head: Option<String> = match head {
//             Value::Symbol(head) => Some(format!("{}", head)),
//             Value::Cell(cell) => Some("cell".to_string()),
//             Value::Nil => None,
//         };
//         let tail = None;
//         [head, tail]
//     }
// }
// impl<'c> From<Value<'c>> for Cell<'c> {
//     fn from(head: Value<'c>) -> Cell<'c> {
//         match head {
//             Value::Symbol(head) => Cell {
//                 head: Value::from(head),
//                 tail: None,
//             },
//             Value::Nil => Cell::nil(),
//             Value::Cell(cell) => {
//                 // cell
//                 Cell {
//                     head: Value::from(format!("{}:{}", file!(), line!())),
//                     tail: None,
//                 }
//             }
//         }
//     }
// }

// impl<'c> From<Option<Cell<'c>>> for Cell<'c> {
//     fn from(cell: Option<Cell<'c>>) -> Cell<'c> {
//         match cell {
//             Some(cell) => cell,
//             None => Cell::nil(),
//         }
//     }
// }
// impl std::fmt::Display for Cell<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", {
//             let parts = self.split_string();
//             let expressions = parts
//                 .clone()
//                 .iter()
//                 .filter(|part| part.is_some())
//                 .map(|expression| expression.clone().map(String::from).unwrap())
//                 .collect::<Vec<String>>();
//             let expressions = expressions.join(" ");
//             let mut wrap = expressions.len() > 0;
//             // let mut wrap = parts.iter().all(|part|part.is_some());
//             format!("({})", expressions)
//         })
//     }
// }
// // impl std::fmt::Debug for Cell<'_> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// //         write!(f, "{}", {
// //             let parts = self.split_string();
// //             let expressions = parts
// //                 .clone()
// //                 .iter()
// //                 .filter(|part| part.is_some())
// //                 .map(|expression| expression.clone().map(String::from).unwrap())
// //                 .collect::<Vec<String>>();
// //             let expressions = expressions.join(" . ");
// //             let mut wrap = expressions.len() > 0;
// //             // let mut wrap = parts.iter().all(|part|part.is_some());
// //             if wrap {
// //                 format!("({})", expressions)
// //             } else {
// //                 "nil".to_string()
// //             }
// //         })
// //     }
// // }

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
//         // assert_debug_equal!(Cell::nil(), Cell::from(Value::Nil));
//         // assert_debug_equal!(Cell::nil(), "nil");
//         // assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
//         // assert_display_equal!(Cell::nil(), "()");
//     }
//     #[test]
//     fn from_cell_nil() {
//         assert_equal!(Cell::nil(), Cell::from(Cell::nil()));
//         // assert_debug_equal!(Cell::nil(), Cell::from(Value::Nil));
//         // assert_debug_equal!(Cell::nil(), "nil");
//         assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
//         assert_display_equal!(Cell::nil(), "()");
//     }
//     #[test]
//     fn from_cell_debug_head_and_tail_with_head_symbol_tail_nil() {
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
//         // assert_debug_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head . tail)");
//         assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
//     }
//     #[test]
//     fn from_cell_debug_head_and_tail_with_head_nil_tail_head_symbol() {
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
//         // assert_debug_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head . tail)");
//         assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
//     }
// }
