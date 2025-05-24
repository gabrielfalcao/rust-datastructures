use std::rc::Rc;

use crate::{step, Cell, Value};

#[macro_export]
macro_rules! list {
    ($( $item:literal ),* ) => {{
        let mut cell = Cell::nil();
        $(cell.add(&mut Cell::from($item));
        )*
        cell
    }};
}

pub fn cons<'c, H: Into<Value<'c>>>(head: H, tail: &mut Cell<'c>) -> Cell<'c> {
    let mut head = Cell::new(head.into());
    head.add(tail);
    head
}
pub fn cdr<'c>(cell: &Cell<'c>) -> Cell<'c> {
    if let Some(tail) = cell.tail() {
        tail.clone()
    } else {
        Cell::nil()
    }
}
pub fn car<'c>(cell: &Cell<'c>) -> Value<'c> {
    if let Some(head) = cell.head() {
        head
    } else {
        Value::nil()
    }
}
// #[cfg(test)]
// mod cons_tests {
//     use std::rc::Rc;

//     use k9::assert_equal;

//     use crate::*;

//     #[test]
//     fn from_value_nil() {
//         let cell = Cons::from(Value::Nil);
//         assert_equal!(cell, Cons::Empty);
//         assert_equal!(cell, Cons::nil());
//         assert_equal!(cell.len(), 0);
//         assert_debug_equal!(cell, "(nil)");
//         assert_display_equal!(cell, "()");
//     }
//     #[test]
//     fn from_value_symbol() {
//         let cell = Cons::from(Value::from("symbol"));
//         assert_equal!(cell, Cons::Head(Value::from("symbol")));
//         assert_equal!(cell.len(), 1);
//         assert_display_equal!(cell, "symbol");
//         assert_debug_equal!(cell, "('symbol . nil)");
//     }
//     #[test]
//     fn cons_function_simple_head_tail() {
//         let head = Cons::from(Value::from("head"));
//         let tail = Cons::from(Value::from("tail"));
//         let cell = Cons::Cell(Value::from("head"), Rc::new(tail.clone()));
//         assert_equal!(cons(head, tail), cell);
//         assert_equal!(cell.len(), 2);
//         assert_display_equal!(cell, "(head tail)");
//         assert_debug_equal!(cell, "('head . 'tail . nil)");
//     }
//     #[test]
//     fn cons_function_head_tail_with_tail() {
//         let head = Cons::from(Value::from("head"));
//         let tail = Cons::Cell(Value::from("cell"), Rc::new(Cons::from(Value::from("tail"))));
//         let cell = Cons::Cell(
//             Value::from("head"),
//             Rc::new(Cons::Cell(Value::from("cell"), Rc::new(Cons::from(Value::from("tail"))))),
//         );
//         assert_equal!(cell.len(), 3);
//         assert_equal!(cons(head, tail), cell);
//         assert_display_equal!(cell, "(head cell tail)");
//         assert_debug_equal!(cell, "('head . 'cell . 'tail . nil)");
//     }
// }
