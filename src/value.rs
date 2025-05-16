use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::ListValue;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Default)]
pub enum Value<'v> {
    #[default]
    Nil,
    Symbol(Cow<'v, str>),
    Cell(Rc<Cell<'v>>),
}
impl<'v> Value<'_> {
    pub fn nil() -> Value<'v> {
        Value::Nil
    }

    pub fn is_nil(&self) -> bool {
        if let Value::Cell(cell) = self {
            cell.as_ref().is_nil()
        } else if *self == Value::Nil {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod value_tests {
    use k9::assert_equal;

    use super::*;

    #[test]
    fn value_from_static_str() {
        let value = "static-str";
        assert_equal!(Value::from(value).to_string(), "'static-str");
    }
    #[test]
    fn value_from_str() {
        let value = "str".to_string().leak();
        assert_equal!(Value::from(value).to_string(), "'str");
    }
    #[test]
    fn value_from_string() {
        let value = "string".to_string();
        assert_equal!(Value::from(value).to_string(), "'string");
    }
}

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

impl<'v> From<&'v str> for Value<'v> {
    fn from(value: &'v str) -> Value<'v> {
        Value::Symbol(Cow::from(value))
    }
}
impl<'v> From<&'v mut str> for Value<'v> {
    fn from(value: &'v mut str) -> Value<'v> {
        Value::Symbol(Cow::<'v, str>::Borrowed(&*value))
    }
}
impl<'v> From<String> for Value<'v> {
    fn from(value: String) -> Value<'v> {
        Value::Symbol(Cow::from(value))
    }
}
impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::Symbol(h) => format!("'{}", h),
                Value::Cell(c) => c.to_string(),
            }
        )
    }
}
