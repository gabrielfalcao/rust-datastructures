use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;
use crate::{ListValue, Cell, Value};


pub fn cons<'a, H: Into<Value<'a>>>(head: H, tail: H) -> Cell<'a> {
    let head = head.into();
    let tail = tail.into();

    Cell {
        head: if head.is_nil() { Value::Nil } else { head },
        tail: match tail {
            Value::Nil => None,
            Value::Cell(cell) =>
                if cell.as_ref().is_nil() {
                    None
                } else {
                    Some(cell)
                },
            value => Some(Rc::new(Cell::from(value))),
        },
    }
}
#[cfg(test)]
mod cons_tests {
    use k9::assert_equal;

    use super::*;

    #[test]
    fn cons_nil() {
        assert_equal!(cons(Cell::nil(), Cell::nil()), Cell::nil());
    }
}
