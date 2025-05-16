use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{Cell, ListValue, Value};

pub fn cons<'a, H: Into<Value<'a>>, C: Into<Cell<'a>>>(head: H, tail: C) -> Cell<'a> {
    let head = head.into();
    let tail = tail.into();
    let mut head = if head.is_nil() { Value::Nil } else { head };
    let mut tail = match tail.head {
        Value::Nil => None,
        Value::Cell(cell) =>
            if cell.as_ref().is_nil() {
                None
            } else {
                Some(cell)
            },
        value => Some(Rc::new(Cell::from(value))),
    };
    // while head.is_nil() && !tail.is_nil() {
    //     tail = tail.as_ref().tail.clone();
    //     head = tail.head;
    // }
    Cell {
        head: head,
        tail: tail,
    }
}
pub fn car<'c>(cell: Cell<'c>) -> Value<'c> {
    cell.head
}
pub fn cdr<'c>(cell: Cell<'c>) -> Value<'c> {
    cell.tail
        .clone()
        .map(|cell| Value::from(cell.as_ref().clone()))
        .unwrap_or_else(|| Value::Nil)
}

#[cfg(test)]
mod cons_tests {
    use k9::assert_equal;

    use crate::*;

    #[test]
    fn cons_nil_from_cell_nil() {
        assert_equal!(cons(Cell::nil(), Cell::nil()), Cell::nil());
    }
    #[test]
    fn cons_nil_from_value_nil() {
        assert_equal!(cons(Value::nil(), Cell::nil()), Cell::nil());
    }
    // #[test]
    // fn cons_nil_from_value_nil_cell_symbol() {
    //     assert_equal!(
    //         cons(Value::nil(), Cell::from(Value::from("symbol"))),
    //         Cell::from(Value::from("symbol"))
    //     );
    // }
}
