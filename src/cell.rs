use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{cons, ListValue, Value};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Default)]
pub struct Cell<'c> {
    pub head: Value<'c>,
    pub tail: Option<Rc<Cell<'c>>>,
}
impl<'c> Cell<'_> {
    pub fn nil() -> Cell<'c> {
        Cell::from(Value::Nil)
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_nil() && self.tail.is_none()
    }
}
impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(head: Value<'c>) -> Cell<'c> {
        Cell { head, tail: None }
    }
}
impl<'v> Into<Value<'v>> for Cell<'v> {
    fn into(self) -> Value<'v> {
        Value::Cell(self.into())
    }
}

impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}{})",
            self.head,
            String::new(),
            //self.tail.map(|cell|cell.to_string()).unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod cell_tests {
    use k9::assert_equal;

    use super::*;

    #[test]
    fn from_value_nil() {
        assert_equal!(
            Cell::from(Value::Nil),
            Cell {
                head: Value::Nil,
                tail: None
            }
        );
    }
    #[test]
    fn from_value_symbol() {
        assert_equal!(
            Cell::from(Value::from("symbol")),
            Cell {
                head: Value::from("symbol"),
                tail: None
            }
        );
    }
    #[test]
    fn nil() {
        assert_equal!(Cell::nil(), Cell::from(Value::Nil));
    }
    #[test]
    fn from_cell_nil() {
        assert_equal!(Cell::nil(), Cell::from(Cell::nil()));
    }
}
